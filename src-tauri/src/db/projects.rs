use super::types::*;
use rusqlite::Result as SqliteResult;

use rusqlite::Connection;

pub fn list(conn: &Connection) -> Result<Vec<Project>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, template_id, color, created_at, updated_at FROM projects ORDER BY updated_at DESC",
    )?;
    let projects = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                template_id: row.get(3)?,
                color: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(projects)
}

pub fn create_from_template(conn: &Connection, input: ProjectInput) -> Result<Project, AppError> {
    let template_exists: i64 = conn.query_row(
        "SELECT count(*) FROM templates WHERE id = ?1",
        [input.template_id],
        |row| row.get(0),
    )?;
    if template_exists == 0 {
        return Err(AppError::NotFound(format!(
            "template {}",
            input.template_id
        )));
    }

    conn.execute(
        "INSERT INTO projects (name, description, template_id, color) VALUES (?1, ?2, ?3, ?4)",
        (
            &input.name,
            input.description.as_deref().unwrap_or(""),
            input.template_id,
            input.color.as_deref().unwrap_or("#6366f1"),
        ),
    )?;
    let project_id = conn.last_insert_rowid();

    // Copy template sprints into project
    let template_sprints = super::template_sprints::list(conn, input.template_id)?;
    for (i, ts) in template_sprints.iter().enumerate() {
        conn.execute(
            "INSERT INTO project_sprints (project_id, name, description, status, sort_order, is_custom) VALUES (?1, ?2, ?3, ?4, ?5, 0)",
            (project_id, &ts.name, &ts.description, if i == 0 { "active" } else { "pending" }, i as i64),
        )?;
        let project_sprint_id = conn.last_insert_rowid();

        // Copy sections from template sprint
        let ts_sections = super::template_sprints::get_sections(conn, ts.id)?;
        for tss in &ts_sections {
            let section = super::shared_sections::get_internal(conn, tss.section_id)
                .ok_or_else(|| AppError::NotFound(format!("shared section {}", tss.section_id)))?;

            conn.execute(
                "INSERT INTO project_sprint_sections (sprint_id, name, description, sort_order, is_custom, linked_from_section_id) VALUES (?1, ?2, ?3, ?4, 0, ?5)",
                (project_sprint_id, &section.name, &section.description, tss.sort_order, tss.section_id),
            )?;
            let project_section_id = conn.last_insert_rowid();

            // Copy items from shared section
            let items = super::shared_sections::get_items(conn, tss.section_id)?;
            for item in &items {
                conn.execute(
                    "INSERT INTO project_items (section_id, title, description, checked, notes, sort_order, is_custom) VALUES (?1, ?2, ?3, 0, '', ?4, 0)",
                    (project_section_id, &item.title, &item.description, item.sort_order),
                )?;
            }
        }
    }

    get_internal(conn, project_id)
        .ok_or_else(|| AppError::NotFound(format!("project {project_id}")))
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("project {id}")));
    }
    Ok(())
}

fn get_internal(conn: &Connection, id: i64) -> Option<Project> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, template_id, color, created_at, updated_at FROM projects WHERE id = ?1",
    ).ok()?;
    stmt.query_row([id], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            template_id: row.get(3)?,
            color: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })
    .ok()
}
