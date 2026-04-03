use super::types::*;
use rusqlite::Result as SqliteResult;

use rusqlite::Connection;

pub fn list(conn: &Connection) -> Result<Vec<SharedSprint>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, sort_order, created_at, updated_at FROM shared_sprints ORDER BY sort_order",
    )?;
    let sprints = stmt
        .query_map([], |row| {
            Ok(SharedSprint {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                sort_order: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(sprints)
}

pub fn get_with_sections(
    conn: &Connection,
    sprint_id: i64,
) -> Result<SharedSprintWithSections, AppError> {
    let sprint = get_internal(conn, sprint_id)
        .ok_or_else(|| AppError::NotFound(format!("shared sprint {sprint_id}")))?;
    let sections = get_sections(conn, sprint_id)?;
    Ok(SharedSprintWithSections { sprint, sections })
}

pub fn create(conn: &Connection, input: SharedSprintInput) -> Result<SharedSprint, AppError> {
    let max_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order), -1) FROM shared_sprints",
        [],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO shared_sprints (name, description, sort_order) VALUES (?1, ?2, ?3)",
        (
            &input.name,
            input.description.as_deref().unwrap_or(""),
            max_order + 1,
        ),
    )?;
    let id = conn.last_insert_rowid();
    get_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("shared sprint {id}")))
}

pub fn update(
    conn: &Connection,
    id: i64,
    name: Option<String>,
    description: Option<String>,
) -> Result<SharedSprint, AppError> {
    if let Some(ref n) = name {
        conn.execute(
            "UPDATE shared_sprints SET name = ?1, updated_at = datetime('now') WHERE id = ?2",
            (n, id),
        )?;
    }
    if let Some(ref d) = description {
        conn.execute("UPDATE shared_sprints SET description = ?1, updated_at = datetime('now') WHERE id = ?2", (d, id))?;
    }
    get_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("shared sprint {id}")))
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM shared_sprints WHERE id = ?1", [id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("shared sprint {id}")));
    }
    Ok(())
}

pub fn add_section(
    conn: &Connection,
    input: SharedSprintSectionInput,
) -> Result<SharedSprintSection, AppError> {
    let max_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order), -1) FROM shared_sprint_sections WHERE sprint_id = ?1",
        [input.sprint_id],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO shared_sprint_sections (sprint_id, section_id, sort_order, is_linked) VALUES (?1, ?2, ?3, ?4)",
        (input.sprint_id, input.section_id, max_order + 1, input.is_linked as i64),
    )?;
    let id = conn.last_insert_rowid();
    get_section_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("sprint section {id}")))
}

pub fn delete_section(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM shared_sprint_sections WHERE id = ?1", [id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("sprint section {id}")));
    }
    Ok(())
}

// Internal helpers
fn get_internal(conn: &Connection, id: i64) -> Option<SharedSprint> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, sort_order, created_at, updated_at FROM shared_sprints WHERE id = ?1",
    ).ok()?;
    stmt.query_row([id], |row| {
        Ok(SharedSprint {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            sort_order: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })
    .ok()
}

fn get_sections(conn: &Connection, sprint_id: i64) -> Result<Vec<SharedSprintSection>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, sprint_id, section_id, sort_order, is_linked FROM shared_sprint_sections WHERE sprint_id = ?1 ORDER BY sort_order",
    )?;
    let sections = stmt
        .query_map([sprint_id], |row| {
            Ok(SharedSprintSection {
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

fn get_section_internal(conn: &Connection, id: i64) -> Option<SharedSprintSection> {
    let mut stmt = conn.prepare(
        "SELECT id, sprint_id, section_id, sort_order, is_linked FROM shared_sprint_sections WHERE id = ?1",
    ).ok()?;
    stmt.query_row([id], |row| {
        Ok(SharedSprintSection {
            id: row.get(0)?,
            sprint_id: row.get(1)?,
            section_id: row.get(2)?,
            sort_order: row.get(3)?,
            is_linked: row.get::<_, i64>(4)? != 0,
        })
    })
    .ok()
}
