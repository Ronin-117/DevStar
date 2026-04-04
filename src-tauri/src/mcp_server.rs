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
use uuid::Uuid;

// ─── Database path (same as main app) ───

fn get_db_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("com.njne2.devstar")
        .join("devstar.db")
}

fn get_agent_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".devstar")
        .join("agents")
}

fn get_agent_id() -> String {
    let dir = get_agent_dir();
    std::fs::create_dir_all(&dir).ok();
    // Use process name as agent identifier (e.g., "claude-code", "opencode", "antigravity")
    let process_name = std::env::current_exe()
        .ok()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_string()))
        .unwrap_or_else(|| "unknown-agent".to_string());
    let path = dir.join(format!("{}.json", process_name));
    if let Ok(content) = std::fs::read_to_string(&path) {
        if let Ok(v) = serde_json::from_str::<Value>(&content) {
            if let Some(id) = v.get("id").and_then(|v| v.as_str()) {
                return id.to_string();
            }
        }
    }
    let id = Uuid::new_v4().to_string();
    let data = json!({ "id": id, "name": process_name, "first_seen": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) });
    std::fs::write(&path, serde_json::to_string(&data).unwrap_or_default()).ok();
    id
}

fn read_project_context(project_dir: &str) -> Result<Value, String> {
    let dir = if project_dir == "." || project_dir.is_empty() {
        std::env::current_dir().map_err(|e| e.to_string())?
    } else {
        PathBuf::from(project_dir)
    };
    let config_path = dir.join(".devstar.json");
    let content = std::fs::read_to_string(&config_path).map_err(|e| {
        format!(
            "No .devstar.json found in {}. Create a project first with create_project.",
            dir.display()
        )
    })?;
    let config: Value =
        serde_json::from_str(&content).map_err(|e| format!("Invalid .devstar.json: {}", e))?;
    let project_id = config
        .get("project_id")
        .and_then(|v| v.as_i64())
        .ok_or("Missing project_id in .devstar.json")?;
    Ok(json!({ "project_id": project_id, "config": config }))
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
        "created_at": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
    });
    std::fs::write(
        &config_path,
        serde_json::to_string_pretty(&config).unwrap_or_default(),
    )
    .map_err(|e| format!("Failed to write .devstar.json: {}", e))?;
    Ok(())
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
    result: Option<Value>,
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
            "name": "list_templates",
            "description": "List all available templates with sprint and section counts",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "get_template",
            "description": "Get a template's sprints, sections, and items",
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
            "description": "List all shared sections with item counts",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "list_shared_sprints",
            "description": "List all shared sprints with section counts",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "create_project",
            "description": "Create a new project from a template",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string", "description": "Project name" },
                    "template_id": { "type": "integer", "description": "Template ID to create from" },
                    "description": { "type": "string", "description": "Optional description" },
                    "color": { "type": "string", "description": "Optional hex color" },
                    "project_dir": { "type": "string", "description": "Directory to write .devstar.json config to (optional)" }
                },
                "required": ["name", "template_id"]
            }
        },
        {
            "name": "get_project_context",
            "description": "Read .devstar.json from the current or specified directory and return full project details including active sprint",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_dir": { "type": "string", "description": "Directory containing .devstar.json (default: current directory)" }
                }
            }
        },
        {
            "name": "get_project",
            "description": "Get project details with sprints, sections, items, and progress",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "integer", "description": "Project ID" }
                },
                "required": ["project_id"]
            }
        },
        {
            "name": "get_active_sprint",
            "description": "Get the current active sprint for a project",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "integer", "description": "Project ID" }
                },
                "required": ["project_id"]
            }
        },
        {
            "name": "update_item",
            "description": "Check/uncheck an item or add notes",
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
            "description": "Add a custom item to a section",
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
            "name": "set_sprint_status",
            "description": "Set a sprint's status (pending, active, done)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "sprint_id": { "type": "integer", "description": "Sprint ID" },
                    "status": { "type": "string", "enum": ["pending", "active", "done"] }
                },
                "required": ["sprint_id", "status"]
            }
        },
        {
            "name": "complete_sprint",
            "description": "Mark all items in a sprint as done and advance to the next sprint",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "sprint_id": { "type": "integer", "description": "Sprint ID" },
                    "project_id": { "type": "integer", "description": "Project ID" }
                },
                "required": ["sprint_id", "project_id"]
            }
        },
        {
            "name": "get_progress",
            "description": "Get completion stats for a project (checked/total items)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "integer", "description": "Project ID" }
                },
                "required": ["project_id"]
            }
        },
        {
            "name": "log_error",
            "description": "Log an error as an unchecked todo item in an 'Agent Errors' section",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "integer", "description": "Project ID" },
                    "sprint_id": { "type": "integer", "description": "Sprint ID to add error to" },
                    "error": { "type": "string", "description": "Error message" }
                },
                "required": ["project_id", "sprint_id", "error"]
            }
        }
    ])
}

