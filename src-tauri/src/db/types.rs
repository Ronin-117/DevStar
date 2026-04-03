use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// ---------------------------------------------------------------------------
// Shared Sections
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSection {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSectionItem {
    pub id: i64,
    pub section_id: i64,
    pub title: String,
    pub description: String,
    pub sort_order: i64,
}

// ---------------------------------------------------------------------------
// Shared Sprints
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSprint {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub sort_order: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSprintSection {
    pub id: i64,
    pub sprint_id: i64,
    pub section_id: i64,
    pub sort_order: i64,
    pub is_linked: bool,
}

// ---------------------------------------------------------------------------
// Templates
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Template Sprints
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSprint {
    pub id: i64,
    pub template_id: i64,
    pub name: String,
    pub description: String,
    pub sort_order: i64,
    pub is_custom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSprintSection {
    pub id: i64,
    pub sprint_id: i64,
    pub section_id: i64,
    pub sort_order: i64,
    pub is_linked: bool,
}

// ---------------------------------------------------------------------------
// Projects
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub template_id: i64,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Project Sprints
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSprint {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub description: String,
    pub status: String, // pending, active, done
    pub sort_order: i64,
    pub is_custom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSprintSection {
    pub id: i64,
    pub sprint_id: i64,
    pub name: String,
    pub description: String,
    pub sort_order: i64,
    pub is_custom: bool,
    pub linked_from_section_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectItem {
    pub id: i64,
    pub section_id: i64,
    pub title: String,
    pub description: String,
    pub checked: bool,
    pub notes: String,
    pub sort_order: i64,
    pub is_custom: bool,
}

// ---------------------------------------------------------------------------
// API response wrappers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionWithItems {
    pub section: SharedSection,
    pub items: Vec<SharedSectionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSprintWithSections {
    pub sprint: SharedSprint,
    pub sections: Vec<SharedSprintSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSprintWithSections {
    pub sprint: TemplateSprint,
    pub sections: Vec<TemplateSprintSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSprintWithSections {
    pub sprint: ProjectSprint,
    pub sections: Vec<ProjectSprintSectionWithItems>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSprintSectionWithItems {
    pub section: ProjectSprintSection,
    pub items: Vec<ProjectItem>,
}

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct TemplateInput {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SharedSectionInput {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SharedSectionItemInput {
    pub section_id: i64,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SharedSprintInput {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SharedSprintSectionInput {
    pub sprint_id: i64,
    pub section_id: i64,
    pub is_linked: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProjectInput {
    pub name: String,
    pub description: Option<String>,
    pub template_id: i64,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectItemUpdate {
    pub id: i64,
    pub checked: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectSectionInput {
    pub sprint_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub linked_from_section_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectItemInput {
    pub section_id: i64,
    pub title: String,
    pub description: Option<String>,
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum AppError {
    Database(String),
    Serialization(String),
    NotFound(String),
    Validation(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Database(msg) => write!(f, "Database error: {msg}"),
            AppError::Serialization(msg) => write!(f, "Serialization error: {msg}"),
            AppError::NotFound(msg) => write!(f, "Not found: {msg}"),
            AppError::Validation(msg) => write!(f, "Validation error: {msg}"),
        }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::Database(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Serialization(e.to_string())
    }
}

// ---------------------------------------------------------------------------
// Database wrapper
// ---------------------------------------------------------------------------

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, AppError> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA foreign_keys = ON;")?;
        conn.execute_batch(include_str!("schema.sql"))?;
        Ok(Database {
            conn: Mutex::new(conn),
        })
    }
}
