use super::{
    add_custom_sprint_section, add_template, add_template_sprint, add_template_sprint_sections,
};
use super::super::super::types::AppError;
use rusqlite::Connection;

pub fn seed(
    conn: &Connection,
    section_map: &std::collections::HashMap<String, i64>,
    sprint_map: &std::collections::HashMap<String, i64>,
) -> Result<(), AppError> {
    let planning = section_map.get("planning").copied().unwrap_or(0);
    let security = section_map.get("security").copied().unwrap_or(0);
    let testing = section_map.get("testing").copied().unwrap_or(0);
    let cicd = section_map.get("cicd").copied().unwrap_or(0);
    let docs = section_map.get("docs").copied().unwrap_or(0);
    let quality = section_map.get("quality").copied().unwrap_or(0);
    let performance = section_map.get("performance").copied().unwrap_or(0);
    let monitoring = section_map.get("monitoring").copied().unwrap_or(0);
    let database = section_map.get("database").copied().unwrap_or(0);

    let planning_setup = sprint_map.get("planning_setup").copied().unwrap_or(0);
    let security_quality = sprint_map.get("security_quality").copied().unwrap_or(0);
    let testing_qa = sprint_map.get("testing_qa").copied().unwrap_or(0);
    let cicd_deploy = sprint_map.get("cicd_deploy").copied().unwrap_or(0);
    let monitoring_ops = sprint_map.get("monitoring_ops").copied().unwrap_or(0);
    let perf_sprint = sprint_map.get("performance").copied().unwrap_or(0);

    // === Desktop Application Development ===
    let tpl = add_template(
        conn,
        "Desktop Application",
        "Tauri / Electron / native — cross-platform desktop app checklist",
        "#f59e0b",
    )?;

    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Setup",
        "Project kickoff and dev environment",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning, quality])?;

    let s2 = add_template_sprint(
        conn,
        tpl,
        "App Architecture",
        "Window management, IPC, and process model",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Process Architecture",
        "Main/renderer separation and IPC design",
        "#f59e0b",
        &[
            (
                "Main process structure defined",
                "App lifecycle, window management, and system tray",
            ),
            (
                "Renderer process isolation configured",
                "Context isolation, sandbox mode, and preload scripts",
            ),
            (
                "IPC contract documented",
                "Channel names, payload schemas, and error handling",
            ),
            (
                "State synchronization strategy chosen",
                "Event-based, polling, or shared storage",
            ),
            (
                "Security model defined",
                "Node integration disabled, CSP configured, protocol handler set",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Window & UI Framework",
        "Native window management and UI toolkit",
        "#f97316",
        &[
            (
                "Window configuration defined",
                "Size, min/max, resizable, frameless, transparency",
            ),
            (
                "UI framework selected and integrated",
                "React, Vue, Svelte, or native UI toolkit",
            ),
            (
                "Multi-window support planned",
                "Parent-child windows, modal dialogs, and window communication",
            ),
            (
                "System tray integration configured",
                "Tray icon, context menu, and click handlers",
            ),
            (
                "Native menu bar implemented",
                "Application menu with keyboard shortcuts",
            ),
        ],
    )?;

    let s3 = add_template_sprint(
        conn,
        tpl,
        "Core Features",
        "Main application functionality",
        2,
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "File System Integration",
        "File operations and native dialogs",
        "#06b6d4",
        &[
            (
                "File open/save dialogs implemented",
                "Native file picker with filter support",
            ),
            (
                "File read/write operations secured",
                "Scoped file access with user consent",
            ),
            (
                "Drag and drop support added",
                "File drop zones with validation and feedback",
            ),
            (
                "Recent files list maintained",
                "App-level recent documents with quick access",
            ),
            (
                "File association configured",
                "Double-click file opens app with document",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Native APIs",
        "OS-specific capabilities and integrations",
        "#14b8a6",
        &[
            (
                "Clipboard access implemented",
                "Read/write clipboard with format detection",
            ),
            (
                "System notifications configured",
                "Native notification API with action buttons",
            ),
            (
                "Global keyboard shortcuts registered",
                "App-wide hotkeys with conflict detection",
            ),
            (
                "Native dialog boxes implemented",
                "Alert, confirm, and prompt dialogs",
            ),
            (
                "OS integration features added",
                "Dock/taskbar progress, badge counts, jump lists",
            ),
        ],
    )?;

    let s4 = add_template_sprint(
        conn,
        tpl,
        "Data & Storage",
        "Local database and preferences",
        3,
    )?;
    add_template_sprint_sections(conn, s4, &[database])?;
    add_custom_sprint_section(
        conn,
        s4,
        "App Preferences",
        "User settings and configuration",
        "#8b5cf6",
        &[
            (
                "Settings storage mechanism chosen",
                "JSON file, SQLite, or platform preferences API",
            ),
            (
                "User preferences UI built",
                "Settings window with categories and search",
            ),
            (
                "Theme/appearance settings implemented",
                "Light, dark, and system theme with persistence",
            ),
            (
                "Keyboard shortcut customization added",
                "User-remappable shortcuts with conflict resolution",
            ),
            (
                "Settings import/export implemented",
                "Backup and restore user configuration",
            ),
        ],
    )?;

    let s5 = add_template_sprint(
        conn,
        tpl,
        "Security Hardening",
        "Desktop-specific security measures",
        4,
    )?;
    add_template_sprint_sections(conn, s5, &[security])?;
    add_custom_sprint_section(
        conn,
        s5,
        "Desktop Security",
        "App-level security for desktop environments",
        "#ef4444",
        &[
            (
                "Content security policy enforced",
                "Strict CSP for renderer processes",
            ),
            (
                "Node.js APIs restricted",
                "Only exposed APIs through preload script",
            ),
            (
                "Protocol handler validated",
                "Custom URL scheme with input sanitization",
            ),
            (
                "Local server secured",
                "If using local HTTP server, auth and CORS configured",
            ),
            (
                "Sensitive data not stored in plaintext",
                "Encrypted storage for passwords and tokens",
            ),
        ],
    )?;

    let s6 = add_template_sprint(
        conn,
        tpl,
        "Auto-Update System",
        "Automatic updates and version management",
        5,
    )?;
    add_custom_sprint_section(
        conn,
        s6,
        "Update Infrastructure",
        "Update server, channels, and delivery",
        "#6366f1",
        &[
            (
                "Update server configured",
                "Release channel server with version manifest",
            ),
            (
                "Update check mechanism implemented",
                "Periodic check with manual check option",
            ),
            (
                "Download and install flow built",
                "Background download with progress indicator",
            ),
            (
                "Rollback on failed update",
                "Previous version preserved for fallback",
            ),
            (
                "Release notes displayed",
                "Changelog shown before or after update",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s6,
        "Build & Packaging",
        "Platform-specific builds and installers",
        "#f59e0b",
        &[
            (
                "Windows installer created",
                "MSI or NSIS with silent install support",
            ),
            (
                "macOS app bundle and DMG created",
                "Notarized and signed for Gatekeeper",
            ),
            ("Linux packages created", "AppImage, deb, and rpm formats"),
            (
                "Code signing configured",
                "Certificates for all target platforms",
            ),
            (
                "Build automation in CI",
                "Cross-platform builds triggered on release tag",
            ),
        ],
    )?;

    let s7 = add_template_sprint(
        conn,
        tpl,
        "Testing & QA",
        "Desktop-specific testing strategy",
        6,
    )?;
    add_template_sprint_sections(conn, s7, &[testing])?;
    add_custom_sprint_section(
        conn,
        s7,
        "Desktop Testing",
        "Platform-specific and integration testing",
        "#10b981",
        &[
            (
                "IPC communication tested",
                "Main-to-renderer and renderer-to-main message flows",
            ),
            (
                "File operations tested across platforms",
                "Path handling, permissions, and edge cases",
            ),
            (
                "Window behavior tested",
                "Resize, minimize, maximize, and multi-monitor",
            ),
            (
                "Auto-update flow tested",
                "Check, download, install, and rollback scenarios",
            ),
            (
                "Crash recovery tested",
                "App restart with state preservation after crash",
            ),
        ],
    )?;

    let s8 = add_template_sprint(
        conn,
        tpl,
        "Performance Optimization",
        "Memory, startup, and resource usage",
        7,
    )?;
    add_template_sprint_sections(conn, s8, &[performance])?;
    add_custom_sprint_section(
        conn,
        s8,
        "Desktop Performance",
        "Resource optimization for desktop apps",
        "#f97316",
        &[
            (
                "Memory usage profiled",
                "Heap analysis and memory leak detection",
            ),
            (
                "Startup time optimized",
                "Lazy loading and deferred initialization",
            ),
            (
                "Bundle size minimized",
                "Tree-shaking, code splitting, and asset optimization",
            ),
            (
                "CPU usage during idle measured",
                "Background tasks optimized for low CPU",
            ),
            (
                "GPU acceleration verified",
                "Hardware acceleration for rendering where applicable",
            ),
        ],
    )?;

    let s9 = add_template_sprint(
        conn,
        tpl,
        "CI/CD & Deployment",
        "Automated build and release pipeline",
        8,
    )?;
    add_template_sprint_sections(conn, s9, &[cicd, docs])?;

    let s10 = add_template_sprint(
        conn,
        tpl,
        "Monitoring & Operations",
        "Crash reporting and production monitoring",
        9,
    )?;
    add_template_sprint_sections(conn, s10, &[monitoring, docs])?;

    Ok(())
}
