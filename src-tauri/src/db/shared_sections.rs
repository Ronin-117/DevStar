use super::types::*;
use rusqlite::Result as SqliteResult;

use rusqlite::Connection;

pub fn list(conn: &Connection) -> Result<Vec<SharedSection>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, color, created_at, updated_at FROM shared_sections ORDER BY name",
    )?;
    let sections = stmt
        .query_map([], |row| {
            Ok(SharedSection {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(sections)
}

pub fn get_with_items(conn: &Connection, section_id: i64) -> Result<SectionWithItems, AppError> {
    let section = get_internal(conn, section_id)
        .ok_or_else(|| AppError::NotFound(format!("shared section {section_id}")))?;
    let items = get_items(conn, section_id)?;
    Ok(SectionWithItems { section, items })
}

pub fn create(conn: &Connection, input: SharedSectionInput) -> Result<SharedSection, AppError> {
    conn.execute(
        "INSERT INTO shared_sections (name, description, color) VALUES (?1, ?2, ?3)",
        (
            &input.name,
            input.description.as_deref().unwrap_or(""),
            input.color.as_deref().unwrap_or("#6b7280"),
        ),
    )?;
    let id = conn.last_insert_rowid();
    get_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("shared section {id}")))
}

pub fn update(
    conn: &Connection,
    id: i64,
    name: Option<String>,
    description: Option<String>,
    color: Option<String>,
) -> Result<SharedSection, AppError> {
    if let Some(ref n) = name {
        conn.execute(
            "UPDATE shared_sections SET name = ?1, updated_at = datetime('now') WHERE id = ?2",
            (n, id),
        )?;
    }
    if let Some(ref d) = description {
        conn.execute("UPDATE shared_sections SET description = ?1, updated_at = datetime('now') WHERE id = ?2", (d, id))?;
    }
    if let Some(ref c) = color {
        conn.execute(
            "UPDATE shared_sections SET color = ?1, updated_at = datetime('now') WHERE id = ?2",
            (c, id),
        )?;
    }
    get_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("shared section {id}")))
}

pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM shared_sections WHERE id = ?1", [id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("shared section {id}")));
    }
    Ok(())
}

pub fn add_item(
    conn: &Connection,
    input: SharedSectionItemInput,
) -> Result<SharedSectionItem, AppError> {
    let max_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order), -1) FROM shared_section_items WHERE section_id = ?1",
        [input.section_id],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO shared_section_items (section_id, title, description, sort_order) VALUES (?1, ?2, ?3, ?4)",
        (input.section_id, &input.title, input.description.as_deref().unwrap_or(""), max_order + 1),
    )?;
    let id = conn.last_insert_rowid();
    get_item_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("item {id}")))
}

pub fn update_item(
    conn: &Connection,
    id: i64,
    title: Option<String>,
    description: Option<String>,
) -> Result<SharedSectionItem, AppError> {
    if let Some(ref t) = title {
        conn.execute(
            "UPDATE shared_section_items SET title = ?1 WHERE id = ?2",
            (t, id),
        )?;
    }
    if let Some(ref d) = description {
        conn.execute(
            "UPDATE shared_section_items SET description = ?1 WHERE id = ?2",
            (d, id),
        )?;
    }
    get_item_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("item {id}")))
}

pub fn delete_item(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM shared_section_items WHERE id = ?1", [id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("item {id}")));
    }
    Ok(())
}

// Internal helpers
pub fn get_internal(conn: &Connection, id: i64) -> Option<SharedSection> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, color, created_at, updated_at FROM shared_sections WHERE id = ?1",
    ).ok()?;
    stmt.query_row([id], |row| {
        Ok(SharedSection {
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

pub fn get_items(conn: &Connection, section_id: i64) -> Result<Vec<SharedSectionItem>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, section_id, title, description, sort_order FROM shared_section_items WHERE section_id = ?1 ORDER BY sort_order",
    )?;
    let items = stmt
        .query_map([section_id], |row| {
            Ok(SharedSectionItem {
                id: row.get(0)?,
                section_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                sort_order: row.get(4)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(items)
}

fn get_item_internal(conn: &Connection, id: i64) -> Option<SharedSectionItem> {
    let mut stmt = conn.prepare(
        "SELECT id, section_id, title, description, sort_order FROM shared_section_items WHERE id = ?1",
    ).ok()?;
    stmt.query_row([id], |row| {
        Ok(SharedSectionItem {
            id: row.get(0)?,
            section_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            sort_order: row.get(4)?,
        })
    })
    .ok()
}
