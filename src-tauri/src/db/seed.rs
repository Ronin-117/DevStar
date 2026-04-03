use super::types::AppError;
use rusqlite::Connection;

pub fn seed_if_empty(conn: &Connection) -> Result<(), AppError> {
    let count: i64 = conn.query_row("SELECT count(*) FROM templates", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    // --- Shared Sections ---
    let docker_id = add_shared_section_with_items(
        conn,
        "Docker Setup",
        "Container configuration for development and production",
        "#6b7280",
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

    let security_id = add_shared_section_with_items(
        conn,
        "Security Basics",
        "Core security practices across all project types",
        "#6b7280",
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

    let cicd_id = add_shared_section_with_items(
        conn,
        "CI/CD Pipeline",
        "Automated build, test, and deploy",
        "#6b7280",
        &[
            ("CI pipeline configured", "Lint, test, build on every push"),
            ("Automated tests in CI", "Unit + integration tests"),
            ("Build artifact versioning", "Semantic versioning strategy"),
            ("Deployment pipeline defined", "Staging → production flow"),
            ("Rollback strategy defined", "How to revert a bad deploy"),
        ],
    )?;

    let planning_id = add_shared_section_with_items(
        conn,
        "Project Planning",
        "Requirements, architecture decisions, and setup",
        "#6b7280",
        &[
            (
                "Requirements documented",
                "User stories, acceptance criteria",
            ),
            ("Tech stack decided", "Languages, frameworks, libraries"),
            (
                "Architecture diagram created",
                "System components and data flow",
            ),
            ("Repository structure set up", "Monorepo vs separate repos"),
            (
                "Development environment configured",
                "IDE, linters, formatters",
            ),
        ],
    )?;

    let testing_id = add_shared_section_with_items(
        conn,
        "Testing & QA",
        "Test strategy and quality assurance",
        "#6b7280",
        &[
            ("Unit tests written", "Core logic coverage"),
            (
                "Integration tests configured",
                "API and service integration",
            ),
            ("E2E tests set up", "Critical user flows"),
            ("Performance benchmarks defined", "Load testing criteria"),
            (
                "Code review process established",
                "PR templates, checklists",
            ),
        ],
    )?;

    let docs_id = add_shared_section_with_items(
        conn,
        "Documentation",
        "Setup, API docs, and deployment guides",
        "#6b7280",
        &[
            ("README with setup instructions", "Quick start for new devs"),
            ("API documentation generated", "Swagger/OpenAPI or similar"),
            ("Deployment guide written", "Step-by-step deploy process"),
            ("Environment variables documented", "Example .env file"),
            ("Architecture decision records", "Key technical decisions"),
        ],
    )?;

    // --- Shared Sprints ---
    add_shared_sprint_with_sections(
        conn,
        "Planning & Setup",
        "Initial project setup and planning",
        0,
        &[planning_id],
    )?;
    add_shared_sprint_with_sections(
        conn,
        "Foundation & Architecture",
        "Core infrastructure and architecture",
        1,
        &[docker_id, security_id],
    )?;
    add_shared_sprint_with_sections(conn, "Core Development", "Main feature development", 2, &[])?;
    add_shared_sprint_with_sections(
        conn,
        "Testing & QA",
        "Testing and quality assurance",
        3,
        &[testing_id],
    )?;
    add_shared_sprint_with_sections(
        conn,
        "Deployment & Launch",
        "Deployment and production readiness",
        4,
        &[cicd_id, docs_id],
    )?;

    // --- Templates ---

    // Full Stack Web App
    conn.execute(
        "INSERT INTO templates (name, description, color) VALUES (?1, ?2, ?3)",
        (
            "Full Stack Web App",
            "React + Node / Next.js / Django — complete web app checklist",
            "#3b82f6",
        ),
    )?;
    let web_id = conn.last_insert_rowid();

    add_template_sprint_with_sections(
        conn,
        web_id,
        "Sprint 1: Planning & Setup",
        "Initial setup",
        0,
        &[planning_id],
    )?;
    add_template_sprint_with_sections(
        conn,
        web_id,
        "Sprint 2: Foundation",
        "Infrastructure and security",
        1,
        &[docker_id, security_id],
    )?;

    let web_api = add_template_sprint(
        conn,
        web_id,
        "Sprint 3: API Layer",
        "REST or GraphQL setup",
        2,
    )?;
    add_sprint_items(
        conn,
        web_api,
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

    let web_ui = add_template_sprint(
        conn,
        web_id,
        "Sprint 4: Frontend UI",
        "Components, state, and UX",
        3,
    )?;
    add_sprint_items(
        conn,
        web_ui,
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

    let web_int = add_template_sprint(
        conn,
        web_id,
        "Sprint 5: Integration",
        "Frontend-backend connection",
        4,
    )?;
    add_sprint_items(
        conn,
        web_int,
        &[
            ("API service layer (no direct fetch in components)", ""),
            ("Global error handler (401, 500, etc.)", ""),
            ("Token refresh logic", ""),
            ("Retry logic for failed requests", ""),
        ],
    )?;

    let web_db = add_template_sprint(conn, web_id, "Sprint 6: Database", "Schema and patterns", 5)?;
    add_sprint_items(
        conn,
        web_db,
        &[
            ("User-centric schema (accounts, sessions)", ""),
            ("Audit fields (created_at, updated_at)", ""),
            ("Soft deletes if needed", ""),
            ("Indexing for common queries", ""),
        ],
    )?;

    let web_perf = add_template_sprint(
        conn,
        web_id,
        "Sprint 7: Performance",
        "Web-specific optimizations",
        6,
    )?;
    add_sprint_items(
        conn,
        web_perf,
        &[
            ("Code splitting configured", ""),
            ("Lazy loading components", ""),
            ("Image optimization", ""),
            ("Bundle size checked", ""),
            ("Backend response time optimized", ""),
            ("Caching strategy (Redis / HTTP cache)", ""),
        ],
    )?;

    add_template_sprint_with_sections(
        conn,
        web_id,
        "Sprint 8: Testing & QA",
        "Testing and quality assurance",
        7,
        &[testing_id],
    )?;
    add_template_sprint_with_sections(
        conn,
        web_id,
        "Sprint 9: Deployment & Launch",
        "Deployment and production readiness",
        8,
        &[cicd_id, docs_id],
    )?;

    // Mobile App
    conn.execute(
        "INSERT INTO templates (name, description, color) VALUES (?1, ?2, ?3)",
        (
            "Mobile App",
            "React Native / Flutter / Swift / Kotlin — mobile checklist",
            "#8b5cf6",
        ),
    )?;
    let mobile_id = conn.last_insert_rowid();

    add_template_sprint_with_sections(
        conn,
        mobile_id,
        "Sprint 1: Planning & Setup",
        "Platform decisions and setup",
        0,
        &[planning_id],
    )?;

    let mobile_foundation = add_template_sprint(
        conn,
        mobile_id,
        "Sprint 2: Foundation",
        "Architecture and structure",
        1,
    )?;
    add_sprint_items(
        conn,
        mobile_foundation,
        &[
            ("Native vs Cross-platform decided", ""),
            ("Feature-based modules (not just components folder)", ""),
            ("Clear separation: UI / Logic / Data", ""),
        ],
    )?;

    let mobile_nav = add_template_sprint(
        conn,
        mobile_id,
        "Sprint 3: Navigation & Flow",
        "Screens and routing",
        2,
    )?;
    add_sprint_items(
        conn,
        mobile_nav,
        &[
            ("Navigation system (stack / tab / drawer)", ""),
            ("Deep linking configured", ""),
            ("Screen lifecycle awareness", ""),
            ("Auth flow (login → app → logout)", ""),
            ("Onboarding flow designed", ""),
            ("Error fallback screens", ""),
        ],
    )?;

    let mobile_ui = add_template_sprint(
        conn,
        mobile_id,
        "Sprint 4: UI / UX",
        "Mobile-specific design",
        3,
    )?;
    add_sprint_items(
        conn,
        mobile_ui,
        &[
            ("Responsive for different screen sizes", ""),
            ("Safe areas (notch, status bar)", ""),
            ("Keyboard handling", ""),
            ("Loading indicators", ""),
            ("Offline state UI", ""),
            ("Retry UI for failed actions", ""),
        ],
    )?;

    let mobile_api = add_template_sprint(
        conn,
        mobile_id,
        "Sprint 5: API & Data",
        "Network and offline",
        4,
    )?;
    add_sprint_items(
        conn,
        mobile_api,
        &[
            ("Central API service layer", ""),
            ("Request/response interceptors", ""),
            ("Token storage + refresh logic", ""),
            ("Local cache strategy (SQLite / AsyncStorage)", ""),
            ("Sync mechanism when back online", ""),
            ("Conflict resolution strategy", ""),
        ],
    )?;

    let mobile_storage = add_template_sprint(
        conn,
        mobile_id,
        "Sprint 6: Local Storage",
        "Persistent data",
        5,
    )?;
    add_sprint_items(
        conn,
        mobile_storage,
        &[
            ("Secure storage for tokens (keychain/keystore)", ""),
            ("Persistent app data strategy", ""),
            ("Cache invalidation rules", ""),
        ],
    )?;

    let mobile_device = add_template_sprint(
        conn,
        mobile_id,
        "Sprint 7: Device Integration",
        "Native features",
        6,
    )?;
    add_sprint_items(
        conn,
        mobile_device,
        &[
            ("Permissions defined (camera, location, storage)", ""),
            ("Push notifications (FCM / APNs)", ""),
            ("Background tasks configured", ""),
            ("App lifecycle handling", ""),
        ],
    )?;

    let mobile_perf = add_template_sprint(
        conn,
        mobile_id,
        "Sprint 8: Performance",
        "Mobile optimizations",
        7,
    )?;
    add_sprint_items(
        conn,
        mobile_perf,
        &[
            ("Avoid unnecessary re-renders", ""),
            ("Optimize lists (FlatList / RecyclerView)", ""),
            ("Image optimization", ""),
            ("Bundle size reduced", ""),
            ("Memory leak checks", ""),
        ],
    )?;

    let mobile_build = add_template_sprint(
        conn,
        mobile_id,
        "Sprint 9: Build & Distribution",
        "Store readiness",
        8,
    )?;
    add_sprint_items(
        conn,
        mobile_build,
        &[
            ("Android: APK/AAB build + signing", ""),
            ("iOS: Certificates & provisioning profiles", ""),
            ("Versioning strategy", ""),
            ("Release notes process", ""),
        ],
    )?;

    add_template_sprint_with_sections(
        conn,
        mobile_id,
        "Sprint 10: Testing & QA",
        "Testing and quality assurance",
        9,
        &[testing_id],
    )?;
    add_template_sprint_with_sections(
        conn,
        mobile_id,
        "Sprint 11: Deployment & Launch",
        "App store submission",
        10,
        &[cicd_id, docs_id],
    )?;

    // Desktop/Cross-Platform
    conn.execute(
        "INSERT INTO templates (name, description, color) VALUES (?1, ?2, ?3)",
        (
            "Desktop / Cross-Platform",
            "Tauri / Electron / native desktop checklist",
            "#f59e0b",
        ),
    )?;
    let desktop_id = conn.last_insert_rowid();

    add_template_sprint_with_sections(
        conn,
        desktop_id,
        "Sprint 1: Planning & Setup",
        "Platform decisions",
        0,
        &[planning_id],
    )?;

    let desktop_arch = add_template_sprint(
        conn,
        desktop_id,
        "Sprint 2: Architecture",
        "App structure",
        1,
    )?;
    add_sprint_items(
        conn,
        desktop_arch,
        &[
            ("Main process vs renderer separation", ""),
            ("IPC contract defined", ""),
            ("State management strategy", ""),
            ("Feature-based module organization", ""),
        ],
    )?;

    let desktop_ui = add_template_sprint(conn, desktop_id, "Sprint 3: UI Layer", "Desktop UI", 2)?;
    add_sprint_items(
        conn,
        desktop_ui,
        &[
            ("Window management (size, resize, min/max)", ""),
            ("System tray integration if needed", ""),
            ("Native menus and dialogs", ""),
            ("Dark mode support", ""),
            ("Accessibility (keyboard nav, screen readers)", ""),
        ],
    )?;

    let desktop_data = add_template_sprint(
        conn,
        desktop_id,
        "Sprint 4: Data Layer",
        "Local storage and sync",
        3,
    )?;
    add_sprint_items(
        conn,
        desktop_data,
        &[
            ("Local database configured (SQLite)", ""),
            ("File system access patterns", ""),
            ("Settings/preferences storage", ""),
            ("Cloud sync if needed", ""),
        ],
    )?;

    add_template_sprint_with_sections(
        conn,
        desktop_id,
        "Sprint 5: Security",
        "Desktop security",
        4,
        &[security_id],
    )?;

    let desktop_build = add_template_sprint(
        conn,
        desktop_id,
        "Sprint 6: Build & Distribution",
        "Packaging and release",
        5,
    )?;
    add_sprint_items(
        conn,
        desktop_build,
        &[
            ("Installer packaging (MSI / DMG / deb / rpm)", ""),
            ("Code signing certificates", ""),
            ("Auto-update channel configured", ""),
            ("Release notes process", ""),
        ],
    )?;

    let desktop_perf = add_template_sprint(
        conn,
        desktop_id,
        "Sprint 7: Performance",
        "Desktop optimizations",
        6,
    )?;
    add_sprint_items(
        conn,
        desktop_perf,
        &[
            ("Memory usage monitored", ""),
            ("Startup time optimized", ""),
            ("Bundle size checked", ""),
        ],
    )?;

    add_template_sprint_with_sections(
        conn,
        desktop_id,
        "Sprint 8: Testing & QA",
        "Testing and quality assurance",
        7,
        &[testing_id],
    )?;
    add_template_sprint_with_sections(
        conn,
        desktop_id,
        "Sprint 9: Deployment & Launch",
        "Release and distribution",
        8,
        &[cicd_id, docs_id],
    )?;

    Ok(())
}

// =======================================================================
// Helper functions
// =======================================================================

fn add_shared_section_with_items(
    conn: &Connection,
    name: &str,
    description: &str,
    color: &str,
    items: &[(&str, &str)],
) -> Result<i64, AppError> {
    conn.execute(
        "INSERT INTO shared_sections (name, description, color) VALUES (?1, ?2, ?3)",
        (name, description, color),
    )?;
    let section_id = conn.last_insert_rowid();
    for (i, (title, desc)) in items.iter().enumerate() {
        conn.execute(
            "INSERT INTO shared_section_items (section_id, title, description, sort_order) VALUES (?1, ?2, ?3, ?4)",
            (section_id, title, desc, i as i64),
        )?;
    }
    Ok(section_id)
}

fn add_shared_sprint_with_sections(
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

fn add_template_sprint(
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

fn add_template_sprint_with_sections(
    conn: &Connection,
    template_id: i64,
    name: &str,
    description: &str,
    sort_order: i64,
    section_ids: &[i64],
) -> Result<i64, AppError> {
    let sprint_id = add_template_sprint(conn, template_id, name, description, sort_order)?;
    for (i, section_id) in section_ids.iter().enumerate() {
        conn.execute(
            "INSERT INTO template_sprint_sections (sprint_id, section_id, sort_order, is_linked) VALUES (?1, ?2, ?3, 1)",
            (sprint_id, section_id, i as i64),
        )?;
    }
    Ok(sprint_id)
}

fn add_sprint_items(
    conn: &Connection,
    sprint_id: i64,
    items: &[(&str, &str)],
) -> Result<(), AppError> {
    // Create a shared section for these items
    let section_id = add_shared_section_with_items(
        conn,
        &format!("Section {}", sprint_id),
        "",
        "#6b7280",
        items,
    )?;
    // Link it to the template sprint
    let max_order: i64 = conn.query_row(
        "SELECT COALESCE(MAX(sort_order), -1) FROM template_sprint_sections WHERE sprint_id = ?1",
        [sprint_id],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO template_sprint_sections (sprint_id, section_id, sort_order, is_linked) VALUES (?1, ?2, ?3, 1)",
        (sprint_id, section_id, max_order + 1),
    )?;
    Ok(())
}