// ─── Tool Handlers ───

fn handle_list_templates(conn: &Connection) -> Result<Value, String> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, t.description, t.color,
                (SELECT count(*) FROM template_sprints ts WHERE ts.template_id = t.id) as sprint_count
         FROM templates t ORDER BY t.name"
    ).map_err(|e| e.to_string())?;

    let templates: Vec<Value> = stmt
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "description": row.get::<_, String>(2)?,
                "color": row.get::<_, String>(3)?,
                "sprint_count": row.get::<_, i64>(4)?,
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
        .prepare("SELECT id, name, description, color FROM templates WHERE id = ?1")
        .map_err(|e| e.to_string())?;
    let template: Value = t_stmt
        .query_row([template_id], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "description": row.get::<_, String>(2)?,
                "color": row.get::<_, String>(3)?,
            }))
        })
        .map_err(|e| e.to_string())?;

    let mut s_stmt = conn.prepare(
        "SELECT ts.id, ts.name, ts.description, ts.sort_order,
                (SELECT count(*) FROM template_sprint_sections tss WHERE tss.sprint_id = ts.id) as section_count
         FROM template_sprints ts WHERE ts.template_id = ?1 ORDER BY ts.sort_order"
    ).map_err(|e| e.to_string())?;

    let sprints: Vec<Value> = s_stmt
        .query_map([template_id], |row| {
            let sprint_id: i64 = row.get(0)?;
            let mut sec_stmt = conn
                .prepare(
                    "SELECT tss.id, ss.name, ss.description, ss.color, tss.is_linked
             FROM template_sprint_sections tss
             JOIN shared_sections ss ON tss.section_id = ss.id
             WHERE tss.sprint_id = ?1 ORDER BY tss.sort_order",
                )
                .map_err(|_| rusqlite::Error::InvalidQuery)?;

            let sections: Vec<Value> = sec_stmt
                .query_map([sprint_id], |r| {
                    Ok(json!({
                        "id": r.get::<_, i64>(0)?,
                        "name": r.get::<_, String>(1)?,
                        "description": r.get::<_, String>(2)?,
                        "color": r.get::<_, String>(3)?,
                        "is_linked": r.get::<_, bool>(4)?,
                    }))
                })
                .map_err(|_| rusqlite::Error::InvalidQuery)?
                .filter_map(|r| r.ok())
                .collect();

            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "description": row.get::<_, String>(2)?,
                "sort_order": row.get::<_, i64>(3)?,
                "section_count": row.get::<_, i64>(4)?,
                "sections": sections,
            }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "template": template, "sprints": sprints }))
}

