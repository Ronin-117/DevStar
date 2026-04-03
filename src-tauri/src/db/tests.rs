use super::*;
use types::*;

fn test_db() -> Database {
    let db = Database::new(":memory:").expect("Failed to create in-memory database");
    {
        let conn = db.conn.lock().unwrap();
        seed::seed_if_empty(&conn).expect("Failed to seed database");
    }
    db
}

#[test]
fn test_schema_creates_all_tables() {
    let db = Database::new(":memory:").unwrap();
    let conn = db.conn.lock().unwrap();
    let tables = [
        "templates",
        "shared_sections",
        "shared_section_items",
        "shared_sprints",
        "shared_sprint_sections",
        "template_sprints",
        "template_sprint_sections",
        "projects",
        "project_sprints",
        "project_sprint_sections",
        "project_items",
    ];
    for table in &tables {
        let count: i64 = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?",
                [table],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1, "Table {table} should exist");
    }
}

#[test]
fn test_seed_creates_templates() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let templates = templates::list(&conn).unwrap();
    assert_eq!(templates.len(), 3);
    let names: Vec<&str> = templates.iter().map(|t| t.name.as_str()).collect();
    assert!(names.contains(&"Full Stack Web App"));
    assert!(names.contains(&"Mobile App"));
    assert!(names.contains(&"Desktop / Cross-Platform"));
}

#[test]
fn test_seed_creates_shared_sections() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let sections = shared_sections::list(&conn).unwrap();
    // 6 base shared sections + sections created by add_sprint_items for template sprints
    assert!(sections.len() >= 6);
    let names: Vec<&str> = sections.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"Docker Setup"));
    assert!(names.contains(&"Security Basics"));
    assert!(names.contains(&"CI/CD Pipeline"));
    assert!(names.contains(&"Project Planning"));
    assert!(names.contains(&"Testing & QA"));
    assert!(names.contains(&"Documentation"));
}

#[test]
fn test_seed_creates_shared_sprints() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let sprints = shared_sprints::list(&conn).unwrap();
    assert_eq!(sprints.len(), 5);
    let names: Vec<&str> = sprints.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"Planning & Setup"));
    assert!(names.contains(&"Foundation & Architecture"));
    assert!(names.contains(&"Core Development"));
    assert!(names.contains(&"Testing & QA"));
    assert!(names.contains(&"Deployment & Launch"));
}

#[test]
fn test_seed_creates_template_sprints() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let templates = templates::list(&conn).unwrap();
    let web = templates
        .iter()
        .find(|t| t.name == "Full Stack Web App")
        .unwrap();
    let sprints = template_sprints::list(&conn, web.id).unwrap();
    assert!(!sprints.is_empty());
    assert!(sprints[0].name.starts_with("Sprint 1"));
}

#[test]
fn test_shared_section_has_items() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let sections = shared_sections::list(&conn).unwrap();
    let docker = sections.iter().find(|s| s.name == "Docker Setup").unwrap();
    let with_items = shared_sections::get_with_items(&conn, docker.id).unwrap();
    assert_eq!(with_items.items.len(), 5);
    assert_eq!(
        with_items.items[0].title,
        "Dockerfile multi-stage build configured"
    );
}

#[test]
fn test_shared_sprint_has_sections() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let sprints = shared_sprints::list(&conn).unwrap();
    let foundation = sprints
        .iter()
        .find(|s| s.name == "Foundation & Architecture")
        .unwrap();
    let with_sections = shared_sprints::get_with_sections(&conn, foundation.id).unwrap();
    assert!(!with_sections.sections.is_empty());
}

#[test]
fn test_create_project_from_template() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let templates = templates::list(&conn).unwrap();
    let web = templates
        .iter()
        .find(|t| t.name == "Full Stack Web App")
        .unwrap();

    let project = projects::create_from_template(
        &conn,
        ProjectInput {
            name: "My Web App".into(),
            description: Some("A test project".into()),
            template_id: web.id,
            color: None,
        },
    )
    .unwrap();
    assert_eq!(project.name, "My Web App");
    assert_eq!(project.template_id, web.id);

    // Verify sprints were copied
    let sprints = project_sprints::list_with_sections(&conn, project.id).unwrap();
    assert!(!sprints.is_empty());
    assert_eq!(sprints[0].sprint.status, "active");
    assert_eq!(sprints[1].sprint.status, "pending");

    // Verify sections and items were copied
    for sprint in &sprints {
        for section in &sprint.sections {
            assert!(
                !section.items.is_empty(),
                "Section '{}' should have items",
                section.section.name
            );
        }
    }
}

