//! DevStar MCP Server — stdio transport for AI agents
//!
//! Usage: `devstar-mcp`
//! Connects via JSON-RPC over stdio following the MCP protocol.
//! Shares the same SQLite database as the main Tauri app.

#![allow(clippy::type_complexity)]
#![allow(clippy::map_entry)]

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

// ─── Database path ───

fn get_db_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("com.njne2.devstar")
        .join("devstar.db")
}

// ─── Agent Identity ───

fn get_agent_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".devstar")
        .join("agents")
}

fn get_or_create_agent_id(client_name: &str) -> String {
    let dir = get_agent_dir();
    std::fs::create_dir_all(&dir).ok();
    let path = dir.join(format!("{}.json", client_name));
    if let Ok(content) = std::fs::read_to_string(&path) {
        if let Ok(v) = serde_json::from_str::<Value>(&content) {
            if let Some(id) = v.get("id").and_then(|v| v.as_str()) {
                return id.to_string();
            }
        }
    }
    let id = Uuid::new_v4().to_string();
    let data = json!({
        "id": id,
        "name": client_name,
        "first_seen": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    });
    std::fs::write(&path, serde_json::to_string(&data).unwrap_or_default()).ok();
    id
}

// ─── Project Context (.devstar.json) ───

fn read_project_context(project_dir: &str) -> Result<Value, String> {
    let dir = if project_dir == "." || project_dir.is_empty() {
        std::env::current_dir().map_err(|e| e.to_string())?
    } else {
        PathBuf::from(project_dir)
    };
    let config_path = dir.join(".devstar.json");
    let content = std::fs::read_to_string(&config_path).map_err(|_e| {

        format!(
            "No .devstar.json found in {}. Run `create_project` first.",
            dir.display()
        )
    })?;
    let config: Value =
        serde_json::from_str(&content).map_err(|e| format!("Invalid .devstar.json: {}", e))?;
    let project_id = config
        .get("project_id")
        .and_then(|v| v.as_i64())
        .ok_or("Missing project_id in .devstar.json")?;
    Ok(json!({ "project_id": project_id, "project_name": config.get("project_name").and_then(|v| v.as_str()).unwrap_or(""), "config": config }))
}

fn write_project_context(
    project_dir: &str,
    project_id: i64,
    project_uuid: &str,
    project_name: &str,
    template_id: i64,
) -> Result<(), String> {
    let dir = if project_dir.is_empty() {
        std::env::current_dir().map_err(|e| e.to_string())?
    } else {
        PathBuf::from(project_dir)
    };
    let config_path = dir.join(".devstar.json");
    let config = json!({
        "project_id": project_id,
        "project_uuid": project_uuid,
        "project_name": project_name,
        "template_id": template_id,
        "created_at": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    });
    std::fs::write(
        &config_path,
        serde_json::to_string_pretty(&config).unwrap_or_default(),
    )
    .map_err(|e| format!("Failed to write .devstar.json: {}", e))?;
    Ok(())
}

// ─── Access Control ───

fn get_project_id_from_uuid(conn: &Connection, uuid: &str) -> Option<i64> {
    conn.query_row(
        "SELECT id FROM projects WHERE uuid = ?1 LIMIT 1",
        [uuid],
        |row| row.get(0),
    ).ok()
}

/// Resolve project_id from params or directory search.
fn resolve_project_id(params: &Value, conn: &Connection) -> Option<i64> {
    if let Some(pid) = params.get("project_id").and_then(|v| v.as_i64()) {
        return Some(pid);
    }
    // BUG-01: also resolve project_uuid
    if let Some(uuid) = params.get("project_uuid").and_then(|v| v.as_str()) {
        if let Some(pid) = get_project_id_from_uuid(conn, uuid) {
            return Some(pid);
        }
    }
    if let Ok(ctx) = read_project_context(".") {
        return ctx.get("project_id").and_then(|v| v.as_i64());
    }
    None
}

/// Get project_id, returning a helpful error if none found.
fn require_project_id(params: &Value, conn: &Connection) -> Result<i64, String> {
    if let Some(pid) = get_scoped_project() {
        return Ok(pid);
    }
    if let Some(pid) = resolve_project_id(params, conn) {
        return Ok(pid);
    }
    Err("No project found. Run from a project directory or pass project_id.".to_string())
}

// ─── MCP Protocol Types ───

#[derive(Deserialize)]
struct JsonRpcRequest {
    #[serde(rename = "jsonrpc")]
    _jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Serialize)]
struct JsonRpcError {
    code: i64,
    message: String,
}

// ─── Session State ───

static ACTIVE_PROJECT_ID: OnceLock<Mutex<Option<i64>>> = OnceLock::new();