fn handle_list_shared_sections(conn: &Connection) -> Result<Value, String> {
    let mut stmt = conn.prepare(
        "SELECT ss.id, ss.name, ss.description, ss.color,
                (SELECT count(*) FROM shared_section_items ssi WHERE ssi.section_id = ss.id) as item_count
         FROM shared_sections ss ORDER BY ss.name"
    ).map_err(|e| e.to_string())?;

    let sections: Vec<Value> = stmt
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "description": row.get::<_, String>(2)?,
                "color": row.get::<_, String>(3)?,
                "item_count": row.get::<_, i64>(4)?,
            }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "sections": sections }))
}

fn handle_list_shared_sprints(conn: &Connection) -> Result<Value, String> {
    let mut stmt = conn.prepare(
        "SELECT ss.id, ss.name, ss.description,
                (SELECT count(*) FROM shared_sprint_sections sss WHERE sss.sprint_id = ss.id) as section_count
         FROM shared_sprints ss ORDER BY ss.sort_order"
    ).map_err(|e| e.to_string())?;

    let sprints: Vec<Value> = stmt
        .query_map([], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "description": row.get::<_, String>(2)?,
                "section_count": row.get::<_, i64>(3)?,
            }))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(json!({ "sprints": sprints }))
}

fn handle_get_project_context(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_dir = params
        .get("project_dir")
        .and_then(|v| v.as_str())
        .unwrap_or(".");
    let ctx = read_project_context(project_dir)?;
    let project_id = ctx.get("project_id").and_then(|v| v.as_i64()).unwrap();

    // Get full project details
    let project = handle_get_project(conn, &json!({ "project_id": project_id }))?;
    let active = handle_get_active_sprint(conn, &json!({ "project_id": project_id }))?;

    Ok(json!({
        "project_dir": project_dir,
        "project_id": project_id,
        "project": project.get("project"),
        "sprints": project.get("sprints"),
        "progress": project.get("progress"),
        "active_sprint": active.get("sprint"),
        "active_sprint_sections": active.get("sections"),
    }))
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
    let mut t_sprints = conn.prepare(
        "SELECT id, name, description, sort_order FROM template_sprints WHERE template_id = ?1 ORDER BY sort_order"
    ).map_err(|e| e.to_string())?;

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
        let mut t_sections = conn.prepare(
            "SELECT tss.section_id, tss.sort_order FROM template_sprint_sections tss WHERE tss.sprint_id = ?1 ORDER BY tss.sort_order"
        ).map_err(|e| e.to_string())?;

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
            let mut items = conn.prepare(
                "SELECT title, description, sort_order FROM shared_section_items WHERE section_id = ?1 ORDER BY sort_order"
            ).map_err(|e| e.to_string())?;

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

    // Write .devstar.json if project_dir specified
    if !project_dir.is_empty() {
        write_project_context(project_dir, project_id, name, template_id)?;
    }

    Ok(json!({ "project_id": project_id, "name": name }))
}

