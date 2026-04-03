pub mod api_backend;
pub mod cloud_infra;
pub mod data_science_ai;
pub mod desktop_app;
pub mod embedded_iot;
pub mod enterprise_systems;
pub mod game_dev;
pub mod mobile_app;
pub mod security_software;
pub mod systems_programming;
pub mod tools_libraries;
pub mod web_dev;

use rusqlite::Connection;
use super::super::types::AppError;

pub fn add_template(
    conn: &Connection,
    name: &str,
    description: &str,
    color: &str,
) -> Result<i64, AppError> {
    conn.execute(
        "INSERT INTO templates (name, description, color) VALUES (?1, ?2, ?3)",
        (name, description, color),
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn add_template_sprint(
    conn: &Connection,
    template_id: i64,
    name: &str,
    description: &str,
    sort_order: i64,
) -> Result<i64, AppError> {
    conn.execute(
        "INSERT INTO template_sprints (template_id, name, description, sort_order, is_custom) VALUES (?1, ?2, ?3, ?4, 0)",
        (template_id, name, description, sort_order),
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn add_template_sprint_sections(
    conn: &Connection,
    sprint_id: i64,
    section_ids: &[i64],
) -> Result<(), AppError> {
    for (i, section_id) in section_ids.iter().enumerate() {
        conn.execute(
            "INSERT INTO template_sprint_sections (sprint_id, section_id, sort_order, is_linked) VALUES (?1, ?2, ?3, 1)",
            (sprint_id, section_id, i as i64),
        )?;
    }
    Ok(())
}

pub fn add_custom_sprint_section(
    conn: &Connection,
    sprint_id: i64,
    name: &str,
    description: &str,
    color: &str,
    items: &[(&str, &str)],
) -> Result<i64, AppError> {
    let section_id = super::add_shared_section(conn, name, description, color, items)?;
    let max_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order), -1) FROM template_sprint_sections WHERE sprint_id = ?1",
        [sprint_id],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO template_sprint_sections (sprint_id, section_id, sort_order, is_linked) VALUES (?1, ?2, ?3, 1)",
        (sprint_id, section_id, max_order + 1),
    )?;
    Ok(section_id)
}
