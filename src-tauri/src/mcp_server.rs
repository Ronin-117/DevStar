//! DevStar MCP Server — stdio transport for AI agents
//!
//! Usage: `devstar-mcp`
//! Connects via JSON-RPC over stdio following the MCP protocol.
//! Shares the same SQLite database as the main Tauri app.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::type_complexity)]
#![allow(clippy::map_entry)]

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::sync::OnceLock;
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
    let content = std::fs::read_to_string(&config_path).map_err(|e| {
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
//
// Rules:
// - If .devstar.json exists in working directory, the agent is SCOPED to that project.
//   Project-scoped tools (get_project, get_active_sprint, update_item, add_item, etc.)
//   will only work on the scoped project.
// - Shared resources (templates, shared_sections, shared_sprints) are always accessible.
// - create_project is always accessible (creates new project + writes .devstar.json).
//
// This means: agents in different project directories can only see/edit their own projects.
// The human user via the UI can see ALL projects.

/// Returns the scoped project_id from .devstar.json if it exists, otherwise None.
fn get_scoped_project() -> Option<i64> {
    if let Ok(ctx) = read_project_context(".") {
        ctx.get("project_id").and_then(|v| v.as_i64())
    } else {
        None
    }
}

/// If .devstar.json exists, verify the given project_id matches the scoped project.
fn verify_project_access(project_id: i64) -> Result<(), String> {
    if let Some(scoped_id) = get_scoped_project() {
        if project_id != scoped_id {
            return Err(format!(
                "Access denied: .devstar.json in this directory scopes you to project {}. Requested project {}.",
                scoped_id, project_id
            ));
        }
    }
    Ok(())
}

/// If .devstar.json exists, return the scoped project_id. Otherwise error.
fn require_scoped_project() -> Result<i64, String> {
    get_scoped_project().ok_or_else(|| {
        "No .devstar.json found in this directory. Agents must work in a project-scoped directory. Run `create_project` first.".to_string()
    })
}

// ─── MCP Protocol Types ───

#[derive(Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
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

// ─── Tool Definitions ───

fn tool_definitions() -> Value {
    json!([
        {
            "name": "get_project_context",
            "description": "Zero-config discovery. Reads .devstar.json from the working directory and returns compact project overview with active sprint sections/items.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_dir": { "type": "string", "description": "Directory containing .devstar.json (default: current directory)" }
                }
            }
        },
        {
            "name": "dashboard",
            "description": "Compact overview of ALL projects: name, progress %, active sprint. Use to survey the workspace.",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "create_project",
            "description": "Create a new project from a template. Writes .devstar.json to project_dir for agent scoping.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Project name" },
                    "template_id": { "type": "integer", "description": "Template ID (use list_templates to find)" },
                    "description": { "type": "string", "description": "Optional description" },
                    "color": { "type": "string", "description": "Optional hex color" },
                    "project_dir": { "type": "string", "description": "Directory to write .devstar.json (default: current dir)" }
                },
                "required": ["name", "template_id"]
            }
        },
        {
            "name": "get_active_sprint_detail",
            "description": "Get the active sprint with all sections and items. Use to see what tasks need to be done. Only works in a scoped project.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "integer", "description": "Project ID (uses .devstar.json if omitted)" }
                }
            }
        },
        {
            "name": "add_task",
            "description": "Add a task to the active sprint. Specify section name or omit to add to first section. No section ID needed.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "title": { "type": "string", "description": "Task title" },
                    "description": { "type": "string", "description": "Optional description" },
                    "section_name": { "type": "string", "description": "Section name (optional, uses first section if omitted)" }
                },
                "required": ["title"]
            }
        },
        {
            "name": "check_task",
            "description": "Check off a task by title (partial match). No item ID needed.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "title": { "type": "string", "description": "Task title (partial match, case-insensitive)" }
                },
                "required": ["title"]
            }
        },
        {
            "name": "update_item",
            "description": "Check/uncheck an item by ID or add notes.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "item_id": { "type": "integer", "description": "Item ID" },
                    "checked": { "type": "boolean", "description": "Whether the item is checked" },
                    "notes": { "type": "string", "description": "Optional notes" }
                },
                "required": ["item_id"]
            }
        },
        {
            "name": "add_item",
            "description": "Add a custom item to a specific section (need section_id).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "section_id": { "type": "integer", "description": "Section ID" },
                    "title": { "type": "string", "description": "Item title" },
                    "description": { "type": "string", "description": "Optional description" }
                },
                "required": ["section_id", "title"]
            }
        },
        {
            "name": "complete_sprint",
            "description": "Mark all items in active sprint done and advance to next sprint.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "integer", "description": "Project ID (uses .devstar.json if omitted)" }
                }
            }
        },
        {
            "name": "get_progress",
            "description": "Get completion stats for the scoped project.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "integer", "description": "Project ID (uses .devstar.json if omitted)" }
                }
            }
        },
        {
            "name": "log_error",
            "description": "Log an error as an unchecked item in the active sprint. Auto-creates 'Agent Errors' section.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "error": { "type": "string", "description": "Error message" }
                },
                "required": ["error"]
            }
        },
        {
            "name": "get_project_sprints",
            "description": "List ALL sprints with status, progress, and section counts. Use to understand the full project plan.",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "get_sprint",
            "description": "Get any sprint by number (1-based) or name with sections and items. Use to inspect past/upcoming sprints.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "number": { "type": "integer", "description": "Sprint number (1-based)" },
                    "name": { "type": "string", "description": "Sprint name (partial match)" }
                }
            }
        },
        {
            "name": "get_tasks",
            "description": "List tasks in the active sprint, optionally filtered by status. Use to see what's left vs what's done.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "status": { "type": "string", "enum": ["pending", "done", "all"], "description": "Filter: 'pending' (unchecked), 'done' (checked), or 'all'" }
                }
            }
        },
        {
            "name": "uncheck_task",
            "description": "Uncheck a task by title (partial match). Undo accidental check.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "title": { "type": "string", "description": "Task title (partial match, case-insensitive)" }
                },
                "required": ["title"]
            }
        },
        {
            "name": "add_section",
            "description": "Add a new section to the active sprint. Use to organize tasks into new categories.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Section name" },
                    "description": { "type": "string", "description": "Optional description" }
                },
                "required": ["name"]
            }
        },
        {
            "name": "search_tasks",
            "description": "Search all tasks in the project by keyword. Finds matching tasks across all sections and sprints.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search term (case-insensitive partial match)" }
                },
                "required": ["query"]
            }
        },
        {
            "name": "list_templates",
            "description": "List all templates with sprint counts. Anyone can access this.",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "get_template",
            "description": "Get a template's sprints, sections, and items.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "template_id": { "type": "integer", "description": "Template ID" }
                },
                "required": ["template_id"]
            }
        },
        {
            "name": "list_shared_sections",
            "description": "List all shared sections with item counts.",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "list_shared_sprints",
            "description": "List all shared sprints with section counts.",
            "inputSchema": { "type": "object", "properties": {} }
        }
    ])
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
                "sprints": row.get::<_, i64>(2)?,
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

    let mut s_stmt = conn
        .prepare(
            "SELECT ts.id, ts.name, ts.description, ts.sort_order,
                    (SELECT count(*) FROM template_sprint_sections tss WHERE tss.sprint_id = ts.id)
             FROM template_sprints ts WHERE ts.template_id = ?1 ORDER BY ts.sort_order",
        )
        .map_err(|e| e.to_string())?;

    let sprints: Vec<Value> = s_stmt
        .query_map([template_id], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "description": row.get::<_, String>(2)?,
                "sort_order": row.get::<_, i64>(3)?,
                "sections": row.get::<_, i64>(4)?,
            }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "template": template, "sprints": sprints }))
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
            "agent_id": get_agent_id_for_project(project_id),
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

    conn.execute(
        "INSERT INTO projects (name, description, template_id, color) VALUES (?1, ?2, ?3, ?4)",
        (name, description, template_id, color),
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
    write_project_context(&dir, project_id, name, template_id)?;

    Ok(json!({ "project_id": project_id, "name": name }))
}

