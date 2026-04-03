use super::super::types::AppError;
use rusqlite::Connection;

/// Add a shared sprint with references to shared sections. Returns the sprint ID.
pub fn add_shared_sprint(
    conn: &Connection,
    name: &str,
    description: &str,
    sort_order: i64,
    section_ids: &[i64],
) -> Result<i64, AppError> {
    conn.execute(
        "INSERT INTO shared_sprints (name, description, sort_order) VALUES (?1, ?2, ?3)",
        (name, description, sort_order),
    )?;
    let sprint_id = conn.last_insert_rowid();
    for (i, section_id) in section_ids.iter().enumerate() {
        conn.execute(
            "INSERT INTO shared_sprint_sections (sprint_id, section_id, sort_order, is_linked) VALUES (?1, ?2, ?3, 1)",
            (sprint_id, section_id, i as i64),
        )?;
    }
    Ok(sprint_id)
}

/// Seed shared sprints using the section IDs from shared_sections.
/// Returns a map of sprint name -> sprint ID.
pub fn seed(
    conn: &Connection,
    section_map: &std::collections::HashMap<String, i64>,
) -> Result<std::collections::HashMap<String, i64>, AppError> {
    let mut sprints = std::collections::HashMap::new();

    // 1. Planning & Setup
    let planning = section_map.get("planning").copied().unwrap_or(0);
    let quality = section_map.get("quality").copied().unwrap_or(0);
    let id = add_shared_sprint(
        conn,
        "Planning & Setup",
        "Project kickoff, requirements, and development environment",
        0,
        &[planning, quality],
    )?;
    sprints.insert("planning_setup".to_string(), id);

    // 2. Security & Quality
    let security = section_map.get("security").copied().unwrap_or(0);
    let id = add_shared_sprint(
        conn,
        "Security & Quality",
        "Security hardening and code quality enforcement",
        1,
        &[security, quality],
    )?;
    sprints.insert("security_quality".to_string(), id);

    // 3. Testing & QA
    let testing = section_map.get("testing").copied().unwrap_or(0);
    let id = add_shared_sprint(
        conn,
        "Testing & QA",
        "Comprehensive testing strategy and quality assurance",
        2,
        &[testing],
    )?;
    sprints.insert("testing_qa".to_string(), id);

    // 4. CI/CD & Deployment
    let cicd = section_map.get("cicd").copied().unwrap_or(0);
    let docs = section_map.get("docs").copied().unwrap_or(0);
    let id = add_shared_sprint(
        conn,
        "CI/CD & Deployment",
        "Automated pipeline and production deployment",
        3,
        &[cicd, docs],
    )?;
    sprints.insert("cicd_deploy".to_string(), id);

    // 5. Monitoring & Operations
    let monitoring = section_map.get("monitoring").copied().unwrap_or(0);
    let id = add_shared_sprint(
        conn,
        "Monitoring & Operations",
        "Observability, alerting, and incident response",
        4,
        &[monitoring, docs],
    )?;
    sprints.insert("monitoring_ops".to_string(), id);

    // 6. Performance & Optimization
    let performance = section_map.get("performance").copied().unwrap_or(0);
    let id = add_shared_sprint(
        conn,
        "Performance & Optimization",
        "Profiling, benchmarking, and performance tuning",
        5,
        &[performance],
    )?;
    sprints.insert("performance".to_string(), id);

    // 7. Database & Data
    let database = section_map.get("database").copied().unwrap_or(0);
    let id = add_shared_sprint(
        conn,
        "Database & Data Management",
        "Schema design, migrations, and data integrity",
        6,
        &[database],
    )?;
    sprints.insert("database".to_string(), id);

    // 8. Accessibility & UX
    let a11y = section_map.get("a11y").copied().unwrap_or(0);
    let id = add_shared_sprint(
        conn,
        "Accessibility & UX",
        "WCAG compliance, usability, and inclusive design",
        7,
        &[a11y],
    )?;
    sprints.insert("a11y_ux".to_string(), id);

    Ok(sprints)
}