fn handle_get_project(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = params
        .get("project_id")
        .and_then(|v| v.as_i64())
        .ok_or("project_id required")?;

    let mut p_stmt = conn
        .prepare("SELECT id, name, description, template_id, color FROM projects WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    let project: Value = p_stmt
        .query_row([project_id], |row| {
            Ok(json!({
                "id": row.get::<_, i64>(0)?,
                "name": row.get::<_, String>(1)?,
                "description": row.get::<_, String>(2)?,
                "template_id": row.get::<_, i64>(3)?,
                "color": row.get::<_, String>(4)?,
            }))
        })
        .map_err(|e| e.to_string())?;

    let mut s_stmt = conn.prepare(
        "SELECT ps.id, ps.name, ps.description, ps.status, ps.sort_order,
                (SELECT count(*) FROM project_sprint_sections pss WHERE pss.sprint_id = ps.id) as section_count
         FROM project_sprints ps WHERE ps.project_id = ?1 ORDER BY ps.sort_order"
    ).map_err(|e| e.to_string())?;

    let sprints: Vec<Value> = s_stmt.query_map([project_id], |row| {
        let sprint_id: i64 = row.get(0)?;
        let mut sec_stmt = conn.prepare(
            "SELECT pss.id, pss.name, pss.description, pss.is_custom, pss.linked_from_section_id,
                    (SELECT count(*) FROM project_items pi WHERE pi.section_id = pss.id) as item_count,
                    (SELECT count(*) FROM project_items pi WHERE pi.section_id = pss.id AND pi.checked = 1) as checked_count
             FROM project_sprint_sections pss WHERE pss.sprint_id = ?1 ORDER BY pss.sort_order"
        ).map_err(|_| rusqlite::Error::InvalidQuery)?;

        let sections: Vec<Value> = sec_stmt.query_map([sprint_id], |r| {
            Ok(json!({
                "id": r.get::<_, i64>(0)?,
                "name": r.get::<_, String>(1)?,
                "description": r.get::<_, String>(2)?,
                "is_custom": r.get::<_, bool>(3)?,
                "linked_from_section_id": r.get::<_, Option<i64>>(4)?,
                "item_count": r.get::<_, i64>(5)?,
                "checked_count": r.get::<_, i64>(6)?,
            }))
        }).map_err(|_| rusqlite::Error::InvalidQuery)?
        .filter_map(|r| r.ok())
        .collect();

        Ok(json!({
            "id": row.get::<_, i64>(0)?,
            "name": row.get::<_, String>(1)?,
            "description": row.get::<_, String>(2)?,
            "status": row.get::<_, String>(3)?,
            "sort_order": row.get::<_, i64>(4)?,
            "section_count": row.get::<_, i64>(5)?,
            "sections": sections,
        }))
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();

    let (checked, total): (i64, i64) = conn.query_row(
        "SELECT \
            (SELECT count(*) FROM project_items pi JOIN project_sprint_sections pss ON pi.section_id = pss.id JOIN project_sprints ps ON pss.sprint_id = ps.id WHERE ps.project_id = ?1 AND pi.checked = 1), \
            (SELECT count(*) FROM project_items pi JOIN project_sprint_sections pss ON pi.section_id = pss.id JOIN project_sprints ps ON pss.sprint_id = ps.id WHERE ps.project_id = ?1)",
        [project_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ).unwrap_or((0, 0));

    Ok(
        json!({ "project": project, "sprints": sprints, "progress": { "checked": checked, "total": total } }),
    )
}

fn handle_get_active_sprint(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = params
        .get("project_id")
        .and_then(|v| v.as_i64())
        .ok_or("project_id required")?;

    let sprint_id: Option<i64> = conn
        .query_row(
            "SELECT id FROM project_sprints WHERE project_id = ?1 AND status = 'active' LIMIT 1",
            [project_id],
            |row| row.get(0),
        )
        .ok();

    let sprint =
        match sprint_id {
            Some(sid) => {
                let mut sec_stmt = conn
                    .prepare(
                        "SELECT pss.id, pss.name, pss.description, pss.is_custom,
                        pi.id as item_id, pi.title, pi.description, pi.checked, pi.notes
                 FROM project_sprint_sections pss
                 LEFT JOIN project_items pi ON pi.section_id = pss.id
                 WHERE pss.sprint_id = ?1
                 ORDER BY pss.sort_order, pi.sort_order",
                    )
                    .map_err(|e| e.to_string())?;

                let mut sections_map: std::collections::HashMap<i64, Value> =
                    std::collections::HashMap::new();
                let mut section_order: Vec<i64> = Vec::new();

                let rows: Vec<(
                    i64,
                    String,
                    String,
                    bool,
                    Option<i64>,
                    Option<String>,
                    Option<String>,
                    Option<i64>,
                    Option<String>,
                )> = sec_stmt
                    .query_map([sid], |row| {
                        Ok((
                            row.get(0)?,
                            row.get(1)?,
                            row.get(2)?,
                            row.get(3)?,
                            row.get(4)?,
                            row.get(5)?,
                            row.get(6)?,
                            row.get(7)?,
                            row.get(8)?,
                        ))
                    })
                    .map_err(|e| e.to_string())?
                    .filter_map(|r| r.ok())
                    .collect();

                for (
                    sec_id,
                    sec_name,
                    sec_desc,
                    sec_custom,
                    item_id,
                    item_title,
                    item_desc,
                    item_checked,
                    item_notes,
                ) in rows
                {
                    if !sections_map.contains_key(&sec_id) {
                        sections_map.insert(
                            sec_id,
                            json!({
                                "id": sec_id,
                                "name": sec_name,
                                "description": sec_desc,
                                "is_custom": sec_custom,
                                "items": [],
                            }),
                        );
                        section_order.push(sec_id);
                    }
                    if let Some(iid) = item_id {
                        let items = sections_map
                            .get_mut(&sec_id)
                            .unwrap()
                            .get_mut("items")
                            .unwrap()
                            .as_array_mut()
                            .unwrap();
                        items.push(json!({
                            "id": iid,
                            "title": item_title.unwrap_or_default(),
                            "description": item_desc.unwrap_or_default(),
                            "checked": item_checked.unwrap_or(0) != 0,
                            "notes": item_notes.unwrap_or_default(),
                        }));
                    }
                }

                let sections: Vec<Value> = section_order
                    .iter()
                    .filter_map(|id| sections_map.get(id).cloned())
                    .collect();

                let sprint_info: Value = conn.query_row(
                "SELECT id, name, description, sort_order FROM project_sprints WHERE id = ?1",
                [sid],
                |row| Ok(json!({
                    "id": row.get::<_, i64>(0)?,
                    "name": row.get::<_, String>(1)?,
                    "description": row.get::<_, String>(2)?,
                    "sort_order": row.get::<_, i64>(3)?,
                }))
            ).unwrap_or(json!(null));

                json!({ "sprint": sprint_info, "sections": sections })
            }
            None => json!({ "sprint": null, "sections": [] }),
        };

    Ok(sprint)
}

fn handle_update_item(conn: &Connection, params: &Value) -> Result<Value, String> {
    let item_id = params
        .get("item_id")
        .and_then(|v| v.as_i64())
        .ok_or("item_id required")?;

    if let Some(checked) = params.get("checked").and_then(|v| v.as_bool()) {
        conn.execute(
            "UPDATE project_items SET checked = ?1 WHERE id = ?2",
            (if checked { 1i64 } else { 0i64 }, item_id),
        )
        .map_err(|e| e.to_string())?;
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

fn handle_set_sprint_status(conn: &Connection, params: &Value) -> Result<Value, String> {
    let sprint_id = params
        .get("sprint_id")
        .and_then(|v| v.as_i64())
        .ok_or("sprint_id required")?;
    let status = params
        .get("status")
        .and_then(|v| v.as_str())
        .ok_or("status required")?;

    conn.execute(
        "UPDATE project_sprints SET status = ?1 WHERE id = ?2",
        (status, sprint_id),
    )
    .map_err(|e| e.to_string())?;

    Ok(json!({ "ok": true, "sprint_id": sprint_id, "status": status }))
}

fn handle_complete_sprint(conn: &Connection, params: &Value) -> Result<Value, String> {
    let sprint_id = params
        .get("sprint_id")
        .and_then(|v| v.as_i64())
        .ok_or("sprint_id required")?;
    let project_id = params
        .get("project_id")
        .and_then(|v| v.as_i64())
        .ok_or("project_id required")?;

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
    let next: Option<i64> = conn.query_row(
        "SELECT id FROM project_sprints WHERE project_id = ?1 AND status = 'pending' ORDER BY sort_order LIMIT 1",
        [project_id],
        |row| row.get(0),
    ).ok();

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
    let project_id = params
        .get("project_id")
        .and_then(|v| v.as_i64())
        .ok_or("project_id required")?;

    let (checked, total): (i64, i64) = conn.query_row(
        "SELECT \
            (SELECT count(*) FROM project_items pi JOIN project_sprint_sections pss ON pi.section_id = pss.id JOIN project_sprints ps ON pss.sprint_id = ps.id WHERE ps.project_id = ?1 AND pi.checked = 1), \
            (SELECT count(*) FROM project_items pi JOIN project_sprint_sections pss ON pi.section_id = pss.id JOIN project_sprints ps ON pss.sprint_id = ps.id WHERE ps.project_id = ?1)",
        [project_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ).unwrap_or((0, 0));

    let pct = if total > 0 {
        (checked as f64 / total as f64 * 100.0).round()
    } else {
        0.0
    };

    Ok(json!({ "checked": checked, "total": total, "percentage": pct as i64 }))
}

fn handle_log_error(conn: &Connection, params: &Value) -> Result<Value, String> {
    let project_id = params
        .get("project_id")
        .and_then(|v| v.as_i64())
        .ok_or("project_id required")?;
    let sprint_id = params
        .get("sprint_id")
        .and_then(|v| v.as_i64())
        .ok_or("sprint_id required")?;
    let error = params
        .get("error")
        .and_then(|v| v.as_str())
        .ok_or("error required")?;
    let agent_id = get_agent_id();

    // Find or create "Agent Errors" section in this sprint
    let section_id: Option<i64> = conn.query_row(
        "SELECT pss.id FROM project_sprint_sections pss WHERE pss.sprint_id = ?1 AND pss.name = 'Agent Errors' LIMIT 1",
        [sprint_id],
        |row| row.get(0),
    ).ok();

    let section_id = match section_id {
        Some(id) => id,
        None => {
            let max_order: i64 = conn.query_row(
                "SELECT COALESCE(MAX(sort_order), -1) FROM project_sprint_sections WHERE sprint_id = ?1",
                [sprint_id],
                |row| row.get(0),
            ).map_err(|e| e.to_string())?;

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
    Ok(json!({ "ok": true, "item_id": item_id, "section_id": section_id }))
}

// ─── Main Loop ───

fn main() {
    let db_path = get_db_path();
    if !db_path.exists() {
        eprintln!("Database not found at {:?}", db_path);
        eprintln!("Make sure the DevStar app has been run at least once.");
        std::process::exit(1);
    }

    let agent_id = get_agent_id();
    eprintln!("DevStar MCP Server started (agent: {})", agent_id);

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

        let conn = match Connection::open(&db_path) {
            Ok(c) => c,
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
        };

        let result = match request.method.as_str() {
            "initialize" => Ok(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "devstar-mcp", "version": "0.1.0" }
            })),
            "tools/list" => Ok(json!({ "tools": tool_definitions() })),
            "tools/call" => {
                let params = request.params.as_ref().unwrap();
                let tool = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let tool_params = params.get("arguments").cloned().unwrap_or(json!({}));

                match tool {
                    "list_templates" => handle_list_templates(&conn),
                    "get_template" => handle_get_template(&conn, &tool_params),
                    "list_shared_sections" => handle_list_shared_sections(&conn),
                    "list_shared_sprints" => handle_list_shared_sprints(&conn),
                    "create_project" => handle_create_project(&conn, &tool_params),
                    "get_project_context" => handle_get_project_context(&conn, &tool_params),
                    "get_project" => handle_get_project(&conn, &tool_params),
                    "get_active_sprint" => handle_get_active_sprint(&conn, &tool_params),
                    "update_item" => handle_update_item(&conn, &tool_params),
                    "add_item" => handle_add_item(&conn, &tool_params),
                    "set_sprint_status" => handle_set_sprint_status(&conn, &tool_params),
                    "complete_sprint" => handle_complete_sprint(&conn, &tool_params),
                    "get_progress" => handle_get_progress(&conn, &tool_params),
                    "log_error" => handle_log_error(&conn, &tool_params),
                    _ => Err(format!("Unknown tool: {}", tool)),
                }
            }
            _ => Ok(null_value()),
        };

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
