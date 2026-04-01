use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// ---------------------------------------------------------------------------
// Domain types
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    pub id: i64,
    pub template_id: i64,
    pub name: String,
    pub description: String,
    pub sort_order: i64,
    pub linked_from_section_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateItem {
    pub id: i64,
    pub section_id: i64,
    pub title: String,
    pub description: String,
    pub sort_order: i64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSection {
    pub id: i64,
    pub project_id: i64,
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

// Input types
#[derive(Debug, Deserialize)]
pub struct TemplateInput {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TemplateSectionInput {
    pub template_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub linked_from_section_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TemplateItemInput {
    pub section_id: i64,
    pub title: String,
    pub description: Option<String>,
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
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectItemInput {
    pub section_id: i64,
    pub title: String,
    pub description: Option<String>,
}

// Section with its items (for API responses)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionWithItems {
    pub section: TemplateSection,
    pub items: Vec<TemplateItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSectionWithItems {
    pub section: ProjectSection,
    pub items: Vec<ProjectItem>,
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
    conn: Mutex<Connection>,
}

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS templates (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    color       TEXT    NOT NULL DEFAULT '#6366f1',
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS template_sections (
    id                       INTEGER PRIMARY KEY AUTOINCREMENT,
    template_id              INTEGER NOT NULL REFERENCES templates(id) ON DELETE CASCADE,
    name                     TEXT    NOT NULL,
    description              TEXT    NOT NULL DEFAULT '',
    sort_order               INTEGER NOT NULL DEFAULT 0,
    linked_from_section_id   INTEGER REFERENCES template_sections(id)
);

CREATE TABLE IF NOT EXISTS template_items (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    section_id  INTEGER NOT NULL REFERENCES template_sections(id) ON DELETE CASCADE,
    title       TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    sort_order  INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS projects (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    template_id INTEGER NOT NULL,
    color       TEXT    NOT NULL DEFAULT '#6366f1',
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS project_sections (
    id                       INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id               INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name                     TEXT    NOT NULL,
    description              TEXT    NOT NULL DEFAULT '',
    sort_order               INTEGER NOT NULL DEFAULT 0,
    is_custom                INTEGER NOT NULL DEFAULT 0,
    linked_from_section_id   INTEGER REFERENCES template_sections(id)
);

CREATE TABLE IF NOT EXISTS project_items (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    section_id  INTEGER NOT NULL REFERENCES project_sections(id) ON DELETE CASCADE,
    title       TEXT    NOT NULL,
    description TEXT    NOT NULL DEFAULT '',
    checked     INTEGER NOT NULL DEFAULT 0,
    notes       TEXT    NOT NULL DEFAULT '',
    sort_order  INTEGER NOT NULL DEFAULT 0,
    is_custom   INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_template_sections_template ON template_sections(template_id);
CREATE INDEX IF NOT EXISTS idx_template_items_section ON template_items(section_id);
CREATE INDEX IF NOT EXISTS idx_project_sections_project ON project_sections(project_id);
CREATE INDEX IF NOT EXISTS idx_project_items_section ON project_items(section_id);
CREATE INDEX IF NOT EXISTS idx_projects_template ON projects(template_id);
"#;

impl Database {
    pub fn new(path: &str) -> Result<Self, AppError> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA foreign_keys = ON;")?;
        conn.execute_batch(SCHEMA)?;
        Ok(Database {
            conn: Mutex::new(conn),
        })
    }

    // --- Helper methods ---

    fn add_section_with_items(
        conn: &Connection,
        template_id: i64,
        name: &str,
        description: &str,
        items: &[(&str, &str)],
    ) -> Result<(), AppError> {
        let actual_template_id = if template_id < 0 {
            conn.execute(
                "INSERT INTO templates (name, description, color) VALUES (?1, ?2, ?3)",
                (name, "Shared section", "#6b7280"),
            )?;
            conn.last_insert_rowid()
        } else {
            template_id
        };

        let max_order: i64 = conn.query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM template_sections WHERE template_id = ?1",
            [actual_template_id],
            |row| row.get(0),
        )?;
        let sort_order = max_order + 1;

        conn.execute(
            "INSERT INTO template_sections (template_id, name, description, sort_order) VALUES (?1, ?2, ?3, ?4)",
            (actual_template_id, name, description, sort_order),
        )?;
        let section_id = conn.last_insert_rowid();

        for (i, (title, desc)) in items.iter().enumerate() {
            conn.execute(
                "INSERT INTO template_items (section_id, title, description, sort_order) VALUES (?1, ?2, ?3, ?4)",
                (section_id, title, desc, i as i64),
            )?;
        }
        Ok(())
    }

    fn get_template_internal(&self, conn: &Connection, id: i64) -> Option<Template> {
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

    fn get_section_internal(&self, conn: &Connection, id: i64) -> Option<TemplateSection> {
        let mut stmt = conn.prepare(
            "SELECT id, template_id, name, description, sort_order, linked_from_section_id FROM template_sections WHERE id = ?1",
        ).ok()?;
        stmt.query_row([id], |row| {
            Ok(TemplateSection {
                id: row.get(0)?,
                template_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                sort_order: row.get(4)?,
                linked_from_section_id: row.get(5)?,
            })
        })
        .ok()
    }

    fn get_item_internal(&self, conn: &Connection, id: i64) -> Option<TemplateItem> {
        let mut stmt = conn.prepare(
            "SELECT id, section_id, title, description, sort_order FROM template_items WHERE id = ?1",
        ).ok()?;
        stmt.query_row([id], |row| {
            Ok(TemplateItem {
                id: row.get(0)?,
                section_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                sort_order: row.get(4)?,
            })
        })
        .ok()
    }

    fn get_project_internal(&self, conn: &Connection, id: i64) -> Option<Project> {
        let mut stmt = conn.prepare(
            "SELECT id, name, description, template_id, color, created_at, updated_at FROM projects WHERE id = ?1",
        ).ok()?;
        stmt.query_row([id], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                template_id: row.get(3)?,
                color: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .ok()
    }

    fn get_project_section_internal(&self, conn: &Connection, id: i64) -> Option<ProjectSection> {
        let mut stmt = conn.prepare(
            "SELECT id, project_id, name, description, sort_order, is_custom, linked_from_section_id FROM project_sections WHERE id = ?1",
        ).ok()?;
        stmt.query_row([id], |row| {
            Ok(ProjectSection {
                id: row.get(0)?,
                project_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                sort_order: row.get(4)?,
                is_custom: row.get::<_, i64>(5)? != 0,
                linked_from_section_id: row.get(6)?,
            })
        })
        .ok()
    }

    fn get_project_item_internal(&self, conn: &Connection, id: i64) -> Option<ProjectItem> {
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

    fn get_items_for_section(
        &self,
        conn: &Connection,
        section_id: i64,
    ) -> Result<Vec<TemplateItem>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id, section_id, title, description, sort_order FROM template_items WHERE section_id = ?1 ORDER BY sort_order",
        )?;
        let items = stmt
            .query_map([section_id], |row| {
                Ok(TemplateItem {
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

    fn list_template_sections_internal(
        &self,
        conn: &Connection,
        template_id: i64,
    ) -> Result<Vec<TemplateSection>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id, template_id, name, description, sort_order, linked_from_section_id FROM template_sections WHERE template_id = ?1 ORDER BY sort_order",
        )?;
        let sections = stmt
            .query_map([template_id], |row| {
                Ok(TemplateSection {
                    id: row.get(0)?,
                    template_id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    sort_order: row.get(4)?,
                    linked_from_section_id: row.get(5)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(sections)
    }

    // --- Seed ---

    pub fn seed_if_empty(&self) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT count(*) FROM templates", [], |row| row.get(0))?;
        if count > 0 {
            return Ok(());
        }

        // Shared sections — each is a single-section template
        Self::add_section_with_items(
            &conn,
            -1,
            "Shared: Docker Setup",
            "Container configuration for development and production",
            &[
                (
                    "Dockerfile multi-stage build configured",
                    "Separate build and runtime stages",
                ),
                (
                    "docker-compose.yml for local dev",
                    "Database, app, and any dependencies",
                ),
                (
                    "Environment variables externalized",
                    "No hardcoded secrets in images",
                ),
                (
                    "Image size optimized",
                    "Use alpine/distroless base where possible",
                ),
                ("Health checks configured", "Docker healthcheck in compose"),
            ],
        )?;

        Self::add_section_with_items(
            &conn,
            -1,
            "Shared: Security Basics",
            "Core security practices across all project types",
            &[
                (
                    "Input validation on all user inputs",
                    "Server-side validation, not just client",
                ),
                (
                    "Authentication flow designed",
                    "JWT, sessions, or OAuth decided",
                ),
                ("Authorization rules defined", "RBAC or similar"),
                (
                    "Secrets not in code/repos",
                    "Use env vars or secret manager",
                ),
                ("HTTPS/TLS enforced", "No plain HTTP in production"),
                (
                    "Error messages don't leak internals",
                    "Generic error responses",
                ),
            ],
        )?;

        Self::add_section_with_items(
            &conn,
            -1,
            "Shared: CI/CD Pipeline",
            "Automated build, test, and deploy",
            &[
                ("CI pipeline configured", "Lint, test, build on every push"),
                ("Automated tests in CI", "Unit + integration tests"),
                ("Build artifact versioning", "Semantic versioning strategy"),
                ("Deployment pipeline defined", "Staging → production flow"),
                ("Rollback strategy defined", "How to revert a bad deploy"),
            ],
        )?;

        // Full Stack Web App template
        conn.execute(
            "INSERT INTO templates (name, description, color) VALUES (?1, ?2, ?3)",
            (
                "Full Stack Web App",
                "React + Node / Next.js / Django — complete web app checklist",
                "#3b82f6",
            ),
        )?;
        let web_id = conn.last_insert_rowid();
        Self::add_section_with_items(
            &conn,
            web_id,
            "Project Foundation",
            "Define the base architecture",
            &[
                ("App type defined (SPA, SSR, SSG)", ""),
                ("SEO requirements decided", ""),
                ("Target devices (mobile-first or desktop)", ""),
                ("Browser support defined", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            web_id,
            "Architecture",
            "Frontend-backend boundary and rendering",
            &[
                ("API-first design (no tight coupling)", ""),
                ("Separate deployable units", "Frontend vs backend"),
                ("Monorepo vs separate repos decided", ""),
                ("Rendering strategy chosen (CSR/SSR/Hybrid)", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            web_id,
            "API Layer",
            "REST or GraphQL setup",
            &[
                (
                    "API base client configured",
                    "/services/api.ts or equivalent",
                ),
                ("Auth flow defined (cookies vs tokens)", ""),
                ("CORS configured properly", ""),
                ("Rate limiting for public APIs", ""),
                ("CSRF protection (if using cookies)", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            web_id,
            "Frontend UI Layer",
            "Components, state, and UX",
            &[
                ("Pages vs components separation", ""),
                ("Layout system (header/sidebar/footer)", ""),
                ("Routing configured (file-based or config)", ""),
                ("Server state management (React Query/SWR)", ""),
                ("Client state management (Zustand/Redux)", ""),
                ("Loading skeletons implemented", ""),
                ("Error boundaries in place", ""),
                ("Empty states designed", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            web_id,
            "Frontend-Backend Integration",
            "Connecting the layers",
            &[
                ("API service layer (no direct fetch in components)", ""),
                ("Global error handler (401, 500, etc.)", ""),
                ("Token refresh logic", ""),
                ("Retry logic for failed requests", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            web_id,
            "Database",
            "Schema and patterns",
            &[
                ("User-centric schema (accounts, sessions)", ""),
                ("Audit fields (created_at, updated_at)", ""),
                ("Soft deletes if needed", ""),
                ("Indexing for common queries", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            web_id,
            "Performance",
            "Web-specific optimizations",
            &[
                ("Code splitting configured", ""),
                ("Lazy loading components", ""),
                ("Image optimization", ""),
                ("Bundle size checked", ""),
                ("Backend response time optimized", ""),
                ("Caching strategy (Redis / HTTP cache)", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            web_id,
            "Observability",
            "Monitoring and tracking",
            &[
                ("Frontend error tracking (Sentry)", ""),
                ("Backend logging configured", ""),
                ("API latency tracking", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            web_id,
            "Documentation",
            "Setup and reference",
            &[
                ("API docs (Swagger/OpenAPI)", ""),
                ("Frontend env setup documented", ""),
                ("Deployment steps documented", ""),
                ("Example .env provided", ""),
            ],
        )?;

        // Mobile App template
        conn.execute(
            "INSERT INTO templates (name, description, color) VALUES (?1, ?2, ?3)",
            (
                "Mobile App",
                "React Native / Flutter / Swift / Kotlin — mobile checklist",
                "#8b5cf6",
            ),
        )?;
        let mobile_id = conn.last_insert_rowid();
        Self::add_section_with_items(
            &conn,
            mobile_id,
            "Project Foundation",
            "Platform and device decisions",
            &[
                ("Target platforms (iOS / Android / both)", ""),
                ("Device range (low-end vs high-end)", ""),
                ("Connectivity assumptions (offline-first or not)", ""),
                ("App store requirements understood", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            mobile_id,
            "Architecture",
            "Structure and separation",
            &[
                ("Native vs Cross-platform decided", ""),
                ("Feature-based modules (not just components folder)", ""),
                ("Clear separation: UI / Logic / Data", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            mobile_id,
            "Navigation & App Flow",
            "Screens and routing",
            &[
                ("Navigation system (stack / tab / drawer)", ""),
                ("Deep linking configured", ""),
                ("Screen lifecycle awareness", ""),
                ("Auth flow (login → app → logout)", ""),
                ("Onboarding flow designed", ""),
                ("Error fallback screens", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            mobile_id,
            "UI / UX",
            "Mobile-specific design",
            &[
                ("Responsive for different screen sizes", ""),
                ("Safe areas (notch, status bar)", ""),
                ("Keyboard handling", ""),
                ("Loading indicators", ""),
                ("Offline state UI", ""),
                ("Retry UI for failed actions", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            mobile_id,
            "API & Data Layer",
            "Network and offline",
            &[
                ("Central API service layer", ""),
                ("Request/response interceptors", ""),
                ("Token storage + refresh logic", ""),
                ("Local cache strategy (SQLite / AsyncStorage)", ""),
                ("Sync mechanism when back online", ""),
                ("Conflict resolution strategy", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            mobile_id,
            "Local Storage",
            "Persistent data",
            &[
                ("Secure storage for tokens (keychain/keystore)", ""),
                ("Persistent app data strategy", ""),
                ("Cache invalidation rules", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            mobile_id,
            "Device & OS Integration",
            "Native features",
            &[
                ("Permissions defined (camera, location, storage)", ""),
                ("Push notifications (FCM / APNs)", ""),
                ("Background tasks configured", ""),
                ("App lifecycle handling", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            mobile_id,
            "Performance",
            "Mobile-critical optimizations",
            &[
                ("Avoid unnecessary re-renders", ""),
                ("Optimize lists (FlatList / RecyclerView)", ""),
                ("Image optimization", ""),
                ("Bundle size reduced", ""),
                ("Memory leak checks", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            mobile_id,
            "Build & Distribution",
            "Store readiness",
            &[
                ("Android: APK/AAB build + signing", ""),
                ("iOS: Certificates & provisioning profiles", ""),
                ("Versioning strategy", ""),
                ("Release notes process", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            mobile_id,
            "Observability",
            "Crash reporting and analytics",
            &[
                ("Crash reporting (Firebase Crashlytics / Sentry)", ""),
                ("Analytics (user behavior)", ""),
                ("Performance monitoring", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            mobile_id,
            "Documentation",
            "Setup and release",
            &[
                ("Setup steps documented (especially iOS builds)", ""),
                ("Environment configs documented", ""),
                ("Release process documented", ""),
            ],
        )?;

        // Desktop/Cross-Platform template
        conn.execute(
            "INSERT INTO templates (name, description, color) VALUES (?1, ?2, ?3)",
            (
                "Desktop / Cross-Platform",
                "Tauri / Electron / native desktop checklist",
                "#f59e0b",
            ),
        )?;
        let desktop_id = conn.last_insert_rowid();
        Self::add_section_with_items(
            &conn,
            desktop_id,
            "Project Foundation",
            "Platform and framework",
            &[
                ("Target platforms (Windows / macOS / Linux)", ""),
                ("Framework chosen (Tauri / Electron / native)", ""),
                ("Minimum OS versions defined", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            desktop_id,
            "Architecture",
            "App structure",
            &[
                ("Main process vs renderer separation", ""),
                ("IPC contract defined", ""),
                ("State management strategy", ""),
                ("Feature-based module organization", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            desktop_id,
            "UI Layer",
            "Desktop UI considerations",
            &[
                ("Window management (size, resize, min/max)", ""),
                ("System tray integration if needed", ""),
                ("Native menus and dialogs", ""),
                ("Dark mode support", ""),
                ("Accessibility (keyboard nav, screen readers)", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            desktop_id,
            "Data Layer",
            "Local storage and sync",
            &[
                ("Local database configured (SQLite)", ""),
                ("File system access patterns", ""),
                ("Settings/preferences storage", ""),
                ("Cloud sync if needed", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            desktop_id,
            "Security",
            "Desktop-specific security",
            &[
                ("CSP configured", ""),
                ("Node integration disabled (Electron)", ""),
                ("Context isolation enabled", ""),
                ("No secrets bundled in app", ""),
                ("Auto-update with signature verification", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            desktop_id,
            "Build & Distribution",
            "Packaging and release",
            &[
                ("Installer packaging (MSI / DMG / deb / rpm)", ""),
                ("Code signing certificates", ""),
                ("Auto-update channel configured", ""),
                ("Release notes process", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            desktop_id,
            "Performance",
            "Desktop optimizations",
            &[
                ("Memory usage monitored", ""),
                ("Startup time optimized", ""),
                ("Bundle size checked", ""),
            ],
        )?;
        Self::add_section_with_items(
            &conn,
            desktop_id,
            "Documentation",
            "Setup and reference",
            &[
                ("Dev setup documented", ""),
                ("Build process documented", ""),
                ("Release process documented", ""),
            ],
        )?;

        Ok(())
    }

    // --- Templates ---

    pub fn list_templates(&self) -> Result<Vec<Template>, AppError> {
        let conn = self.conn.lock().unwrap();
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

    pub fn create_template(&self, input: TemplateInput) -> Result<Template, AppError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO templates (name, description, color) VALUES (?1, ?2, ?3)",
            (
                &input.name,
                input.description.as_deref().unwrap_or(""),
                input.color.as_deref().unwrap_or("#6366f1"),
            ),
        )?;
        let id = conn.last_insert_rowid();
        self.get_template_internal(&conn, id)
            .ok_or_else(|| AppError::NotFound(format!("template {id}")))
    }

    pub fn update_template(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        color: Option<String>,
    ) -> Result<Template, AppError> {
        let conn = self.conn.lock().unwrap();
        if self.get_template_internal(&conn, id).is_none() {
            return Err(AppError::NotFound(format!("template {id}")));
        }
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
        self.get_template_internal(&conn, id)
            .ok_or_else(|| AppError::NotFound(format!("template {id}")))
    }

    pub fn delete_template(&self, id: i64) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM templates WHERE id = ?1", [id])?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("template {id}")));
        }
        Ok(())
    }

    // --- Template Sections ---

    pub fn list_template_sections(
        &self,
        template_id: i64,
    ) -> Result<Vec<TemplateSection>, AppError> {
        let conn = self.conn.lock().unwrap();
        self.list_template_sections_internal(&conn, template_id)
    }

    pub fn get_template_section_with_items(
        &self,
        section_id: i64,
    ) -> Result<SectionWithItems, AppError> {
        let conn = self.conn.lock().unwrap();
        let section = self
            .get_section_internal(&conn, section_id)
            .ok_or_else(|| AppError::NotFound(format!("section {section_id}")))?;

        let effective_section_id = section.linked_from_section_id.unwrap_or(section_id);
        let items = self.get_items_for_section(&conn, effective_section_id)?;

        Ok(SectionWithItems { section, items })
    }

    pub fn list_template_sections_with_items(
        &self,
        template_id: i64,
    ) -> Result<Vec<SectionWithItems>, AppError> {
        let sections = self.list_template_sections(template_id)?;
        let mut result = Vec::new();
        for section in sections {
            let effective_id = section.linked_from_section_id.unwrap_or(section.id);
            let items = {
                let conn = self.conn.lock().unwrap();
                self.get_items_for_section(&conn, effective_id)?
            };
            result.push(SectionWithItems { section, items });
        }
        Ok(result)
    }

    pub fn add_template_section(
        &self,
        input: TemplateSectionInput,
    ) -> Result<TemplateSection, AppError> {
        let conn = self.conn.lock().unwrap();
        let max_order: i64 = conn.query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM template_sections WHERE template_id = ?1",
            [input.template_id],
            |row| row.get(0),
        )?;
        conn.execute(
            "INSERT INTO template_sections (template_id, name, description, sort_order, linked_from_section_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            (input.template_id, &input.name, input.description.as_deref().unwrap_or(""), max_order + 1, input.linked_from_section_id),
        )?;
        let id = conn.last_insert_rowid();
        self.get_section_internal(&conn, id)
            .ok_or_else(|| AppError::NotFound(format!("section {id}")))
    }

    pub fn update_template_section(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<TemplateSection, AppError> {
        let conn = self.conn.lock().unwrap();
        if let Some(ref n) = name {
            conn.execute(
                "UPDATE template_sections SET name = ?1 WHERE id = ?2",
                (n, id),
            )?;
        }
        if let Some(ref d) = description {
            conn.execute(
                "UPDATE template_sections SET description = ?1 WHERE id = ?2",
                (d, id),
            )?;
        }
        self.get_section_internal(&conn, id)
            .ok_or_else(|| AppError::NotFound(format!("section {id}")))
    }

    pub fn delete_template_section(&self, id: i64) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM template_sections WHERE id = ?1", [id])?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("section {id}")));
        }
        Ok(())
    }

    pub fn reorder_template_section(&self, id: i64, new_order: i64) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute(
            "UPDATE template_sections SET sort_order = ?1 WHERE id = ?2",
            (new_order, id),
        )?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("section {id}")));
        }
        Ok(())
    }

    // --- Template Items ---

    pub fn list_template_items(&self, section_id: i64) -> Result<Vec<TemplateItem>, AppError> {
        let conn = self.conn.lock().unwrap();
        self.get_items_for_section(&conn, section_id)
    }

    pub fn add_template_item(&self, input: TemplateItemInput) -> Result<TemplateItem, AppError> {
        let conn = self.conn.lock().unwrap();
        let max_order: i64 = conn.query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM template_items WHERE section_id = ?1",
            [input.section_id],
            |row| row.get(0),
        )?;
        conn.execute(
            "INSERT INTO template_items (section_id, title, description, sort_order) VALUES (?1, ?2, ?3, ?4)",
            (input.section_id, &input.title, input.description.as_deref().unwrap_or(""), max_order + 1),
        )?;
        let id = conn.last_insert_rowid();
        self.get_item_internal(&conn, id)
            .ok_or_else(|| AppError::NotFound(format!("item {id}")))
    }

    pub fn update_template_item(
        &self,
        id: i64,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<TemplateItem, AppError> {
        let conn = self.conn.lock().unwrap();
        if let Some(ref t) = title {
            conn.execute(
                "UPDATE template_items SET title = ?1 WHERE id = ?2",
                (t, id),
            )?;
        }
        if let Some(ref d) = description {
            conn.execute(
                "UPDATE template_items SET description = ?1 WHERE id = ?2",
                (d, id),
            )?;
        }
        self.get_item_internal(&conn, id)
            .ok_or_else(|| AppError::NotFound(format!("item {id}")))
    }

    pub fn delete_template_item(&self, id: i64) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM template_items WHERE id = ?1", [id])?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("item {id}")));
        }
        Ok(())
    }

    pub fn reorder_template_item(&self, id: i64, new_order: i64) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute(
            "UPDATE template_items SET sort_order = ?1 WHERE id = ?2",
            (new_order, id),
        )?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("item {id}")));
        }
        Ok(())
    }

    // --- Projects ---

    pub fn list_projects(&self) -> Result<Vec<Project>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, description, template_id, color, created_at, updated_at FROM projects ORDER BY updated_at DESC",
        )?;
        let projects = stmt
            .query_map([], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    template_id: row.get(3)?,
                    color: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;
        Ok(projects)
    }

    pub fn create_project_from_template(&self, input: ProjectInput) -> Result<Project, AppError> {
        let conn = self.conn.lock().unwrap();

        let template_exists: i64 = conn.query_row(
            "SELECT count(*) FROM templates WHERE id = ?1",
            [input.template_id],
            |row| row.get(0),
        )?;
        if template_exists == 0 {
            return Err(AppError::NotFound(format!(
                "template {}",
                input.template_id
            )));
        }

        conn.execute(
            "INSERT INTO projects (name, description, template_id, color) VALUES (?1, ?2, ?3, ?4)",
            (
                &input.name,
                input.description.as_deref().unwrap_or(""),
                input.template_id,
                input.color.as_deref().unwrap_or("#6366f1"),
            ),
        )?;
        let project_id = conn.last_insert_rowid();

        let sections = self.list_template_sections_internal(&conn, input.template_id)?;
        for section in &sections {
            let effective_section_id = section.linked_from_section_id.unwrap_or(section.id);
            let items = self.get_items_for_section(&conn, effective_section_id)?;

            conn.execute(
                "INSERT INTO project_sections (project_id, name, description, sort_order, is_custom) VALUES (?1, ?2, ?3, ?4, 0)",
                (project_id, &section.name, &section.description, section.sort_order),
            )?;
            let project_section_id = conn.last_insert_rowid();

            for item in &items {
                conn.execute(
                    "INSERT INTO project_items (section_id, title, description, checked, notes, sort_order, is_custom) VALUES (?1, ?2, ?3, 0, '', ?4, 0)",
                    (project_section_id, &item.title, &item.description, item.sort_order),
                )?;
            }
        }

        self.get_project_internal(&conn, project_id)
            .ok_or_else(|| AppError::NotFound(format!("project {project_id}")))
    }

    pub fn delete_project(&self, id: i64) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM projects WHERE id = ?1", [id])?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("project {id}")));
        }
        Ok(())
    }

    // --- Project Sections & Items ---

    pub fn list_project_sections_with_items(
        &self,
        project_id: i64,
    ) -> Result<Vec<ProjectSectionWithItems>, AppError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, name, description, sort_order, is_custom FROM project_sections WHERE project_id = ?1 ORDER BY sort_order",
        )?;
        let sections = stmt
            .query_map([project_id], |row| {
                Ok(ProjectSection {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    sort_order: row.get(4)?,
                    is_custom: row.get::<_, i64>(5)? != 0,
                    linked_from_section_id: None,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;

        let mut result = Vec::new();
        for section in sections {
            let mut item_stmt = conn.prepare(
                "SELECT id, section_id, title, description, checked, notes, sort_order, is_custom FROM project_items WHERE section_id = ?1 ORDER BY sort_order",
            )?;
            let items: SqliteResult<Vec<ProjectItem>> = item_stmt
                .query_map([section.id], |row| {
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
            result.push(ProjectSectionWithItems {
                section,
                items: items?,
            });
        }
        Ok(result)
    }

    pub fn update_project_item(&self, update: ProjectItemUpdate) -> Result<ProjectItem, AppError> {
        let conn = self.conn.lock().unwrap();
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
        self.get_project_item_internal(&conn, update.id)
            .ok_or_else(|| AppError::NotFound(format!("item {}", update.id)))
    }

    pub fn add_project_section(
        &self,
        input: ProjectSectionInput,
    ) -> Result<ProjectSection, AppError> {
        let conn = self.conn.lock().unwrap();
        let max_order: i64 = conn.query_row(
            "SELECT COALESCE(MAX(sort_order), -1) FROM project_sections WHERE project_id = ?1",
            [input.project_id],
            |row| row.get(0),
        )?;
        conn.execute(
            "INSERT INTO project_sections (project_id, name, description, sort_order, is_custom) VALUES (?1, ?2, ?3, ?4, 1)",
            (input.project_id, &input.name, input.description.as_deref().unwrap_or(""), max_order + 1),
        )?;
        let id = conn.last_insert_rowid();
        self.get_project_section_internal(&conn, id)
            .ok_or_else(|| AppError::NotFound(format!("section {id}")))
    }

    pub fn add_project_item(&self, input: ProjectItemInput) -> Result<ProjectItem, AppError> {
        let conn = self.conn.lock().unwrap();
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
        self.get_project_item_internal(&conn, id)
            .ok_or_else(|| AppError::NotFound(format!("item {id}")))
    }

    pub fn delete_project_item(&self, id: i64) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM project_items WHERE id = ?1", [id])?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("item {id}")));
        }
        Ok(())
    }

    pub fn delete_project_section(&self, id: i64) -> Result<(), AppError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute("DELETE FROM project_sections WHERE id = ?1", [id])?;
        if affected == 0 {
            return Err(AppError::NotFound(format!("section {id}")));
        }
        Ok(())
    }

    // --- Project progress ---

    pub fn get_project_progress(&self, project_id: i64) -> Result<(i64, i64), AppError> {
        let conn = self.conn.lock().unwrap();
        let total: i64 = conn.query_row(
            "SELECT count(*) FROM project_items pi JOIN project_sections ps ON pi.section_id = ps.id WHERE ps.project_id = ?1",
            [project_id],
            |row| row.get(0),
        )?;
        let checked: i64 = conn.query_row(
            "SELECT count(*) FROM project_items pi JOIN project_sections ps ON pi.section_id = ps.id WHERE ps.project_id = ?1 AND pi.checked = 1",
            [project_id],
            |row| row.get(0),
        )?;
        Ok((checked, total))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db() -> Database {
        let db = Database::new(":memory:").expect("Failed to create in-memory database");
        db.seed_if_empty().expect("Failed to seed database");
        db
    }

    #[test]
    fn test_schema_creates_all_tables() {
        let db = Database::new(":memory:").unwrap();
        let conn = db.conn.lock().unwrap();
        let tables = [
            "templates",
            "template_sections",
            "template_items",
            "projects",
            "project_sections",
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
        let templates = db.list_templates().unwrap();
        assert_eq!(templates.len(), 6);
        let names: Vec<&str> = templates.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"Full Stack Web App"));
        assert!(names.contains(&"Mobile App"));
        assert!(names.contains(&"Desktop / Cross-Platform"));
    }

    #[test]
    fn test_seed_creates_sections_and_items() {
        let db = test_db();
        let templates = db.list_templates().unwrap();
        let web = templates
            .iter()
            .find(|t| t.name == "Full Stack Web App")
            .unwrap();
        let sections = db.list_template_sections_with_items(web.id).unwrap();
        assert!(!sections.is_empty());
        for section in &sections {
            assert!(
                !section.items.is_empty(),
                "Section '{}' should have items",
                section.section.name
            );
        }
    }

    #[test]
    fn test_shared_sections_have_items() {
        let db = test_db();
        let templates = db.list_templates().unwrap();
        let shared: Vec<&Template> = templates
            .iter()
            .filter(|t| t.name.starts_with("Shared:"))
            .collect();
        assert_eq!(shared.len(), 3);
        for template in &shared {
            let sections = db.list_template_sections_with_items(template.id).unwrap();
            assert_eq!(
                sections.len(),
                1,
                "Shared template '{}' should have exactly one section",
                template.name
            );
            assert!(
                !sections[0].items.is_empty(),
                "Shared section '{}' should have items",
                sections[0].section.name
            );
        }
    }

    #[test]
    fn test_create_project_from_template() {
        let db = test_db();
        let templates = db.list_templates().unwrap();
        let web = templates
            .iter()
            .find(|t| t.name == "Full Stack Web App")
            .unwrap();

        let project = db
            .create_project_from_template(ProjectInput {
                name: "My Web App".into(),
                description: Some("A test project".into()),
                template_id: web.id,
                color: None,
            })
            .unwrap();
        assert_eq!(project.name, "My Web App");
        assert_eq!(project.template_id, web.id);

        let sections = db.list_project_sections_with_items(project.id).unwrap();
        assert!(!sections.is_empty());
        for section in &sections {
            assert!(
                !section.items.is_empty(),
                "Section '{}' should have items",
                section.section.name
            );
        }
    }

    #[test]
    fn test_project_is_independent_snapshot() {
        let db = test_db();
        let templates = db.list_templates().unwrap();
        let web = templates
            .iter()
            .find(|t| t.name == "Full Stack Web App")
            .unwrap();

        let project = db
            .create_project_from_template(ProjectInput {
                name: "Snapshot Test".into(),
                description: None,
                template_id: web.id,
                color: None,
            })
            .unwrap();

        let project_sections = db.list_project_sections_with_items(project.id).unwrap();
        let project_item_count: usize = project_sections.iter().map(|s| s.items.len()).sum();

        db.delete_template(web.id).unwrap();

        let project_sections = db.list_project_sections_with_items(project.id).unwrap();
        let project_item_count_after: usize = project_sections.iter().map(|s| s.items.len()).sum();
        assert_eq!(project_item_count, project_item_count_after);
    }

    #[test]
    fn test_toggle_project_item() {
        let db = test_db();
        let templates = db.list_templates().unwrap();
        let web = templates
            .iter()
            .find(|t| t.name == "Full Stack Web App")
            .unwrap();

        let project = db
            .create_project_from_template(ProjectInput {
                name: "Toggle Test".into(),
                description: None,
                template_id: web.id,
                color: None,
            })
            .unwrap();

        let sections = db.list_project_sections_with_items(project.id).unwrap();
        let first_item = &sections[0].items[0];
        assert!(!first_item.checked);

        let updated = db
            .update_project_item(ProjectItemUpdate {
                id: first_item.id,
                checked: Some(true),
                notes: None,
            })
            .unwrap();
        assert!(updated.checked);
    }

    #[test]
    fn test_project_progress() {
        let db = test_db();
        let templates = db.list_templates().unwrap();
        let web = templates
            .iter()
            .find(|t| t.name == "Full Stack Web App")
            .unwrap();

        let project = db
            .create_project_from_template(ProjectInput {
                name: "Progress Test".into(),
                description: None,
                template_id: web.id,
                color: None,
            })
            .unwrap();

        let (checked, total) = db.get_project_progress(project.id).unwrap();
        assert_eq!(checked, 0);
        assert!(total > 0);

        let sections = db.list_project_sections_with_items(project.id).unwrap();
        for section in &sections {
            for item in &section.items {
                db.update_project_item(ProjectItemUpdate {
                    id: item.id,
                    checked: Some(true),
                    notes: None,
                })
                .unwrap();
            }
        }

        let (checked, total) = db.get_project_progress(project.id).unwrap();
        assert_eq!(checked, total);
    }

    #[test]
    fn test_add_custom_project_section_and_item() {
        let db = test_db();
        let templates = db.list_templates().unwrap();
        let web = templates
            .iter()
            .find(|t| t.name == "Full Stack Web App")
            .unwrap();

        let project = db
            .create_project_from_template(ProjectInput {
                name: "Custom Test".into(),
                description: None,
                template_id: web.id,
                color: None,
            })
            .unwrap();

        let section = db
            .add_project_section(ProjectSectionInput {
                project_id: project.id,
                name: "Custom Section".into(),
                description: None,
            })
            .unwrap();

        let item = db
            .add_project_item(ProjectItemInput {
                section_id: section.id,
                title: "Custom Item".into(),
                description: None,
            })
            .unwrap();

        assert!(item.is_custom);
        assert_eq!(item.title, "Custom Item");
    }

    #[test]
    fn test_delete_project_cascades() {
        let db = test_db();
        let templates = db.list_templates().unwrap();
        let web = templates
            .iter()
            .find(|t| t.name == "Full Stack Web App")
            .unwrap();

        let project = db
            .create_project_from_template(ProjectInput {
                name: "Delete Test".into(),
                description: None,
                template_id: web.id,
                color: None,
            })
            .unwrap();

        db.delete_project(project.id).unwrap();
        let projects = db.list_projects().unwrap();
        assert!(!projects.iter().any(|p| p.id == project.id));
    }

    #[test]
    fn test_template_crud() {
        let db = test_db();
        let t = db
            .create_template(TemplateInput {
                name: "New Template".into(),
                description: Some("Test".into()),
                color: Some("#ff0000".into()),
            })
            .unwrap();
        assert_eq!(t.name, "New Template");

        let updated = db
            .update_template(t.id, Some("Updated".into()), None, None)
            .unwrap();
        assert_eq!(updated.name, "Updated");

        db.delete_template(t.id).unwrap();
        let templates = db.list_templates().unwrap();
        assert!(!templates.iter().any(|tmpl| tmpl.id == t.id));
    }

    #[test]
    fn test_template_section_and_item_crud() {
        let db = test_db();
        let templates = db.list_templates().unwrap();
        let web = templates
            .iter()
            .find(|t| t.name == "Full Stack Web App")
            .unwrap();

        let section = db
            .add_template_section(TemplateSectionInput {
                template_id: web.id,
                name: "New Section".into(),
                description: None,
                linked_from_section_id: None,
            })
            .unwrap();

        let item = db
            .add_template_item(TemplateItemInput {
                section_id: section.id,
                title: "New Item".into(),
                description: Some("Test description".into()),
            })
            .unwrap();
        assert_eq!(item.title, "New Item");

        let updated = db
            .update_template_item(item.id, Some("Updated Item".into()), None)
            .unwrap();
        assert_eq!(updated.title, "Updated Item");

        db.delete_template_item(item.id).unwrap();
        let items = db.list_template_items(section.id).unwrap();
        assert!(!items.iter().any(|i| i.id == item.id));
    }

    #[test]
    fn test_seed_is_idempotent() {
        let db = Database::new(":memory:").unwrap();
        db.seed_if_empty().unwrap();
        db.seed_if_empty().unwrap();
        let templates = db.list_templates().unwrap();
        assert_eq!(templates.len(), 6);
    }
}
