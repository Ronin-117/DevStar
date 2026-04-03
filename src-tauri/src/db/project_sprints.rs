use super::types::*;
use rusqlite::Result as SqliteResult;

use rusqlite::Connection;

pub fn list_with_sections(
    conn: &Connection,
    project_id: i64,
) -> Result<Vec<ProjectSprintWithSections>, AppError> {
    let mut sprint_stmt = conn.prepare(
        "SELECT id, project_id, name, description, status, sort_order, is_custom FROM project_sprints WHERE project_id = ?1 ORDER BY sort_order",
    )?;
    let sprints = sprint_stmt
        .query_map([project_id], |row| {
            Ok(ProjectSprint {
                id: row.get(0)?,
                project_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                status: row.get(4)?,
                sort_order: row.get(5)?,
                is_custom: row.get::<_, i64>(6)? != 0,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;

    let mut result = Vec::new();
    for sprint in sprints {
        let sections = get_sections(conn, sprint.id)?;
        let mut sections_with_items = Vec::new();
        for section in sections {
            let items = get_items(conn, section.id)?;
            sections_with_items.push(ProjectSprintSectionWithItems { section, items });
        }
        result.push(ProjectSprintWithSections {
            sprint,
            sections: sections_with_items,
        });
    }
    Ok(result)
}

pub fn set_status(conn: &Connection, sprint_id: i64, status: String) -> Result<(), AppError> {
    let affected = conn.execute(
        "UPDATE project_sprints SET status = ?1 WHERE id = ?2",
        (&status, sprint_id),
    )?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("project sprint {sprint_id}")));
    }
    Ok(())
}

pub fn get_active(conn: &Connection, project_id: i64) -> Result<Option<ProjectSprint>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, name, description, status, sort_order, is_custom FROM project_sprints WHERE project_id = ?1 AND status = 'active'",
    )?;
    let sprint = stmt
        .query_row([project_id], |row| {
            Ok(ProjectSprint {
                id: row.get(0)?,
                project_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                status: row.get(4)?,
                sort_order: row.get(5)?,
                is_custom: row.get::<_, i64>(6)? != 0,
            })
        })
        .ok();
    Ok(sprint)
}

pub fn get_progress(conn: &Connection, project_id: i64) -> Result<(i64, i64), AppError> {
    let total: i64 = conn.query_row(
        "SELECT count(*) FROM project_items pi JOIN project_sprint_sections ps ON pi.section_id = ps.id JOIN project_sprints prs ON ps.sprint_id = prs.id WHERE prs.project_id = ?1",
        [project_id],
        |row| row.get(0),
    )?;
    let checked: i64 = conn.query_row(
        "SELECT count(*) FROM project_items pi JOIN project_sprint_sections ps ON pi.section_id = ps.id JOIN project_sprints prs ON ps.sprint_id = prs.id WHERE prs.project_id = ?1 AND pi.checked = 1",
        [project_id],
        |row| row.get(0),
    )?;
    Ok((checked, total))
}

pub fn add_section(
    conn: &Connection,
    input: ProjectSectionInput,
) -> Result<ProjectSprintSection, AppError> {
    let max_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order), -1) FROM project_sprint_sections WHERE sprint_id = ?1",
        [input.sprint_id],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO project_sprint_sections (sprint_id, name, description, sort_order, is_custom, linked_from_section_id) VALUES (?1, ?2, ?3, ?4, 1, ?5)",
        (input.sprint_id, &input.name, input.description.as_deref().unwrap_or(""), max_order + 1, input.linked_from_section_id),
    )?;
    let id = conn.last_insert_rowid();
    get_section_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("section {id}")))
}

pub fn delete_section(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM project_sprint_sections WHERE id = ?1", [id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("section {id}")));
    }
    Ok(())
}

pub fn toggle_item(conn: &Connection, id: i64) -> Result<ProjectItem, AppError> {
    let current: i64 = conn.query_row(
        "SELECT checked FROM project_items WHERE id = ?1",
        [id],
        |row| row.get(0),
    )?;
    conn.execute(
        "UPDATE project_items SET checked = ?1 WHERE id = ?2",
        ((1 - current) as i64, id),
    )?;
    get_item_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("item {id}")))
}

pub fn update_item(conn: &Connection, update: ProjectItemUpdate) -> Result<ProjectItem, AppError> {
    if let Some(checked) = update.checked {
        conn.execute(
            "UPDATE project_items SET checked = ?1 WHERE id = ?2",
            (checked as i64, update.id),
        )?;
    }
    if let Some(ref notes) = update.notes {
        conn.execute(
            "UPDATE project_items SET notes = ?1 WHERE id = ?2",
            (notes, update.id),
        )?;
    }
    get_item_internal(conn, update.id)
        .ok_or_else(|| AppError::NotFound(format!("item {}", update.id)))
}