fn get_active_project() -> Option<i64> {
    *ACTIVE_PROJECT_ID
        .get_or_init(|| Mutex::new(None))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn set_active_project(pid: i64) {
    *ACTIVE_PROJECT_ID
        .get_or_init(|| Mutex::new(None))
        .lock()
        .unwrap_or_else(|e| e.into_inner()) = Some(pid);
}

// ─── Tool Definitions ───

fn tool_definitions() -> Value {
    json!([
        {
            "name": "initialize",
            "description": "Initialize the agent session by setting the project scope. Call this tool with a project_uuid (found in .devstar.json) immediately upon startup to lock the session to a specific project. This enables all other tools to operate without needing project-specific arguments. The project_uuid must be at the top level of the params object.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_uuid": { "type": "string", "description": "The unique ID for the project from its .devstar.json file." }
                },
                "required": ["project_uuid"]
            }
        },
        {
            "name": "get_project_context",
            "description": "Zero-config project discovery. Reads .devstar.json from the current working directory (or specified project_dir) and returns a compact overview including: project name, total progress (checked/total), percentage, and the full active sprint with all sections and their items (title + checked status). Use this FIRST when you enter a new project directory to understand what you're working on. If no .devstar.json exists, this tool returns an error — use create_project to start a new project.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_dir": { "type": "string", "description": "Directory containing .devstar.json. Leave empty to use current working directory." }
                }
            }
        },
        {
            "name": "dashboard",
            "description": "Compact overview of ALL projects in the database. Returns each project's id, name, checked count, total count, and active sprint name. No arguments needed. Use this to survey the workspace and decide which project to work on.",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "create_project",
            "description": "Create a new project from a template and write .devstar.json to the specified directory for agent scoping. Workflow: 1) Call list_templates to find available templates and their IDs. 2) Call create_project with name and template_id. 3) The project_dir (default: current directory) gets a .devstar.json file so future tools know which project to work on. This tool copies the template's full sprint/section/item hierarchy.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Project name (e.g., 'My API Service')" },
                    "template_id": { "type": "integer", "description": "Template ID to create from. Call list_templates first to find the right ID." },
                    "description": { "type": "string", "description": "Optional project description." },
                    "color": { "type": "string", "description": "Optional hex color for the project (e.g., '#6366f1')." },
                    "project_dir": { "type": "string", "description": "Directory to write .devstar.json. Leave empty to use current directory." }
                },
                "required": ["name", "template_id"]
            }
        },
        {
            "name": "get_active_sprint_detail",
            "description": "Get the current active sprint with all sections and their items (id, title, checked). Each section shows its id, name, checked count, and total count. Use this to see exactly what tasks need to be done right now. If no .devstar.json exists in the working directory, you must provide project_id. Returns null if no sprint is active.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "integer", "description": "Project ID. Optional if .devstar.json exists in working directory." }
                }
            }
        },
        {
            "name": "add_task",
            "description": "Add a new task to the active sprint. Requires only a title. The tool automatically finds the active sprint and adds the task to the first section. If you specify section_name, it finds a matching section or creates a new one with that name. No section_id or sprint_id needed — the tool figures it out from the scoped project. Use this to add new work items during planning or when you discover new requirements.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "title": { "type": "string", "description": "Task title (e.g., 'Add rate limiting to API endpoints')" },
                    "description": { "type": "string", "description": "Optional longer description for context." },
                    "section_name": { "type": "string", "description": "Section to add to (e.g., 'Security'). If omitted, uses the first section. If the section doesn't exist, it's created." }
                },
                "required": ["title"]
            }
        },
        {
            "name": "check_task",
            "description": "Check off (mark as done) a task by its title. Uses partial, case-insensitive matching — you only need a unique substring. For example, title='rate limit' would match 'Add rate limiting to API endpoints'. If multiple tasks match, the first one (by sort order) is checked. After checking, the tool automatically checks if all tasks in the sprint are done and advances to the next sprint if so. Use this to complete tasks as you finish them.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "title": { "type": "string", "description": "Task title or unique substring to match against (case-insensitive). e.g., 'rate limit' to find 'Add rate limiting to API endpoints'" }
                },
                "required": ["title"]
            }
        },
        {
            "name": "update_item",
            "description": "Update a specific item by its numeric ID. Can check/uncheck it (set checked to true/false) or add notes. This is the low-level tool — prefer check_task and uncheck_task for title-based operations which don't require knowing the ID. Use this when you have a specific item_id from another tool's response (like get_active_sprint_detail or get_tasks).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "item_id": { "type": "integer", "description": "Numeric item ID from a tool response (e.g., from get_active_sprint_detail or get_tasks)." },
                    "checked": { "type": "boolean", "description": "Set to true to mark done, false to mark undone." },
                    "notes": { "type": "string", "description": "Optional notes to attach to the item." }
                },
                "required": ["item_id"]
            }
        },
        {
            "name": "add_item",
            "description": "Add a task to a specific section by its numeric ID. This is the low-level version of add_task (which auto-finds sections). Use only when you have a section_id from get_active_sprint_detail or get_sprint responses. Most agents should prefer add_task instead.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "section_id": { "type": "integer", "description": "Numeric section ID (from get_active_sprint_detail or get_sprint responses)." },
                    "title": { "type": "string", "description": "Task title." },
                    "description": { "type": "string", "description": "Optional task description." }
                },
                "required": ["section_id", "title"]
            }
        },
        {
            "name": "complete_sprint",
            "description": "Mark ALL tasks in the active sprint as done, mark the sprint as done, and activate the next pending sprint. No arguments needed if .devstar.json exists. Use this when you want to fast-forward past a sprint without checking individual tasks.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "integer", "description": "Project ID. Optional if .devstar.json exists in working directory." }
                }
            }
        },
        {
            "name": "get_progress",
            "description": "Get overall project completion stats: checked count, total count, and percentage. No arguments needed if .devstar.json exists. Use this to report progress to users or check how far along the project is.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "integer", "description": "Project ID. Optional if .devstar.json exists in working directory." }
                }
            }
        },
        {
            "name": "log_error",
            "description": "Log an error or issue as an unchecked task in the active sprint. Automatically creates an 'Agent Errors' section if it doesn't exist. The error is tagged with your agent ID for attribution. Use this when you encounter problems, hit blockers, or need to flag issues for the human user to review.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "error": { "type": "string", "description": "Description of the error or issue (e.g., 'Failed to compile: missing dependency libfoo')" }
                },
                "required": ["error"]
            }
        },
        {
            "name": "get_project_sprints",
            "description": "List ALL sprints in the project with their status (pending/active/done), progress (checked/total tasks), and section counts. Returns sprints ordered by their sort_order (1-based position). Use this to understand the full project plan, see which sprints are done, which is current, and what's coming up. This is the best tool for getting the big picture of where a project stands.",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "get_sprint",
            "description": "Get detailed information about any specific sprint — its sections, tasks, and their checked status. Find a sprint by its 1-based number (number=1 for the first sprint) or by name (partial match, case-insensitive). Use this to inspect past sprints (what was done), the current sprint (what's being worked on), or future sprints (what's planned). Example: number=3 gets the third sprint, or name='testing' finds any sprint with 'testing' in its name.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "number": { "type": "integer", "description": "Sprint number (1-based). Sprint 1 is the first sprint. Mutually exclusive with name." },
                    "name": { "type": "string", "description": "Sprint name for partial, case-insensitive matching. e.g., 'test' finds 'Testing & QA'. Mutually exclusive with number." }
                }
            }
        },
        {
            "name": "get_tasks",
            "description": "List tasks in the active sprint, optionally filtered by their status. Returns tasks with their id, title, checked status, and section name. Use status='pending' to see what's left to do, status='done' to see what's completed, or status='all' to see everything. This is the best tool for daily standup — call it with status='pending' to know what to work on next.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "status": { "type": "string", "enum": ["pending", "done", "all"], "description": "Filter tasks: 'pending' = unchecked tasks still to do, 'done' = completed tasks, 'all' = everything. Default is 'pending'." }
                }
            }
        },
        {
            "name": "uncheck_task",
            "description": "Uncheck (mark as not done) a task by its title. Uses partial, case-insensitive matching — only needs a unique substring. Finds the first matching CHECKED task and unchecks it. Use this to undo accidental check-offs or to re-open a task that needs more work. This is the reverse of check_task.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "title": { "type": "string", "description": "Task title or unique substring to match against a CHECKED task (case-insensitive)." }
                },
                "required": ["title"]
            }
        },
        {
            "name": "add_section",
            "description": "Add a new section (category) to the active sprint. Use this to organize tasks when the existing sections don't fit the work you're doing. The section is added at the end of the sprint. After creating a section, use add_task with section_name to add tasks to it.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Section name (e.g., 'Deployment', 'Performance', 'Bug Fixes')" },
                    "description": { "type": "string", "description": "Optional section description." }
                },
                "required": ["name"]
            }
        },
        {
            "name": "search_tasks",
            "description": "Search ALL tasks across ALL sprints in the project by keyword. Returns matching tasks with their id, title, checked status, section name, sprint name, and sprint status. Use this when you need to find tasks related to a specific topic (e.g., 'security', 'database', 'auth') regardless of which sprint or section they're in. The search is case-insensitive partial match.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search keyword (case-insensitive partial match against task titles). e.g., 'auth' to find all authentication-related tasks." }
                },
                "required": ["query"]
            }
        },
        {
            "name": "list_templates",
            "description": "List all available project templates with their id, name, and sprint count. This is a shared resource — accessible from any directory without .devstar.json. Use this first before create_project to find the right template_id for your project type.",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "get_template",
            "description": "Get a template's FULL structure: every sprint, every section inside each sprint, and every checklist item inside each section. Use this BEFORE creating a project to fully understand what the template provides, so you can choose the right template and know exactly which sprints and sections already exist — avoiding duplication when you add project-specific tasks later.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "template_id": { "type": "integer", "description": "Template ID from list_templates response." }
                },
                "required": ["template_id"]
            }
        },
        {
            "name": "get_full_project_plan",
            "description": "Get the COMPLETE project plan: every sprint (pending/active/done) with every section and every task item inside each section, plus checked status. Use this at the start of a planning or implementation session to understand the full current state of the project before making any changes. This is the single best tool to call after initialize — it gives you the entire tree so you know exactly where to add tasks, which sections already exist in which sprints, and what is already done.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "integer", "description": "Project ID. Optional if session is scoped via initialize or .devstar.json exists." }
                }
            }
        },
        {
            "name": "list_shared_sections",
            "description": "List all reusable section templates with their id, name, and item count. Sections are pre-built checklists that can be added to any sprint. Use this to see what checklist blocks are available when planning a project.",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "list_shared_sprints",
            "description": "List all reusable sprint templates with their id, name, and section count. These are pre-built sprint templates that can be added to any project. Use this when adding sprints to a project.",
            "inputSchema": { "type": "object", "properties": {} }
        }
    ])
}

