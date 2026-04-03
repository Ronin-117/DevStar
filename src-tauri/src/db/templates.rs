use super::types::*;
use rusqlite::Result as SqliteResult;

use rusqlite::Connection;

pub fn list(conn: &Connection) -> Result<Vec<Template>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, color, created_at, updated_at FROM templates ORDER BY name",
    )?;
    let templates = stmt
        .query_map([], |row| {
            Ok(Template {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(templates)
}

pub fn create(conn: &Connection, input: TemplateInput) -> Result<Template, AppError> {
    conn.execute(
        "INSERT INTO templates (name, description, color) VALUES (?1, ?2, ?3)",
        (
            &input.name,
            input.description.as_deref().unwrap_or(""),
            input.color.as_deref().unwrap_or("#6366f1"),
        ),
    )?;
    let id = conn.last_insert_rowid();
    get_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("template {id}")))
}

pub fn update(
    conn: &Connection,
    id: i64,
    name: Option<String>,
    description: Option<String>,
    color: Option<String>,
) -> Result<Template, AppError> {
    if let Some(ref n) = name {
        conn.execute(
            "UPDATE templates SET name = ?1, updated_at = datetime('now') WHERE id = ?2",
            (n, id),
        )?;
    }
    if let Some(ref d) = description {
        conn.execute(
            "UPDATE templates SET description = ?1, updated_at = datetime('now') WHERE id = ?2",
            (d, id),
        )?;
    }
    if let Some(ref c) = color {
        conn.execute(
            "UPDATE templates SET color = ?1, updated_at = datetime('now') WHERE id = ?2",
            (c, id),
        )?;
    }
    get_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("template {id}")))
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM templates WHERE id = ?1", [id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("template {id}")));
    }
    Ok(())
}

fn get_internal(conn: &Connection, id: i64) -> Option<Template> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, color, created_at, updated_at FROM templates WHERE id = ?1",
    ).ok()?;
    stmt.query_row([id], |row| {
        Ok(Template {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            color: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })
    .ok()
}
