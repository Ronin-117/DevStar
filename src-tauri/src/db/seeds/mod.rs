mod shared_sections;
mod shared_sprints;
mod templates;

use rusqlite::Connection;
use super::types::AppError;

pub use shared_sections::add_shared_section;
pub use shared_sprints::add_shared_sprint;
pub use templates::add_template;
pub use templates::add_template_sprint;
pub use templates::add_template_sprint_sections;
pub use templates::add_custom_sprint_section;

pub fn seed_all(conn: &Connection) -> Result<(), AppError> {
    let section_tuples = shared_sections::seed(conn)?;
    let section_map: std::collections::HashMap<String, i64> =
        section_tuples.into_iter().collect();

    let sprint_map = shared_sprints::seed(conn, &section_map)?;

    templates::web_dev::seed(conn, &section_map, &sprint_map)?;
    templates::mobile_app::seed(conn, &section_map, &sprint_map)?;
    templates::desktop_app::seed(conn, &section_map, &sprint_map)?;
    templates::game_dev::seed(conn, &section_map, &sprint_map)?;
    templates::embedded_iot::seed(conn, &section_map, &sprint_map)?;
    templates::api_backend::seed(conn, &section_map, &sprint_map)?;
    templates::data_science_ai::seed(conn, &section_map, &sprint_map)?;
    templates::cloud_infra::seed(conn, &section_map, &sprint_map)?;
    templates::systems_programming::seed(conn, &section_map, &sprint_map)?;
    templates::enterprise_systems::seed(conn, &section_map, &sprint_map)?;
    templates::security_software::seed(conn, &section_map, &sprint_map)?;
    templates::tools_libraries::seed(conn, &section_map, &sprint_map)?;

    Ok(())
}