fn get_scoped_project() -> Option<i64> {
    get_active_project()
}

fn like_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

// ─── Tool Handlers ───

fn handle_list_templates(conn: &Connection) -> Result<Value, String> {
    let mut stmt = conn
        .prepare(
            "SELECT t.id, t.name, t.description,
                    (SELECT count(*) FROM template_sprints ts WHERE ts.template_id = t.id) as sc
             FROM templates t ORDER BY t.name",
        )
        .map_err(|e| e.to_string())?;

    let templates: Vec<Value> = stmt
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "sprints": row.get::<_, i64>(3)?,
            }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "templates": templates }))
}

fn handle_get_template(conn: &Connection, params: &Value) -> Result<Value, String> {
    let template_id = params
        .get("template_id")
        .and_then(|v| v.as_i64())
        .ok_or("template_id required")?;

    let mut t_stmt = conn
        .prepare("SELECT id, name, description FROM templates WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    let template: Value = t_stmt
        .query_row([template_id], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "description": row.get::<_, String>(2)?,
            }))
        })
        .map_err(|e| e.to_string())?;

    // Load every sprint with its full section+item tree
    let mut s_stmt = conn
        .prepare(
            "SELECT ts.id, ts.name, ts.description, ts.sort_order
             FROM template_sprints ts WHERE ts.template_id = ?1 ORDER BY ts.sort_order",
        )
        .map_err(|e| e.to_string())?;

    let sprint_rows: Vec<(i64, String, String, i64)> = s_stmt
        .query_map([template_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let mut sprints = Vec::new();
    for (ts_id, ts_name, ts_desc, ts_order) in sprint_rows {
        // Load sections for this sprint
        let mut sec_stmt = conn
            .prepare(
                "SELECT ss.id, ss.name, ss.description, tss.sort_order
                 FROM template_sprint_sections tss
                 JOIN shared_sections ss ON tss.section_id = ss.id
                 WHERE tss.sprint_id = ?1 ORDER BY tss.sort_order",
            )
            .map_err(|e| e.to_string())?;

        let sec_rows: Vec<(i64, String, String, i64)> = sec_stmt
            .query_map([ts_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        let mut sections = Vec::new();
        for (sec_id, sec_name, sec_desc, _sec_order) in sec_rows {
            let mut item_stmt = conn
                .prepare(
                    "SELECT title FROM shared_section_items WHERE section_id = ?1 ORDER BY sort_order",
                )
                .map_err(|e| e.to_string())?;

            let items: Vec<String> = item_stmt
                .query_map([sec_id], |r| r.get(0))
                .map_err(|e| e.to_string())?
                .filter_map(|r| r.ok())
                .collect();

            sections.push(json!({
                "name": sec_name,
                "description": sec_desc,
                "items": items,
            }));
        }

        sprints.push(json!({
            "number": ts_order + 1,
            "name": ts_name,
            "description": ts_desc,
            "sections": sections,
        }));
    }

    Ok(json!({ "template": template, "sprints": sprints }))
}

/// Full project plan: every sprint with every section and every item.
fn handle_get_full_project_plan(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;

    let project_name: String = conn
        .query_row("SELECT name FROM projects WHERE id = ?1", [project_id], |r| r.get(0))
        .unwrap_or_default();

    let (total_checked, total_items): (i64, i64) = conn
        .query_row(
            "SELECT \
             (SELECT count(*) FROM project_items pi JOIN project_sprint_sections pss ON pi.section_id = pss.id JOIN project_sprints ps ON pss.sprint_id = ps.id WHERE ps.project_id = ?1 AND pi.checked = 1), \
             (SELECT count(*) FROM project_items pi JOIN project_sprint_sections pss ON pi.section_id = pss.id JOIN project_sprints ps ON pss.sprint_id = ps.id WHERE ps.project_id = ?1)",
            [project_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap_or((0, 0));

    let mut s_stmt = conn
        .prepare(
            "SELECT id, name, description, status, sort_order FROM project_sprints
             WHERE project_id = ?1 ORDER BY sort_order",
        )
        .map_err(|e| e.to_string())?;

    let sprint_rows: Vec<(i64, String, String, String, i64)> = s_stmt
        .query_map([project_id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let mut sprints = Vec::new();
    for (sprint_id, sprint_name, sprint_desc, sprint_status, sort_order) in sprint_rows {
        let mut sec_stmt = conn
            .prepare(
                "SELECT pss.id, pss.name,
                        (SELECT count(*) FROM project_items pi WHERE pi.section_id = pss.id),
                        (SELECT count(*) FROM project_items pi WHERE pi.section_id = pss.id AND pi.checked = 1)
                 FROM project_sprint_sections pss
                 WHERE pss.sprint_id = ?1 ORDER BY pss.sort_order",
            )
            .map_err(|e| e.to_string())?;

        let sec_rows: Vec<(i64, String, i64, i64)> = sec_stmt
            .query_map([sprint_id], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        let mut sections = Vec::new();
        for (sec_id, sec_name, item_count, checked_count) in &sec_rows {
            let mut item_stmt = conn
                .prepare(
                    "SELECT id, title, checked FROM project_items WHERE section_id = ?1 ORDER BY sort_order",
                )
                .map_err(|e| e.to_string())?;

            let items: Vec<Value> = item_stmt
                .query_map([sec_id], |r| {
                    Ok(json!({
                        "id": r.get::<_, i64>(0)?,
                        "title": r.get::<_, String>(1)?,
                        "checked": r.get::<_, i64>(2)? != 0,
                    }))
                })
                .map_err(|e| e.to_string())?
                .filter_map(|r| r.ok())
                .collect();

            sections.push(json!({
                "id": sec_id,
                "name": sec_name,
                "checked": checked_count,
                "total": item_count,
                "items": items,
            }));
        }

        let sprint_checked: i64 = sec_rows.iter().map(|(_, _, _, c)| c).sum();
        let sprint_total: i64 = sec_rows.iter().map(|(_, _, t, _)| t).sum();

        sprints.push(json!({
            "id": sprint_id,
            "number": sort_order + 1,
            "name": sprint_name,
            "description": sprint_desc,
            "status": sprint_status,
            "checked": sprint_checked,
            "total": sprint_total,
            "sections": sections,
        }));
    }

    Ok(json!({
        "project_id": project_id,
        "project_name": project_name,
        "total_checked": total_checked,
        "total_items": total_items,
        "sprints": sprints,
    }))
}

fn handle_list_shared_sections(conn: &Connection) -> Result<Value, String> {
    let mut stmt = conn
        .prepare(
            "SELECT ss.id, ss.name, ss.description,
                    (SELECT count(*) FROM shared_section_items ssi WHERE ssi.section_id = ss.id)
             FROM shared_sections ss ORDER BY ss.name",
        )
        .map_err(|e| e.to_string())?;

    let sections: Vec<Value> = stmt
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "items": row.get::<_, i64>(3)?,
            }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "sections": sections }))
}

fn handle_list_shared_sprints(conn: &Connection) -> Result<Value, String> {
    let mut stmt = conn
        .prepare(
            "SELECT ss.id, ss.name, ss.description,
                    (SELECT count(*) FROM shared_sprint_sections sss WHERE sss.sprint_id = ss.id)
             FROM shared_sprints ss ORDER BY ss.sort_order",
        )
        .map_err(|e| e.to_string())?;

    let sprints: Vec<Value> = stmt
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "sections": row.get::<_, i64>(3)?,
            }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "sprints": sprints }))
}