#[test]
fn test_project_is_independent_snapshot() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let templates = templates::list(&conn).unwrap();
    let web = templates
        .iter()
        .find(|t| t.name == "Full Stack Web App")
        .unwrap();

    let project = projects::create_from_template(
        &conn,
        ProjectInput {
            name: "Snapshot Test".into(),
            description: None,
            template_id: web.id,
            color: None,
        },
    )
    .unwrap();

    // Count items in project
    let project_sprints = project_sprints::list_with_sections(&conn, project.id).unwrap();
    let project_item_count: usize = project_sprints
        .iter()
        .flat_map(|s| s.sections.iter())
        .map(|s| s.items.len())
        .sum();

    // Delete the template
    templates::delete(&conn, web.id).unwrap();

    // Project should still have all its items
    let project_sprints = project_sprints::list_with_sections(&conn, project.id).unwrap();
    let project_item_count_after: usize = project_sprints
        .iter()
        .flat_map(|s| s.sections.iter())
        .map(|s| s.items.len())
        .sum();
    assert_eq!(project_item_count, project_item_count_after);
}

#[test]
fn test_toggle_project_item() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let templates = templates::list(&conn).unwrap();
    let web = templates
        .iter()
        .find(|t| t.name == "Full Stack Web App")
        .unwrap();

    let project = projects::create_from_template(
        &conn,
        ProjectInput {
            name: "Toggle Test".into(),
            description: None,
            template_id: web.id,
            color: None,
        },
    )
    .unwrap();

    let sprints = project_sprints::list_with_sections(&conn, project.id).unwrap();
    let first_item = &sprints[0].sections[0].items[0];
    assert!(!first_item.checked);

    let updated = project_sprints::update_item(
        &conn,
        ProjectItemUpdate {
            id: first_item.id,
            checked: Some(true),
            notes: None,
        },
    )
    .unwrap();
    assert!(updated.checked);
}

#[test]
fn test_project_progress() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let templates = templates::list(&conn).unwrap();
    let web = templates
        .iter()
        .find(|t| t.name == "Full Stack Web App")
        .unwrap();

    let project = projects::create_from_template(
        &conn,
        ProjectInput {
            name: "Progress Test".into(),
            description: None,
            template_id: web.id,
            color: None,
        },
    )
    .unwrap();

    let (checked, total) = project_sprints::get_progress(&conn, project.id).unwrap();
    assert_eq!(checked, 0);
    assert!(total > 0);

    // Check all items
    let sprints = project_sprints::list_with_sections(&conn, project.id).unwrap();
    for sprint in &sprints {
        for section in &sprint.sections {
            for item in &section.items {
                project_sprints::update_item(
                    &conn,
                    ProjectItemUpdate {
                        id: item.id,
                        checked: Some(true),
                        notes: None,
                    },
                )
                .unwrap();
            }
        }
    }

    let (checked, total) = project_sprints::get_progress(&conn, project.id).unwrap();
    assert_eq!(checked, total);
}

#[test]
fn test_add_custom_project_section_and_item() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let templates = templates::list(&conn).unwrap();
    let web = templates
        .iter()
        .find(|t| t.name == "Full Stack Web App")
        .unwrap();

    let project = projects::create_from_template(
        &conn,
        ProjectInput {
            name: "Custom Test".into(),
            description: None,
            template_id: web.id,
            color: None,
        },
    )
    .unwrap();

    let sprints = project_sprints::list_with_sections(&conn, project.id).unwrap();
    let first_sprint_id = sprints[0].sprint.id;

    let section = project_sprints::add_section(
        &conn,
        ProjectSectionInput {
            sprint_id: first_sprint_id,
            name: "Custom Section".into(),
            description: None,
            linked_from_section_id: None,
        },
    )
    .unwrap();

    let item = project_sprints::add_item(
        &conn,
        ProjectItemInput {
            section_id: section.id,
            title: "Custom Item".into(),
            description: None,
        },
    )
    .unwrap();

    assert!(item.is_custom);
    assert_eq!(item.title, "Custom Item");
}