pub fn add_item(conn: &Connection, input: ProjectItemInput) -> Result<ProjectItem, AppError> {
    let max_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order), -1) FROM project_items WHERE section_id = ?1",
        [input.section_id],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO project_items (section_id, title, description, checked, notes, sort_order, is_custom) VALUES (?1, ?2, ?3, 0, '', ?4, 1)",
        (input.section_id, &input.title, input.description.as_deref().unwrap_or(""), max_order + 1),
    )?;
    let id = conn.last_insert_rowid();
    get_item_internal(conn, id).ok_or_else(|| AppError::NotFound(format!("item {id}")))
}

pub fn delete_item(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected = conn.execute("DELETE FROM project_items WHERE id = ?1", [id])?;
    if affected == 0 {
        return Err(AppError::NotFound(format!("item {id}")));
    }
    Ok(())
}

/// Check if all items in the active sprint are done. If so, mark it done
/// and advance to the next sprint (mark it active).
pub fn check_and_advance_sprint(
    conn: &Connection,
    project_id: i64,
) -> Result<Option<ProjectSprint>, AppError> {
    // Get the active sprint
    let active = get_active(conn, project_id)?;
    let active = match active {
        Some(s) => s,
        None => return Ok(None),
    };

    // Count total and checked items in this sprint
    let (total, checked): (i64, i64) = conn.query_row(
        "SELECT \
           (SELECT count(*) FROM project_items pi JOIN project_sprint_sections ps ON pi.section_id = ps.id WHERE ps.sprint_id = ?1), \
           (SELECT count(*) FROM project_items pi JOIN project_sprint_sections ps ON pi.section_id = ps.id WHERE ps.sprint_id = ?1 AND pi.checked = 1)",
        [active.id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    // If not all items are checked, nothing to do
    if total == 0 || checked < total {
        return Ok(None);
    }

    // Mark current sprint as done
    set_status(conn, active.id, "done".to_string())?;

    // Find the next pending sprint
    let mut next_stmt = conn.prepare(
        "SELECT id, project_id, name, description, status, sort_order, is_custom FROM project_sprints WHERE project_id = ?1 AND status = 'pending' ORDER BY sort_order LIMIT 1",
    )?;
    let next_sprint = next_stmt
        .query_row([project_id], |row| {
            Ok(ProjectSprint {
                id: row.get(0)?,
                project_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                status: row.get(4)?,
                sort_order: row.get(5)?,
                is_custom: row.get::<_, i64>(6)? != 0,
            })
        })
        .ok();

    if let Some(next) = next_sprint {
        set_status(conn, next.id, "active".to_string())?;
        Ok(Some(next))
    } else {
        Ok(None)
    }
}

// Internal helpers
fn get_sections(conn: &Connection, sprint_id: i64) -> Result<Vec<ProjectSprintSection>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, sprint_id, name, description, sort_order, is_custom, linked_from_section_id FROM project_sprint_sections WHERE sprint_id = ?1 ORDER BY sort_order",
    )?;
    let sections = stmt
        .query_map([sprint_id], |row| {
            Ok(ProjectSprintSection {
                id: row.get(0)?,
                sprint_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                sort_order: row.get(4)?,
                is_custom: row.get::<_, i64>(5)? != 0,
                linked_from_section_id: row.get(6)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(sections)
}

fn get_items(conn: &Connection, section_id: i64) -> Result<Vec<ProjectItem>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, section_id, title, description, checked, notes, sort_order, is_custom FROM project_items WHERE section_id = ?1 ORDER BY sort_order",
    )?;
    let items: SqliteResult<Vec<ProjectItem>> = stmt
        .query_map([section_id], |row| {
            Ok(ProjectItem {
                id: row.get(0)?,
                section_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                checked: row.get::<_, i64>(4)? != 0,
                notes: row.get(5)?,
                sort_order: row.get(6)?,
                is_custom: row.get::<_, i64>(7)? != 0,
            })
        })?
        .collect();
    Ok(items?)
}

fn get_section_internal(conn: &Connection, id: i64) -> Option<ProjectSprintSection> {
    let mut stmt = conn.prepare(
        "SELECT id, sprint_id, name, description, sort_order, is_custom, linked_from_section_id FROM project_sprint_sections WHERE id = ?1",
    ).ok()?;
    stmt.query_row([id], |row| {
        Ok(ProjectSprintSection {
            id: row.get(0)?,
            sprint_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            sort_order: row.get(4)?,
            is_custom: row.get::<_, i64>(5)? != 0,
            linked_from_section_id: row.get(6)?,
        })
    })
    .ok()
}

fn get_item_internal(conn: &Connection, id: i64) -> Option<ProjectItem> {
    let mut stmt = conn.prepare(
        "SELECT id, section_id, title, description, checked, notes, sort_order, is_custom FROM project_items WHERE id = ?1",
    ).ok()?;
    stmt.query_row([id], |row| {
        Ok(ProjectItem {
            id: row.get(0)?,
            section_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            checked: row.get::<_, i64>(4)? != 0,
            notes: row.get(5)?,
            sort_order: row.get(6)?,
            is_custom: row.get::<_, i64>(7)? != 0,
        })
    })
    .ok()
}