/// Get active sprint detail for scoped project (no project_id needed if .devstar.json exists).
fn handle_get_active_sprint_detail(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = if let Some(pid) = params.get("project_id").and_then(|v| v.as_i64()) {
        verify_project_access(pid)?;
        pid
    } else {
        require_scoped_project()?
    };

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
    let project_id = require_scoped_project()?;
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
        // Try to find existing section
        let existing: Option<i64> = conn.query_row(
                "SELECT id FROM project_sprint_sections WHERE sprint_id = ?1 AND name = ?2 LIMIT 1",
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
    let project_id = require_scoped_project()?;
    let title = params
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or("title required")?;

    // Find the item in the active sprint
    let pattern = format!("%{}%", title);
    let item_id: Result<i64, _> = conn.query_row(
        "SELECT pi.id FROM project_items pi
         JOIN project_sprint_sections pss ON pi.section_id = pss.id
         JOIN project_sprints ps ON pss.sprint_id = ps.id
         WHERE ps.project_id = ?1 AND ps.status = 'active'
           AND LOWER(pi.title) LIKE LOWER(?2)
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
        let project_ok: Result<i64, _> = conn.query_row(
            "SELECT count(*) FROM project_items pi
             JOIN project_sprint_sections pss ON pi.section_id = pss.id
             JOIN project_sprints ps ON pss.sprint_id = ps.id
             WHERE pss.id = ?1 AND ps.project_id = ?2",
            [section_id, scoped_id],
            |row| row.get(0),
        );
        // section might not have items yet, check via section->sprint->project
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
    let project_id = if let Some(pid) = params.get("project_id").and_then(|v| v.as_i64()) {
        verify_project_access(pid)?;
        pid
    } else {
        require_scoped_project()?
    };

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
    let project_id = if let Some(pid) = params.get("project_id").and_then(|v| v.as_i64()) {
        verify_project_access(pid)?;
        pid
    } else {
        require_scoped_project()?
    };

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
    let project_id = require_scoped_project()?;
    let error = params
        .get("error")
        .and_then(|v| v.as_str())
        .ok_or("error required")?;
    let agent_id = get_agent_id_for_project(project_id);

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
    let item_title = format!("[{}] {}", agent_id, error);

    conn.execute(
        "INSERT INTO project_items (section_id, title, description, checked, notes, sort_order, is_custom) VALUES (?1, ?2, ?3, 0, ?4, ?5, 1)",
        (section_id, &item_title, "Error logged by agent", &timestamp, max_item_order + 1),
    ).map_err(|e| e.to_string())?;

    let item_id = conn.last_insert_rowid();
    Ok(json!({ "ok": true, "item_id": item_id }))
}

/// List ALL sprints with status, progress, section counts.
fn handle_get_project_sprints(conn: &Connection) -> Result<Value, String> {
    let project_id = require_scoped_project()?;

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
    let project_id = require_scoped_project()?;

    let sprint_id: Option<i64> = if let Some(num) = params.get("number").and_then(|v| v.as_i64()) {
        conn.query_row(
            "SELECT id FROM project_sprints WHERE project_id = ?1 AND sort_order = ?2 LIMIT 1",
            [project_id, num - 1],
            |row| row.get(0),
        )
        .ok()
    } else if let Some(name) = params.get("name").and_then(|v| v.as_str()) {
        conn.query_row(
            "SELECT id FROM project_sprints WHERE project_id = ?1 AND name LIKE ?2 LIMIT 1",
            rusqlite::params![project_id, format!("%{}%", name)],
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
    let project_id = require_scoped_project()?;
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
        Some(cv) => {
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

    Ok(json!({ "tasks": tasks, "total": tasks.len() }))
}

/// Uncheck a task by title (partial match).
fn handle_uncheck_task(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_scoped_project()?;
    let title = params
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or("title required")?;

    let pattern = format!("%{}%", title);
    let item_id: Result<i64, _> = conn.query_row(
        "SELECT pi.id FROM project_items pi
         JOIN project_sprint_sections pss ON pi.section_id = pss.id
         JOIN project_sprints ps ON pss.sprint_id = ps.id
         WHERE ps.project_id = ?1 AND ps.status = 'active'
           AND LOWER(pi.title) LIKE LOWER(?2)
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
fn handle_add_section(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_scoped_project()?;
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
    Ok(json!({ "ok": true, "section_id": section_id, "name": name }))
}

/// Search all tasks in the project by keyword.
fn handle_search_tasks(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = require_scoped_project()?;
    let query = params
        .get("query")
        .and_then(|v| v.as_str())
        .ok_or("query required")?;

    let pattern = format!("%{}%", query);
    let mut stmt = conn
        .prepare(
            "SELECT pi.id, pi.title, pi.checked, pss.name as section_name, ps.name as sprint_name, ps.status
             FROM project_items pi
             JOIN project_sprint_sections pss ON pi.section_id = pss.id
             JOIN project_sprints ps ON pss.sprint_id = ps.id
             WHERE ps.project_id = ?1 AND LOWER(pi.title) LIKE LOWER(?2)
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

    Ok(json!({ "tasks": tasks, "total": tasks.len() }))
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

/// Get agent ID for a project — uses the agent tracking based on client name from initialize.
fn get_agent_id_for_project(_project_id: i64) -> String {
    // Use the global agent_id since the MCP process tracks a single client identity
    get_global_agent_id()
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

        let conn = if is_notification {
            None
        } else {
            match Connection::open(&db_path) {
                Ok(c) => Some(c),
                Err(e) => {
                    let resp = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id.clone(),
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32603,
                            message: format!("DB error: {}", e),
                        }),
                    };
                    writeln!(stdout, "{}", serde_json::to_string(&resp).unwrap()).ok();
                    stdout.flush().ok();
                    continue;
                }
            }
        };

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
                let conn_ref = conn.as_ref().unwrap();
                let params = request.params.as_ref().unwrap();
                let tool = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let tool_params = params.get("arguments").cloned().unwrap_or(json!({}));

                match tool {
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
                    "get_project_sprints" => handle_get_project_sprints(conn_ref),
                    "get_sprint" => handle_get_sprint(conn_ref, &tool_params),
                    "get_tasks" => handle_get_tasks(conn_ref, &tool_params),
                    "uncheck_task" => handle_uncheck_task(conn_ref, &tool_params),
                    "add_section" => handle_add_section(conn_ref, &tool_params),
                    "search_tasks" => handle_search_tasks(conn_ref, &tool_params),
                    _ => Err(format!("Unknown tool: {}", tool)),
                }
            }
            _ => Ok(null_value()),
        };

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