#[test]
fn test_set_sprint_status() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let templates = templates::list(&conn).unwrap();
    let web = templates
        .iter()
        .find(|t| t.name == "Full Stack Web App")
        .unwrap();

    let project = projects::create_from_template(
        &conn,
        ProjectInput {
            name: "Status Test".into(),
            description: None,
            template_id: web.id,
            color: None,
        },
    )
    .unwrap();

    let sprints = project_sprints::list_with_sections(&conn, project.id).unwrap();
    let active_sprint = sprints
        .iter()
        .find(|s| s.sprint.status == "active")
        .unwrap();

    // Mark active sprint as done
    project_sprints::set_status(&conn, active_sprint.sprint.id, "done".into()).unwrap();

    // Verify status changed
    let sprints = project_sprints::list_with_sections(&conn, project.id).unwrap();
    let updated = sprints
        .iter()
        .find(|s| s.sprint.id == active_sprint.sprint.id)
        .unwrap();
    assert_eq!(updated.sprint.status, "done");
}

#[test]
fn test_get_active_sprint() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let templates = templates::list(&conn).unwrap();
    let web = templates
        .iter()
        .find(|t| t.name == "Full Stack Web App")
        .unwrap();

    let project = projects::create_from_template(
        &conn,
        ProjectInput {
            name: "Active Test".into(),
            description: None,
            template_id: web.id,
            color: None,
        },
    )
    .unwrap();

    let active = project_sprints::get_active(&conn, project.id).unwrap();
    assert!(active.is_some());
    assert_eq!(active.unwrap().status, "active");
}

#[test]
fn test_seed_is_idempotent() {
    let db = Database::new(":memory:").unwrap();
    {
        let conn = db.conn.lock().unwrap();
        seed::seed_if_empty(&conn).unwrap();
        seed::seed_if_empty(&conn).unwrap();
    }
    let conn = db.conn.lock().unwrap();
    let templates = templates::list(&conn).unwrap();
    assert_eq!(templates.len(), 3);
}

#[test]
fn test_template_sprint_crud() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    let templates = templates::list(&conn).unwrap();
    let web = templates
        .iter()
        .find(|t| t.name == "Full Stack Web App")
        .unwrap();

    let sprint = template_sprints::add(
        &conn,
        web.id,
        "Sprint 10: Custom".into(),
        "Custom sprint".into(),
    )
    .unwrap();
    assert_eq!(sprint.name, "Sprint 10: Custom");
    assert!(sprint.is_custom);

    let updated =
        template_sprints::update(&conn, sprint.id, Some("Sprint 10: Updated".into()), None)
            .unwrap();
    assert_eq!(updated.name, "Sprint 10: Updated");

    template_sprints::delete(&conn, sprint.id).unwrap();
    let sprints = template_sprints::list(&conn, web.id).unwrap();
    assert!(!sprints.iter().any(|s| s.id == sprint.id));
}

#[test]
fn test_shared_section_crud() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();

    let section = shared_sections::create(
        &conn,
        SharedSectionInput {
            name: "Custom Section".into(),
            description: Some("Test".into()),
            color: Some("#ff0000".into()),
        },
    )
    .unwrap();
    assert_eq!(section.name, "Custom Section");

    let updated =
        shared_sections::update(&conn, section.id, Some("Updated".into()), None, None).unwrap();
    assert_eq!(updated.name, "Updated");

    shared_sections::delete(&conn, section.id).unwrap();
    let sections = shared_sections::list(&conn).unwrap();
    assert!(!sections.iter().any(|s| s.id == section.id));
}

#[test]
fn test_shared_sprint_crud() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();

    let sprint = shared_sprints::create(
        &conn,
        SharedSprintInput {
            name: "Custom Sprint".into(),
            description: Some("Test".into()),
        },
    )
    .unwrap();
    assert_eq!(sprint.name, "Custom Sprint");

    let updated = shared_sprints::update(&conn, sprint.id, Some("Updated".into()), None).unwrap();
    assert_eq!(updated.name, "Updated");

    shared_sprints::delete(&conn, sprint.id).unwrap();
    let sprints = shared_sprints::list(&conn).unwrap();
    assert!(!sprints.iter().any(|s| s.id == sprint.id));
}