/// Compact dashboard: all projects with progress and active sprint.
fn handle_dashboard(conn: &Connection) -> Result<Value, String> {
    let mut p_stmt = conn
        .prepare("SELECT id, name FROM projects ORDER BY name")
        .map_err(|e| e.to_string())?;

    let project_ids: Vec<i64> = p_stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let mut projects = Vec::new();
    for pid in project_ids {
        let name: String = conn
            .query_row("SELECT name FROM projects WHERE id = ?", [pid], |r| r.get(0))
            .unwrap_or_default();

        let (checked, total): (i64, i64) = conn
            .query_row(
                "SELECT \
                 (SELECT count(*) FROM project_items pi \
                  JOIN project_sprint_sections pss ON pi.section_id = pss.id \
                  JOIN project_sprints ps ON pss.sprint_id = ps.id \
                  WHERE ps.project_id = ?1 AND pi.checked = 1), \
                 (SELECT count(*) FROM project_items pi \
                  JOIN project_sprint_sections pss ON pi.section_id = pss.id \
                  JOIN project_sprints ps ON pss.sprint_id = ps.id \
                  WHERE ps.project_id = ?1)",
                [pid],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap_or((0, 0));

        let active: Option<(String, i64)> = conn
            .query_row(
                "SELECT name, sort_order FROM project_sprints WHERE project_id = ?1 AND status = 'active' LIMIT 1",
                [pid],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .ok();

        projects.push(json!({
            "id": pid,
            "name": name,
            "checked": checked,
            "total": total,
            "active_sprint": active.as_ref().map(|(n, o)| format!("Sprint {}: {}", o + 1, n)),
        }));
    }

    Ok(json!({ "projects": projects }))
}

/// Full project context: reads .devstar.json and returns compact project + active sprint.
fn handle_get_project_context(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_dir = params
        .get("project_dir")
        .and_then(|v| v.as_str())
        .unwrap_or(".");
    let ctx = read_project_context(project_dir)?;
    let project_id = ctx
        .get("project_id")
        .and_then(|v| v.as_i64())
        .unwrap();

    // Project overview
    let (checked, total): (i64, i64) = conn
        .query_row(
            "SELECT \
             (SELECT count(*) FROM project_items pi JOIN project_sprint_sections pss ON pi.section_id = pss.id JOIN project_sprints ps ON pss.sprint_id = ps.id WHERE ps.project_id = ?1 AND pi.checked = 1), \
             (SELECT count(*) FROM project_items pi JOIN project_sprint_sections pss ON pi.section_id = pss.id JOIN project_sprints ps ON pss.sprint_id = ps.id WHERE ps.project_id = ?1)",
            [project_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap_or((0, 0));

    // Active sprint with sections and items
    let active_sprint: Option<(i64, String, i64)> = conn
        .query_row(
            "SELECT id, name, sort_order FROM project_sprints WHERE project_id = ?1 AND status = 'active' LIMIT 1",
            [project_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .ok();

    let mut sections = Vec::new();
    if let Some((sprint_id, sprint_name, sort_order)) = active_sprint {
        let mut sec_stmt = conn
            .prepare(
                "SELECT pss.id, pss.name,
                        (SELECT count(*) FROM project_items pi WHERE pi.section_id = pss.id),
                        (SELECT count(*) FROM project_items pi WHERE pi.section_id = pss.id AND pi.checked = 1)
                 FROM project_sprint_sections pss
                 WHERE pss.sprint_id = ?1 ORDER BY pss.sort_order",
            )
            .map_err(|e| e.to_string())?;

        let sec_rows: Vec<(i64, String, i64, i64)> = sec_stmt
            .query_map([sprint_id], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        for (sec_id, sec_name, item_count, checked_count) in &sec_rows {
            let mut item_stmt = conn
                .prepare(
                    "SELECT id, title, checked FROM project_items WHERE section_id = ?1 ORDER BY sort_order",
                )
                .map_err(|e| e.to_string())?;

            let items: Vec<Value> = item_stmt
                .query_map([sec_id], |r| {
                    Ok(json!({
                        "id": r.get::<_, i64>(0)?,
                        "title": r.get::<_, String>(1)?,
                        "checked": r.get::<_, i64>(2)? != 0,
                    }))
                })
                .map_err(|e| e.to_string())?
                .filter_map(|r| r.ok())
                .collect();

            sections.push(json!({
                "id": sec_id,
                "name": sec_name,
                "checked": checked_count,
                "total": item_count,
                "items": items,
            }));
        }

        Ok(json!({
            "project_id": project_id,
            "project_name": ctx.get("project_name").and_then(|v| v.as_str()).unwrap_or(""),
            "checked": checked,
            "total": total,
            "active_sprint": {
                "id": sprint_id,
                "name": sprint_name,
                "sort_order": sort_order,
                "sections": sections,
            },
            "agent_id": get_global_agent_id(),
        }))
    } else {
        Ok(json!({
            "project_id": project_id,
            "project_name": ctx.get("project_name").and_then(|v| v.as_str()).unwrap_or(""),
            "checked": checked,
            "total": total,
            "active_sprint": null,
        }))
    }
}

fn handle_create_project(conn: &Connection, params: &Value) -> Result<Value, String> {
    let name = params
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or("name required")?;
    let template_id = params
        .get("template_id")
        .and_then(|v| v.as_i64())
        .ok_or("template_id required")?;
    let description = params
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let color = params
        .get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#6366f1");
    let project_dir = params
        .get("project_dir")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Verify template exists (BUG-10)
    let template_exists: bool = conn
        .query_row(
            "SELECT count(*) FROM templates WHERE id = ?1",
            [template_id],
            |r| r.get::<_, i64>(0),
        )
        .unwrap_or(0) > 0;

    if !template_exists {
        return Err(format!("Template with id {} not found. Call list_templates first.", template_id));
    }

    // Generate a unique UUID for the project
    let project_uuid = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO projects (uuid, name, description, template_id, color) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&project_uuid, name, description, template_id, color),
    )
    .map_err(|e| e.to_string())?;

    let project_id = conn.last_insert_rowid();

    // Copy template sprints to project
    let mut t_sprints = conn
        .prepare(
            "SELECT id, name, description, sort_order FROM template_sprints WHERE template_id = ?1 ORDER BY sort_order"
        )
        .map_err(|e| e.to_string())?;

    let template_sprints: Vec<(i64, String, String, i64)> = t_sprints
        .query_map([template_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    for (ts_id, ts_name, ts_desc, ts_order) in template_sprints {
        conn.execute(
            "INSERT INTO project_sprints (project_id, name, description, status, sort_order, is_custom) VALUES (?1, ?2, ?3, ?4, ?5, 0)",
            (project_id, &ts_name, &ts_desc, if ts_order == 0 { "active" } else { "pending" }, ts_order),
        ).map_err(|e| e.to_string())?;
        let ps_id = conn.last_insert_rowid();

        // Copy sections
        let mut t_sections = conn
            .prepare(
                "SELECT tss.section_id, tss.sort_order FROM template_sprint_sections tss WHERE tss.sprint_id = ?1 ORDER BY tss.sort_order"
            )
            .map_err(|e| e.to_string())?;

        let section_rows: Vec<(i64, i64)> = t_sections
            .query_map([ts_id], |row| Ok((row.get(0)?, row.get(1)?)))
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        for (sec_id, sec_order) in section_rows {
            let sec_name: String = conn
                .query_row(
                    "SELECT name FROM shared_sections WHERE id = ?1",
                    [sec_id],
                    |r| r.get(0),
                )
                .unwrap_or_default();
            let sec_desc: String = conn
                .query_row(
                    "SELECT description FROM shared_sections WHERE id = ?1",
                    [sec_id],
                    |r| r.get(0),
                )
                .unwrap_or_default();

            conn.execute(
                "INSERT INTO project_sprint_sections (sprint_id, name, description, sort_order, is_custom, linked_from_section_id) VALUES (?1, ?2, ?3, ?4, 0, ?5)",
                (ps_id, &sec_name, &sec_desc, sec_order, sec_id),
            ).map_err(|e| e.to_string())?;
            let pss_id = conn.last_insert_rowid();

            // Copy items
            let mut items = conn
                .prepare(
                    "SELECT title, description, sort_order FROM shared_section_items WHERE section_id = ?1 ORDER BY sort_order"
                )
                .map_err(|e| e.to_string())?;

            let item_rows: Vec<(String, String, i64)> = items
                .query_map([sec_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
                .map_err(|e| e.to_string())?
                .filter_map(|r| r.ok())
                .collect();

            for (title, desc, order) in item_rows {
                conn.execute(
                    "INSERT INTO project_items (section_id, title, description, checked, notes, sort_order, is_custom) VALUES (?1, ?2, ?3, 0, '', ?4, 0)",
                    (pss_id, &title, &desc, order),
                ).map_err(|e| e.to_string())?;
            }
        }
    }

    // Write .devstar.json
    let dir = if !project_dir.is_empty() {
        project_dir.to_string()
    } else {
        ".".to_string()
    };
    write_project_context(&dir, project_id, &project_uuid, name, template_id)?;

    Ok(json!({ "project_id": project_id, "project_uuid": project_uuid, "name": name }))
}

/// Get active sprint detail for scoped project (no project_id needed if .devstar.json exists).
fn handle_get_active_sprint_detail(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;

    let active_sprint: Option<(i64, String, i64)> = conn
        .query_row(
            "SELECT id, name, sort_order FROM project_sprints WHERE project_id = ?1 AND status = 'active' LIMIT 1",
            [project_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .ok();

    if active_sprint.is_none() {
        return Ok(json!({ "active_sprint": null }));
    }

    let (sprint_id, sprint_name, sort_order) = active_sprint.unwrap();

    let mut sec_stmt = conn
        .prepare(
            "SELECT pss.id, pss.name,
                    (SELECT count(*) FROM project_items pi WHERE pi.section_id = pss.id),
                    (SELECT count(*) FROM project_items pi WHERE pi.section_id = pss.id AND pi.checked = 1)
             FROM project_sprint_sections pss
             WHERE pss.sprint_id = ?1 ORDER BY pss.sort_order",
        )
        .map_err(|e| e.to_string())?;

    let sec_rows: Vec<(i64, String, i64, i64)> = sec_stmt
        .query_map([sprint_id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let mut sections = Vec::new();
    for (sec_id, sec_name, item_count, checked_count) in &sec_rows {
        let mut item_stmt = conn
            .prepare(
                "SELECT id, title, checked FROM project_items WHERE section_id = ?1 ORDER BY sort_order",
            )
            .map_err(|e| e.to_string())?;

        let items: Vec<Value> = item_stmt
            .query_map([sec_id], |r| {
                Ok(json!({
                    "id": r.get::<_, i64>(0)?,
                    "title": r.get::<_, String>(1)?,
                    "checked": r.get::<_, i64>(2)? != 0,
                }))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        sections.push(json!({
            "id": sec_id,
            "name": sec_name,
            "checked": checked_count,
            "total": item_count,
            "items": items,
        }));
    }

    Ok(json!({
        "project_id": project_id,
        "active_sprint": {
            "id": sprint_id,
            "name": sprint_name,
            "sort_order": sort_order,
            "sections": sections,
        }
    }))
}

/// Add a task to the active sprint. Agent only needs title and optionally section name.
fn handle_add_task(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;
    let title = params
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or("title required")?;
    let description = params
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let section_name = params.get("section_name").and_then(|v| v.as_str());

    // Find active sprint
    let sprint_id: i64 = conn
        .query_row(
            "SELECT id FROM project_sprints WHERE project_id = ?1 AND status = 'active' LIMIT 1",
            [project_id],
            |row| row.get(0),
        )
        .map_err(|_| "No active sprint found".to_string())?;

    // Find or create section
    let section_id: i64 = if let Some(name) = section_name {
        // Try to find existing section — exact match, case-insensitive
        let existing: Option<i64> = conn.query_row(
                "SELECT id FROM project_sprint_sections WHERE sprint_id = ?1 AND lower(name) = lower(?2) LIMIT 1",
                rusqlite::params![sprint_id, name],
                |row| row.get(0),
            )
            .ok();
        if let Some(id) = existing {
            id
        } else {
            // Create new section
            let max_order: i64 = conn
                .query_row(
                    "SELECT COALESCE(MAX(sort_order), -1) FROM project_sprint_sections WHERE sprint_id = ?1",
                    [sprint_id],
                    |row| row.get(0),
                )
                .map_err(|e| e.to_string())?;
            conn.execute(
                "INSERT INTO project_sprint_sections (sprint_id, name, description, sort_order, is_custom, linked_from_section_id) VALUES (?1, ?2, ?3, ?4, 1, NULL)",
                (sprint_id, name, "", max_order + 1),
            ).map_err(|e| e.to_string())?;
            conn.last_insert_rowid()
        }
    } else {
        // Use first section
        conn.query_row(
            "SELECT id FROM project_sprint_sections WHERE sprint_id = ?1 ORDER BY sort_order LIMIT 1",
            [sprint_id],
            |row| row.get(0),
        )
        .map_err(|_| "No sections in active sprint".to_string())?
    };

    // Add item
    let max_order: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM project_items WHERE section_id = ?1",
            [section_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO project_items (section_id, title, description, checked, notes, sort_order, is_custom) VALUES (?1, ?2, ?3, 0, '', ?4, 1)",
        (section_id, title, description, max_order + 1),
    ).map_err(|e| e.to_string())?;

    let item_id = conn.last_insert_rowid();
    Ok(json!({ "ok": true, "item_id": item_id, "section_id": section_id }))
}

/// Check off a task by title (partial, case-insensitive match).
fn handle_check_task(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;
    let title = params
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or("title required")?;

    // Find the item in the active sprint
    let escaped = like_escape(title);
    let pattern = format!("%{}%", escaped);
    let item_id: Result<i64, _> = conn.query_row(
        "SELECT pi.id FROM project_items pi
         JOIN project_sprint_sections pss ON pi.section_id = pss.id
         JOIN project_sprints ps ON pss.sprint_id = ps.id
         WHERE ps.project_id = ?1 AND ps.status = 'active'
           AND LOWER(pi.title) LIKE LOWER(?2) ESCAPE '\\'
           AND pi.checked = 0
         ORDER BY pi.sort_order LIMIT 1",
        rusqlite::params![project_id, pattern],
        |row| row.get(0),
    );

    let item_id = item_id.map_err(|_| format!("No unchecked task matching '{}' found in active sprint", title))?;

    // Check it off
    conn.execute(
        "UPDATE project_items SET checked = 1 WHERE id = ?1",
        [item_id],
    )
    .map_err(|e| e.to_string())?;

    // Auto-advance sprint if all done
    auto_advance_sprint(conn, item_id)?;

    Ok(json!({ "ok": true, "item_id": item_id, "title": title }))
}

fn handle_update_item(conn: &Connection, params: &Value) -> Result<Value, String> {
    let item_id = params
        .get("item_id")
        .and_then(|v| v.as_i64())
        .ok_or("item_id required")?;

    // Access control: verify item belongs to scoped project
    if let Some(scoped_id) = get_scoped_project() {
        let project_ok: Result<i64, _> = conn.query_row(
            "SELECT count(*) FROM project_items pi
             JOIN project_sprint_sections pss ON pi.section_id = pss.id
             JOIN project_sprints ps ON pss.sprint_id = ps.id
             WHERE pi.id = ?1 AND ps.project_id = ?2",
            [item_id, scoped_id],
            |row| row.get(0),
        );
        if let Ok(count) = project_ok {
            if count == 0 {
                return Err(format!(
                    "Access denied: item {} is not in your scoped project.",
                    item_id
                ));
            }
        }
    }

    if let Some(checked) = params.get("checked").and_then(|v| v.as_bool()) {
        conn.execute(
            "UPDATE project_items SET checked = ?1 WHERE id = ?2",
            (if checked { 1i64 } else { 0i64 }, item_id),
        )
        .map_err(|e| e.to_string())?;

        if checked {
            auto_advance_sprint(conn, item_id)?;
        }
    }
    if let Some(notes) = params.get("notes").and_then(|v| v.as_str()) {
        conn.execute(
            "UPDATE project_items SET notes = ?1 WHERE id = ?2",
            (notes, item_id),
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(json!({ "ok": true, "item_id": item_id }))
}

fn handle_add_item(conn: &Connection, params: &Value) -> Result<Value, String> {
    let section_id = params
        .get("section_id")
        .and_then(|v| v.as_i64())
        .ok_or("section_id required")?;
    let title = params
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or("title required")?;
    let description = params
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Access control
    if let Some(scoped_id) = get_scoped_project() {
        // section->sprint->project
        let section_project_ok: Result<i64, _> = conn.query_row(
            "SELECT count(*) FROM project_sprint_sections pss
             JOIN project_sprints ps ON pss.sprint_id = ps.id
             WHERE pss.id = ?1 AND ps.project_id = ?2",
            [section_id, scoped_id],
            |row| row.get(0),
        );
        if let Ok(count) = section_project_ok {
            if count == 0 {
                return Err(format!(
                    "Access denied: section {} is not in your scoped project.",
                    section_id
                ));
            }
        }
    }

    let max_order: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM project_items WHERE section_id = ?1",
            [section_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO project_items (section_id, title, description, checked, notes, sort_order, is_custom) VALUES (?1, ?2, ?3, 0, '', ?4, 1)",
        (section_id, title, description, max_order + 1),
    ).map_err(|e| e.to_string())?;

    let item_id = conn.last_insert_rowid();
    Ok(json!({ "ok": true, "item_id": item_id }))
}

fn handle_complete_sprint(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;

    let sprint_id: i64 = conn
        .query_row(
            "SELECT id FROM project_sprints WHERE project_id = ?1 AND status = 'active' LIMIT 1",
            [project_id],
            |row| row.get(0),
        )
        .map_err(|_| "No active sprint".to_string())?;

    // Mark all items as checked
    conn.execute(
        "UPDATE project_items SET checked = 1 WHERE section_id IN \
         (SELECT id FROM project_sprint_sections WHERE sprint_id = ?1)",
        [sprint_id],
    )
    .map_err(|e| e.to_string())?;

    // Mark sprint as done
    conn.execute(
        "UPDATE project_sprints SET status = 'done' WHERE id = ?1",
        [sprint_id],
    )
    .map_err(|e| e.to_string())?;

    // Activate next sprint
    let next: Option<i64> = conn
        .query_row(
            "SELECT id FROM project_sprints WHERE project_id = ?1 AND status = 'pending' ORDER BY sort_order LIMIT 1",
            [project_id],
            |row| row.get(0),
        )
        .ok();

    if let Some(next_id) = next {
        conn.execute(
            "UPDATE project_sprints SET status = 'active' WHERE id = ?1",
            [next_id],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(json!({ "ok": true, "next_sprint_id": next }))
}

fn handle_get_progress(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;

    let (checked, total): (i64, i64) = conn
        .query_row(
            "SELECT \
             (SELECT count(*) FROM project_items pi JOIN project_sprint_sections pss ON pi.section_id = pss.id JOIN project_sprints ps ON pss.sprint_id = ps.id WHERE ps.project_id = ?1 AND pi.checked = 1), \
             (SELECT count(*) FROM project_items pi JOIN project_sprint_sections pss ON pi.section_id = pss.id JOIN project_sprints ps ON pss.sprint_id = ps.id WHERE ps.project_id = ?1)",
            [project_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap_or((0, 0));

    let pct = if total > 0 {
        (checked as f64 / total as f64 * 100.0).round() as i64
    } else {
        0
    };

    Ok(json!({ "checked": checked, "total": total, "percentage": pct }))
}

fn handle_log_error(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;
    let error = params
        .get("error")
        .and_then(|v| v.as_str())
        .ok_or("error required")?;
    let agent_id = get_global_agent_id();

    // Find active sprint
    let sprint_id: i64 = conn
        .query_row(
            "SELECT id FROM project_sprints WHERE project_id = ?1 AND status = 'active' LIMIT 1",
            [project_id],
            |row| row.get(0),
        )
        .map_err(|_| "No active sprint".to_string())?;

    // Find or create "Agent Errors" section
    let section_id: Option<i64> = conn
        .query_row(
            "SELECT id FROM project_sprint_sections WHERE sprint_id = ?1 AND name = 'Agent Errors' LIMIT 1",
            [sprint_id],
            |row| row.get(0),
        )
        .ok();

    let section_id = match section_id {
        Some(id) => id,
        None => {
            let max_order: i64 = conn
                .query_row(
                    "SELECT COALESCE(MAX(sort_order), -1) FROM project_sprint_sections WHERE sprint_id = ?1",
                    [sprint_id],
                    |row| row.get(0),
                )
                .map_err(|e| e.to_string())?;
            conn.execute(
                "INSERT INTO project_sprint_sections (sprint_id, name, description, sort_order, is_custom, linked_from_section_id) VALUES (?1, 'Agent Errors', 'Errors logged by AI agents', ?2, 1, NULL)",
                (sprint_id, max_order + 1),
            ).map_err(|e| e.to_string())?;
            conn.last_insert_rowid()
        }
    };

    let max_item_order: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM project_items WHERE section_id = ?1",
            [section_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default();

    conn.execute(
        "INSERT INTO project_items (section_id, title, description, checked, notes, agent_id, sort_order, is_custom) VALUES (?1, ?2, ?3, 0, ?4, ?5, ?6, 1)",
        (section_id, error, "Error logged by agent", &timestamp, &agent_id, max_item_order + 1),
    ).map_err(|e| e.to_string())?;

    let item_id = conn.last_insert_rowid();
    Ok(json!({ "ok": true, "item_id": item_id }))
}

/// List ALL sprints with status, progress, section counts.
fn handle_get_project_sprints(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;

    let mut s_stmt = conn
        .prepare(
            "SELECT ps.id, ps.name, ps.status, ps.sort_order,
                    (SELECT count(*) FROM project_sprint_sections pss WHERE pss.sprint_id = ps.id),
                    (SELECT count(*) FROM project_items pi \
                     JOIN project_sprint_sections pss ON pi.section_id = pss.id WHERE pss.sprint_id = ps.id),
                    (SELECT count(*) FROM project_items pi \
                     JOIN project_sprint_sections pss ON pi.section_id = pss.id WHERE pss.sprint_id = ps.id AND pi.checked = 1)
             FROM project_sprints ps WHERE ps.project_id = ?1 ORDER BY ps.sort_order",
        )
        .map_err(|e| e.to_string())?;

    let sprints: Vec<Value> = s_stmt
        .query_map([project_id], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "status": row.get::<_, String>(2)?,
                "sort_order": row.get::<_, i64>(3)?,
                "sections": row.get::<_, i64>(4)?,
                "total": row.get::<_, i64>(5)?,
                "checked": row.get::<_, i64>(6)?,
            }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "project_id": project_id, "sprints": sprints }))
}

/// Get any sprint by number (1-based) or name.
fn handle_get_sprint(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;

    let sprint_id: Option<i64> = if let Some(num) = params.get("number").and_then(|v| v.as_i64()) {
        conn.query_row(
            "SELECT id FROM project_sprints WHERE project_id = ?1 AND sort_order = ?2 LIMIT 1",
            [project_id, num - 1],
            |row| row.get(0),
        )
        .ok()
    } else if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
        let escaped = like_escape(name);
        let pattern = format!("%{}%", escaped);
        conn.query_row(
            "SELECT id FROM project_sprints WHERE project_id = ?1 AND name LIKE ?2 ESCAPE '\\' LIMIT 1",
            rusqlite::params![project_id, pattern],
            |row| row.get(0),
        )
        .ok()
    } else {
        return Err("Provide either 'number' or 'name'".to_string());
    };

    let sprint_id = sprint_id.ok_or_else(|| "Sprint not found".to_string())?;

    let mut sec_stmt = conn
        .prepare(
            "SELECT pss.id, pss.name,
                    (SELECT count(*) FROM project_items pi WHERE pi.section_id = pss.id),
                    (SELECT count(*) FROM project_items pi WHERE pi.section_id = pss.id AND pi.checked = 1)
             FROM project_sprint_sections pss
             WHERE pss.sprint_id = ?1 ORDER BY pss.sort_order",
        )
        .map_err(|e| e.to_string())?;

    let sec_rows: Vec<(i64, String, i64, i64)> = sec_stmt
        .query_map([sprint_id], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let mut sections = Vec::new();
    for (sec_id, sec_name, item_count, checked_count) in &sec_rows {
        let mut item_stmt = conn
            .prepare(
                "SELECT id, title, checked FROM project_items WHERE section_id = ?1 ORDER BY sort_order",
            )
            .map_err(|e| e.to_string())?;

        let items: Vec<Value> = item_stmt
            .query_map([sec_id], |r| {
                Ok(json!({
                    "id": r.get::<_, i64>(0)?,
                    "title": r.get::<_, String>(1)?,
                    "checked": r.get::<_, i64>(2)? != 0,
                }))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        sections.push(json!({
            "id": sec_id,
            "name": sec_name,
            "checked": checked_count,
            "total": item_count,
            "items": items,
        }));
    }

    let (name, sort_order): (String, i64) = conn
        .query_row(
            "SELECT name, sort_order FROM project_sprints WHERE id = ?1",
            [sprint_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| e.to_string())?;

    Ok(json!({
        "project_id": project_id,
        "sprint": {
            "id": sprint_id,
            "name": name,
            "sort_order": sort_order,
            "sections": sections,
        }
    }))
}

/// List tasks in active sprint, filtered by status.
fn handle_get_tasks(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;
    let status_filter = params
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("pending");

    let sprint_id: i64 = conn
        .query_row(
            "SELECT id FROM project_sprints WHERE project_id = ?1 AND status = 'active' LIMIT 1",
            [project_id],
            |row| row.get(0),
        )
        .map_err(|_| "No active sprint".to_string())?;

    let checked_val = match status_filter {
        "done" => Some(1i64),
        "pending" => Some(0i64),
        _ => None,
    };

    let mut stmt = match checked_val {
        Some(_) => {
            let query = "SELECT pi.id, pi.title, pi.checked, pss.name as section_name
             FROM project_items pi
             JOIN project_sprint_sections pss ON pi.section_id = pss.id
             WHERE pss.sprint_id = ?1 AND pi.checked = ?2 ORDER BY pss.sort_order, pi.sort_order";
            conn.prepare(query).map_err(|e| e.to_string())?
        }
        None => {
            let query = "SELECT pi.id, pi.title, pi.checked, pss.name as section_name
             FROM project_items pi
             JOIN project_sprint_sections pss ON pi.section_id = pss.id
             WHERE pss.sprint_id = ?1 ORDER BY pss.sort_order, pi.sort_order";
            conn.prepare(query).map_err(|e| e.to_string())?
        }
    };

    let rows: Vec<(i64, String, i64, String)> = match checked_val {
        Some(cv) => stmt
            .query_map(rusqlite::params![sprint_id, cv], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect(),
        None => stmt
            .query_map([sprint_id], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect(),
    };

    let tasks: Vec<Value> = rows
        .into_iter()
        .map(|(id, title, checked, section)| {
            json!({
                "id": id,
                "title": title,
                "checked": checked != 0,
                "section": section,
            })
        })
        .collect();

    let total = tasks.len();
    Ok(json!({ "tasks": tasks, "total": total }))
}

/// Uncheck a task by title (partial match).
fn handle_uncheck_task(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;
    let title = params
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or("title required")?;

    let escaped = like_escape(title);
    let pattern = format!("%{}%", escaped);
    let item_id: Result<i64, _> = conn.query_row(
        "SELECT pi.id FROM project_items pi
         JOIN project_sprint_sections pss ON pi.section_id = pss.id
         JOIN project_sprints ps ON pss.sprint_id = ps.id
         WHERE ps.project_id = ?1 AND ps.status = 'active'
           AND LOWER(pi.title) LIKE LOWER(?2) ESCAPE '\\'
           AND pi.checked = 1
         ORDER BY pi.sort_order LIMIT 1",
        rusqlite::params![project_id, pattern],
        |row| row.get(0),
    );

    let item_id =
        item_id.map_err(|_| format!("No checked task matching '{}' found in active sprint", title))?;

    conn.execute(
        "UPDATE project_items SET checked = 0 WHERE id = ?1",
        [item_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(json!({ "ok": true, "item_id": item_id, "title": title }))
}

/// Add a new section to the active sprint.
/// Idempotent: if a section with the same name already exists in this sprint,
/// returns that section instead of creating a duplicate.
fn handle_add_section(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;
    let name = params
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or("name required")?;
    let description = params
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let sprint_id: i64 = conn
        .query_row(
            "SELECT id FROM project_sprints WHERE project_id = ?1 AND status = 'active' LIMIT 1",
            [project_id],
            |row| row.get(0),
        )
        .map_err(|_| "No active sprint".to_string())?;

    // Return existing section if name matches (case-insensitive) — prevents duplicates
    if let Ok(existing_id) = conn.query_row(
        "SELECT id FROM project_sprint_sections WHERE sprint_id = ?1 AND lower(name) = lower(?2) LIMIT 1",
        rusqlite::params![sprint_id, name],
        |row| row.get::<_, i64>(0),
    ) {
        return Ok(json!({ "ok": true, "section_id": existing_id, "name": name, "existed": true }));
    }

    let max_order: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM project_sprint_sections WHERE sprint_id = ?1",
            [sprint_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO project_sprint_sections (sprint_id, name, description, sort_order, is_custom, linked_from_section_id) VALUES (?1, ?2, ?3, ?4, 1, NULL)",
        (sprint_id, name, description, max_order + 1),
    )
    .map_err(|e| e.to_string())?;

    let section_id = conn.last_insert_rowid();
    Ok(json!({ "ok": true, "section_id": section_id, "name": name, "existed": false }))
}

/// Search all tasks in the project by keyword.
fn handle_search_tasks(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_project_id(params, conn)?;
    let query = params
        .get("query")
        .and_then(|v| v.as_str())
        .ok_or("query required")?;

    let escaped = like_escape(query);
    let pattern = format!("%{}%", escaped);
    let mut stmt = conn
        .prepare(
            "SELECT pi.id, pi.title, pi.checked, pss.name as section_name, ps.name as sprint_name, ps.status
             FROM project_items pi
             JOIN project_sprint_sections pss ON pi.section_id = pss.id
             JOIN project_sprints ps ON pss.sprint_id = ps.id
             WHERE ps.project_id = ?1 AND LOWER(pi.title) LIKE LOWER(?2) ESCAPE '\\'
             ORDER BY ps.sort_order, pss.sort_order, pi.sort_order",
        )
        .map_err(|e| e.to_string())?;

    let rows: Vec<(i64, String, i64, String, String, String)> = stmt
        .query_map(rusqlite::params![project_id, pattern], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let tasks: Vec<Value> = rows
        .into_iter()
        .map(|(id, title, checked, section, sprint, status)| {
            json!({
                "id": id,
                "title": title,
                "checked": checked != 0,
                "section": section,
                "sprint": sprint,
                "sprint_status": status,
            })
        })
        .collect();

    let total = tasks.len();
    Ok(json!({ "tasks": tasks, "total": total }))
}

// ─── Helpers ───

/// Mark sprint done and activate next if all items are done.
fn auto_advance_sprint(conn: &Connection, item_id: i64) -> Result<(), String> {
    let result: Result<(i64, i64), _> = conn.query_row(
        "SELECT prs.project_id, pss.sprint_id FROM project_items pi
         JOIN project_sprint_sections pss ON pi.section_id = pss.id
         JOIN project_sprints prs ON pss.sprint_id = prs.id
         WHERE pi.id = ?1",
        [item_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    );
    if let Ok((project_id, sprint_id)) = result {
        let counts: Result<(i64, i64), _> = conn.query_row(
            "SELECT
               (SELECT count(*) FROM project_items pi2 JOIN project_sprint_sections pss2 ON pi2.section_id = pss2.id WHERE pss2.sprint_id = ?1),
               (SELECT count(*) FROM project_items pi2 JOIN project_sprint_sections pss2 ON pi2.section_id = pss2.id WHERE pss2.sprint_id = ?1 AND pi2.checked = 1)",
            [sprint_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        );
        if let Ok((total, done)) = counts {
            if total > 0 && total == done {
                let _ = conn.execute(
                    "UPDATE project_sprints SET status = 'done' WHERE id = ?1 AND status = 'active'",
                    [sprint_id],
                );
                let next: Option<i64> = conn
                    .query_row(
                        "SELECT id FROM project_sprints WHERE project_id = ?1 AND status = 'pending' ORDER BY sort_order LIMIT 1",
                        [project_id],
                        |row| row.get::<_, i64>(0),
                    )
                    .ok();
                if let Some(next_id) = next {
                    let _ = conn.execute(
                        "UPDATE project_sprints SET status = 'active' WHERE id = ?1",
                        [next_id],
                    );
                }
            }
        }
    }
    Ok(())
}

// Global agent ID storage (set during initialize)
static GLOBAL_AGENT_ID: OnceLock<String> = OnceLock::new();

fn set_global_agent_id(id: String) {
    let _ = GLOBAL_AGENT_ID.set(id);
}

fn get_global_agent_id() -> String {
    GLOBAL_AGENT_ID
        .get()
        .cloned()
        .unwrap_or_else(|| "unknown".to_string())
}

// ─── Main Loop ───

fn main() {
    let db_path = get_db_path();
    if !db_path.exists() {
        eprintln!("Database not found at {:?}", db_path);
        eprintln!("Make sure the DevStar app has been run at least once.");
        std::process::exit(1);
    }

    let conn = Connection::open(&db_path).expect("Failed to open DB");
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .ok();

    // Startup migration: merge duplicate project_sprint_sections within the same sprint.
    // This cleans up any duplicates created by a previous bug where add_task created a
    // new section instead of finding the existing one with the same name.
    conn.execute_batch(
        "UPDATE project_items
         SET section_id = (
             SELECT MIN(s.id)
             FROM project_sprint_sections s
             JOIN project_sprint_sections src ON src.id = project_items.section_id
             WHERE s.sprint_id = src.sprint_id
               AND lower(s.name) = lower(src.name)
         )
         WHERE section_id NOT IN (
             SELECT MIN(id) FROM project_sprint_sections GROUP BY sprint_id, lower(name)
         );
         DELETE FROM project_sprint_sections
         WHERE id NOT IN (
             SELECT MIN(id) FROM project_sprint_sections GROUP BY sprint_id, lower(name)
         );"
    ).ok();
    let conn = Mutex::new(conn);

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        if line.trim().is_empty() {
            continue;
        }

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {}", e),
                    }),
                };
                writeln!(stdout, "{}", serde_json::to_string(&resp).unwrap()).ok();
                stdout.flush().ok();
                continue;
            }
        };

        let is_notification = request.id.is_none();
        let conn_guard = conn.lock().unwrap_or_else(|e| e.into_inner());

        let result = match request.method.as_str() {
            "initialize" => {
                // Extract client info for agent identity
                let client_name = request
                    .params
                    .as_ref()
                    .and_then(|p| p.get("clientInfo"))
                    .and_then(|c| c.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown-agent");

                let agent_id = get_or_create_agent_id(client_name);
                set_global_agent_id(agent_id.clone());

                // scope session to project if project_uuid is provided
                let params = request.params.as_ref().cloned().unwrap_or(json!({}));
                if let Some(uuid) = params.get("project_uuid").and_then(|v| v.as_str()) {
                    let conn_ref_init: &Connection = &conn_guard;
                    if let Some(pid) = get_project_id_from_uuid(conn_ref_init, uuid) {
                        set_active_project(pid);
                    }
                }

                Ok(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": { "tools": {} },
                    "serverInfo": { "name": "devstar-mcp", "version": "0.1.0" },
                    "agent_id": agent_id,
                }))
            }
            "initialized" => Ok(null_value()),
            "tools/list" => Ok(json!({ "tools": tool_definitions() })),
            "tools/call" => {
                let conn_ref: &Connection = &conn_guard;
                let params = request.params.as_ref().unwrap();
                let tool = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let tool_params = params.get("arguments").cloned().unwrap_or(json!({}));

                let tool_result = match tool {
                    "list_templates" => handle_list_templates(conn_ref),
                    "get_template" => handle_get_template(conn_ref, &tool_params),
                    "list_shared_sections" => handle_list_shared_sections(conn_ref),
                    "list_shared_sprints" => handle_list_shared_sprints(conn_ref),
                    "dashboard" => handle_dashboard(conn_ref),
                    "create_project" => handle_create_project(conn_ref, &tool_params),
                    "get_project_context" => handle_get_project_context(conn_ref, &tool_params),
                    "get_active_sprint_detail" => handle_get_active_sprint_detail(conn_ref, &tool_params),
                    "add_task" => handle_add_task(conn_ref, &tool_params),
                    "check_task" => handle_check_task(conn_ref, &tool_params),
                    "update_item" => handle_update_item(conn_ref, &tool_params),
                    "add_item" => handle_add_item(conn_ref, &tool_params),
                    "complete_sprint" => handle_complete_sprint(conn_ref, &tool_params),
                    "get_progress" => handle_get_progress(conn_ref, &tool_params),
                    "log_error" => handle_log_error(conn_ref, &tool_params),
                    "get_project_sprints" => handle_get_project_sprints(conn_ref, &tool_params),
                    "get_sprint" => handle_get_sprint(conn_ref, &tool_params),
                    "get_tasks" => handle_get_tasks(conn_ref, &tool_params),
                    "uncheck_task" => handle_uncheck_task(conn_ref, &tool_params),
                    "add_section" => handle_add_section(conn_ref, &tool_params),
                    "search_tasks" => handle_search_tasks(conn_ref, &tool_params),
                    "get_full_project_plan" => handle_get_full_project_plan(conn_ref, &tool_params),
                    _ => Err(format!("Unknown tool: {}", tool)),
                };

                // Wrap in MCP-compliant content envelope
                match tool_result {
                    Ok(v) => Ok(json!({
                        "content": [{
                            "type": "text",
                            "text": serde_json::to_string(&v).unwrap_or_default()
                        }],
                        "isError": false
                    })),
                    Err(e) => Ok(json!({
                        "content": [{
                            "type": "text",
                            "text": e
                        }],
                        "isError": true
                    })),
                }
            }
            _ => Ok(null_value()),
        };

        drop(conn_guard);

        if is_notification {
            continue;
        }

        let resp = match result {
            Ok(v) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id.clone(),
                result: Some(v),
                error: None,
            },
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id.clone(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32603,
                    message: e,
                }),
            },
        };

        writeln!(stdout, "{}", serde_json::to_string(&resp).unwrap()).ok();
        stdout.flush().ok();
    }
}

fn null_value() -> Value {
    Value::Null
}