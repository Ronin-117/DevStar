use super::types::*;
use rusqlite::Result as SqliteResult;

use rusqlite::Connection;

pub fn list(conn: &Connection, template_id: i64) -> Result<Vec<TemplateSprint>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, template_id, name, description, sort_order, is_custom FROM template_sprints WHERE template_id = ?1 ORDER BY sort_order",
    )?;
    let sprints = stmt
        .query_map([template_id], |row| {
            Ok(TemplateSprint {
                id: row.get(0)?,
                template_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                sort_order: row.get(4)?,
                is_custom: row.get::<_, i64>(5)? != 0,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(sprints)
}

pub fn get_with_sections(
    conn: &Connection,
    sprint_id: i64,
) -> Result<TemplateSprintWithSections, AppError> {
    let sprint = get_internal(conn, sprint_id)
        .ok_or_else(|| AppError::NotFound(format!("template sprint {sprint_id}")))?;
    let sections = get_sections(conn, sprint_id)?;
    Ok(TemplateSprintWithSections { sprint, sections })
}

pub fn add(
    conn: &Connection,
    template_id: i64,
    name: String,
    description: String,
) -> Result<TemplateSprint, AppError> {
    let max_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order), -1) FROM template_sprints WHERE template_id = ?1",
        [template_id],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO template_sprints (template_id, name, description, sort_order, is_custom) VALUES (?1, ?2, ?3, ?4, 1)",
        (template_id, &name, &description, max_order + 1),
    )?;
    let id = conn.last_insert_rowid();
    get_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("template sprint {id}")))
}

pub fn update(
    conn: &Connection,
    id: i64,
    name: Option<String>,
    description: Option<String>,
) -> Result<TemplateSprint, AppError> {
    if let Some(ref n) = name {
        conn.execute(
            "UPDATE template_sprints SET name = ?1 WHERE id = ?2",
            (n, id),
        )?;
    }
    if let Some(ref d) = description {
        conn.execute(
            "UPDATE template_sprints SET description = ?1 WHERE id = ?2",
            (d, id),
        )?;
    }
    get_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("template sprint {id}")))
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM template_sprints WHERE id = ?1", [id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("template sprint {id}")));
    }
    Ok(())
}

pub fn add_section(
    conn: &Connection,
    sprint_id: i64,
    section_id: i64,
    is_linked: bool,
) -> Result<TemplateSprintSection, AppError> {
    let max_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order), -1) FROM template_sprint_sections WHERE sprint_id = ?1",
        [sprint_id],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO template_sprint_sections (sprint_id, section_id, sort_order, is_linked) VALUES (?1, ?2, ?3, ?4)",
        (sprint_id, section_id, max_order + 1, is_linked as i64),
    )?;
    let id = conn.last_insert_rowid();
    get_section_internal(conn, id)
        .ok_or_else(|| AppError::NotFound(format!("template sprint section {id}")))
}

pub fn delete_section(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM template_sprint_sections WHERE id = ?1", [id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("template sprint section {id}")));
    }
    Ok(())
}

// Internal helpers
fn get_internal(conn: &Connection, id: i64) -> Option<TemplateSprint> {
    let mut stmt = conn.prepare(
        "SELECT id, template_id, name, description, sort_order, is_custom FROM template_sprints WHERE id = ?1",
    ).ok()?;
    stmt.query_row([id], |row| {
        Ok(TemplateSprint {
            id: row.get(0)?,
            template_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            sort_order: row.get(4)?,
            is_custom: row.get::<_, i64>(5)? != 0,
        })
    })
    .ok()
}

pub fn get_sections(
    conn: &Connection,
    sprint_id: i64,
) -> Result<Vec<TemplateSprintSection>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, sprint_id, section_id, sort_order, is_linked FROM template_sprint_sections WHERE sprint_id = ?1 ORDER BY sort_order",
    )?;
    let sections = stmt
        .query_map([sprint_id], |row| {
            Ok(TemplateSprintSection {
                id: row.get(0)?,
                sprint_id: row.get(1)?,
                section_id: row.get(2)?,
                sort_order: row.get(3)?,
                is_linked: row.get::<_, i64>(4)? != 0,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(sections)
}

fn get_section_internal(conn: &Connection, id: i64) -> Option<TemplateSprintSection> {
    let mut stmt = conn.prepare(
        "SELECT id, sprint_id, section_id, sort_order, is_linked FROM template_sprint_sections WHERE id = ?1",
    ).ok()?;
    stmt.query_row([id], |row| {
        Ok(TemplateSprintSection {
            id: row.get(0)?,
            sprint_id: row.get(1)?,
            section_id: row.get(2)?,
            sort_order: row.get(3)?,
            is_linked: row.get::<_, i64>(4)? != 0,
        })
    })
    .ok()
}
