#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::let_underscore_future)]
#![allow(clippy::unnecessary_cast)]

pub mod db;
pub mod rate_limit;

use db::Database;
use db::types::*;
use rate_limit::RateLimiter;
use std::sync::Mutex;
use tauri::Manager;
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState};
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};

struct AppState {
    db: Database,
    rate_limiter: RateLimiter,
    mcp_process: Mutex<Option<std::process::Child>>,
}

fn check_rate_limit(state: &tauri::State<AppState>, window_label: &str) -> Result<(), String> {
    if !state.rate_limiter.allow(window_label) {
        return Err("Rate limit exceeded. Try again later.".into());
    }
    Ok(())
}

// --- Template commands ---

#[tauri::command]
async fn list_templates(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<Vec<Template>, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::templates::list(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_template(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: TemplateInput,
) -> Result<Template, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::templates::create(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_template(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
    name: Option<String>,
    description: Option<String>,
    color: Option<String>,
) -> Result<Template, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::templates::update(&conn, id, name, description, color).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_template(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::templates::delete(&conn, id).map_err(|e| e.to_string())
}

// --- Template Sprint commands ---

#[tauri::command]
async fn list_template_sprints(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    template_id: i64,
) -> Result<Vec<TemplateSprint>, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::template_sprints::list(&conn, template_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_template_sprint_with_sections(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    sprint_id: i64,
) -> Result<TemplateSprintWithSections, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::template_sprints::get_with_sections(&conn, sprint_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_template_sprint(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    template_id: i64,
    name: String,
    description: String,
) -> Result<TemplateSprint, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::template_sprints::add(&conn, template_id, name, description).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_template_sprint(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
    name: Option<String>,
    description: Option<String>,
) -> Result<TemplateSprint, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::template_sprints::update(&conn, id, name, description).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_template_sprint(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::template_sprints::delete(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_template_sprint_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    sprint_id: i64,
    section_id: i64,
    is_linked: bool,
) -> Result<TemplateSprintSection, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::template_sprints::add_section(&conn, sprint_id, section_id, is_linked).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_template_sprint_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::template_sprints::delete_section(&conn, id).map_err(|e| e.to_string())
}

// --- Shared Section commands ---

#[tauri::command]
async fn list_shared_sections(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<Vec<SharedSection>, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sections::list(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_shared_section_with_items(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    section_id: i64,
) -> Result<SectionWithItems, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sections::get_with_items(&conn, section_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_shared_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: SharedSectionInput,
) -> Result<SharedSection, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sections::create(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_shared_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
    name: Option<String>,
    description: Option<String>,
    color: Option<String>,
) -> Result<SharedSection, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sections::update(&conn, id, name, description, color).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_shared_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sections::delete(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_shared_section_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: SharedSectionItemInput,
) -> Result<SharedSectionItem, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sections::add_item(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_shared_section_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
    title: Option<String>,
    description: Option<String>,
) -> Result<SharedSectionItem, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sections::update_item(&conn, id, title, description).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_shared_section_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sections::delete_item(&conn, id).map_err(|e| e.to_string())
}

// --- Shared Sprint commands ---

#[tauri::command]
async fn list_shared_sprints(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<Vec<SharedSprint>, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sprints::list(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_shared_sprint_with_sections(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    sprint_id: i64,
) -> Result<SharedSprintWithSections, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sprints::get_with_sections(&conn, sprint_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_shared_sprint(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: SharedSprintInput,
) -> Result<SharedSprint, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sprints::create(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_shared_sprint(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
    name: Option<String>,
    description: Option<String>,
) -> Result<SharedSprint, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sprints::update(&conn, id, name, description).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_shared_sprint(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sprints::delete(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_shared_sprint_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: SharedSprintSectionInput,
) -> Result<SharedSprintSection, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sprints::add_section(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_shared_sprint_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::shared_sprints::delete_section(&conn, id).map_err(|e| e.to_string())
}

// --- Project commands ---

#[tauri::command]
async fn list_projects(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<Vec<Project>, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::projects::list(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_project_from_template(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: ProjectInput,
) -> Result<Project, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::projects::create_from_template(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_project(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::projects::delete(&conn, id).map_err(|e| e.to_string())
}

// --- Project Sprint commands ---

#[tauri::command]
async fn list_project_sprints(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    project_id: i64,
) -> Result<Vec<ProjectSprintWithSections>, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::list_with_sections(&conn, project_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_sprint_status(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    sprint_id: i64,
    status: String,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::set_status(&conn, sprint_id, status).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_active_sprint(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    project_id: i64,
) -> Result<Option<ProjectSprint>, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::get_active(&conn, project_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_project_progress(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    project_id: i64,
) -> Result<(i64, i64), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::get_progress(&conn, project_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_project_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: ProjectItemUpdate,
) -> Result<ProjectItem, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::update_item(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn toggle_project_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<ProjectItem, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::toggle_item(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_and_advance_sprint(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    project_id: i64,
) -> Result<Option<ProjectSprint>, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::check_and_advance_sprint(&conn, project_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn complete_sprint(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    sprint_id: i64,
    project_id: i64,
) -> Result<Option<ProjectSprint>, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    // Mark all items in this sprint as checked
    db::project_sprints::complete_all_items(&conn, sprint_id).map_err(|e| e.to_string())?;
    // Mark sprint as done and advance to next
    db::project_sprints::check_and_advance_sprint(&conn, project_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_project_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: ProjectSectionInput,
) -> Result<ProjectSprintSection, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::add_section(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_project_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: ProjectItemInput,
) -> Result<ProjectItem, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::add_item(&conn, input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_project_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::delete_item(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_project_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::delete_section(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_project_sprint(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    project_id: i64,
    name: String,
    description: String,
) -> Result<ProjectSprint, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::add_sprint(&conn, project_id, &name, &description).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_shared_sprint_to_project(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    project_id: i64,
    shared_sprint_id: i64,
    is_linked: bool,
) -> Result<ProjectSprint, String> {
    check_rate_limit(&state, window.label())?;
    let conn = state.db.conn.lock().unwrap();
    db::project_sprints::add_shared_sprint_to_project(&conn, project_id, shared_sprint_id, is_linked)
        .map_err(|e| e.to_string())
}

// --- Window management ---

#[derive(Debug, serde::Deserialize)]
enum WindowMode {
    Management,
    Active,
}

#[tauri::command]
async fn toggle_mode(
    app: tauri::AppHandle,
    mode: WindowMode,
) -> Result<(), String> {
    eprintln!("[toggle_mode] called with mode: {:?}", mode);
    match mode {
        WindowMode::Management => {
            eprintln!("[toggle_mode] switching to management");
            if let Some(w) = app.get_webview_window("management") {
                eprintln!("[toggle_mode] showing management window");
                w.show().map_err(|e| e.to_string())?;
                w.set_focus().map_err(|e| e.to_string())?;
            } else {
                eprintln!("[toggle_mode] management window not found!");
            }
            if let Some(w) = app.get_webview_window("active") {
                eprintln!("[toggle_mode] hiding active window");
                w.hide().map_err(|e| e.to_string())?;
            }
        }
        WindowMode::Active => {
            eprintln!("[toggle_mode] switching to active");
            if let Some(w) = app.get_webview_window("active") {
                eprintln!("[toggle_mode] showing existing active window");
                w.show().map_err(|e| e.to_string())?;
                w.set_focus().map_err(|e| e.to_string())?;
            } else {
                eprintln!("[toggle_mode] creating new active window");
                let primary_monitor = app.primary_monitor().ok().flatten();
                let initial_position: Option<(i32, i32)> = primary_monitor.as_ref().map(|m| {
                    let size = m.size();
                    let padding = 16;
                    ((size.width as i32) - 340 - padding, padding)
                });

                let mut builder = tauri::WebviewWindowBuilder::new(
                    &app,
                    "active",
                    tauri::WebviewUrl::App("index.html".into()),
                )
                .title("DevStar - Live")
                .inner_size(340.0, 500.0)
                .always_on_top(true)
                .decorations(false)
                .transparent(true)
                .resizable(false);

                if let Some((x, y)) = initial_position {
                    builder = builder.position(x as f64, y as f64);
                }

                let result = builder.build();
                match result {
                    Ok(w) => {
                        eprintln!("[toggle_mode] active window created successfully");
                        w.show().map_err(|e| e.to_string())?;
                        w.set_focus().map_err(|e| e.to_string())?;
                    }
                    Err(e) => {
                        eprintln!("[toggle_mode] failed to create active window: {:?}", e);
                        return Err(format!("Failed to create window: {}", e));
                    }
                }
            }
            if let Some(w) = app.get_webview_window("management") {
                eprintln!("[toggle_mode] hiding management window");
                w.hide().map_err(|e| e.to_string())?;
            }
        }
    }
    eprintln!("[toggle_mode] done");
    Ok(())
}

#[tauri::command]
async fn get_window_label(window: tauri::Window) -> String {
    window.label().to_string()
}

#[tauri::command]
async fn resize_active_window(
    app: tauri::AppHandle,
    width: f64,
    height: f64,
) -> Result<(), String> {
    if let Some(w) = app.get_webview_window("active") {
        let monitor = w.current_monitor().map_err(|e| e.to_string())?;
        if let Some(m) = monitor {
            let size = m.size();
            let padding = 16.0;
            let new_x = (size.width as f64 - width - padding) as i32;
            let new_y = padding as i32;
            w.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(new_x, new_y)))
                .map_err(|e| e.to_string())?;
        }
        w.set_size(tauri::Size::Physical(tauri::PhysicalSize {
            width: width as u32,
            height: height as u32,
        }))
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn close_window(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[tauri::command]
async fn minimize_window(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_active_window_compact(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_webview_window("active") {
        let monitor = w.current_monitor().map_err(|e| e.to_string())?;
        if let Some(m) = monitor {
            let size = m.size();
            let btn_size = 48.0;
            let padding = 16.0;
            let new_x = (size.width as f64 - btn_size - padding) as i32;
            let new_y = padding as i32;
            w.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(new_x, new_y)))
                .map_err(|e| e.to_string())?;
        }
        w.set_size(tauri::Size::Physical(tauri::PhysicalSize {
            width: 56,
            height: 56,
        }))
        .map_err(|e| e.to_string())?;
        let _ = w.eval(
            "document.documentElement.style.overflow='hidden';\
             document.body.style.overflow='hidden';\
             document.body.style.margin='0';\
             document.body.style.padding='0';\
             document.body.style.background='#4f46e5';\
             document.querySelectorAll('*').forEach(function(el){\
               el.style.scrollbarWidth='none';\
               el.style.msOverflowStyle='none';\
             });\
             var style=document.createElement('style');\
             style.textContent='::-webkit-scrollbar{display:none!important;width:0!important;height:0!important}';\
             document.head.appendChild(style);"
        );
    }
    Ok(())
}

#[tauri::command]
async fn set_active_window_full(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_webview_window("active") {
        let monitor = w.current_monitor().map_err(|e| e.to_string())?;
        if let Some(m) = monitor {
            let size = m.size();
            let win_w = 340.0;
            let btn_size = 48.0;
            let padding = 16.0;
            let btn_x = (size.width as f64 - btn_size - padding) as i32;
            let new_x = btn_x - (win_w as i32) - 8;
            let new_y = padding as i32;
            let clamped_x = if new_x < 0 { padding as i32 } else { new_x };
            w.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(clamped_x, new_y)))
                .map_err(|e| e.to_string())?;
        }
        w.set_size(tauri::Size::Physical(tauri::PhysicalSize {
            width: 340,
            height: 500,
        }))
        .map_err(|e| e.to_string())?;
        // Restore scrollbars for the content area
        let _ = w.eval(
            "document.documentElement.style.overflow='';\
             document.body.style.overflow='';"
        );
    }
    Ok(())
}

#[tauri::command]
async fn toggle_maximize_window(window: tauri::Window) -> Result<(), String> {
    if window.is_maximized().unwrap_or(false) {
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("com.njne2.devstar");
    std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
    let db_path = app_data_dir.join("devstar.db");
    let db_path_str = db_path.to_string_lossy().to_string();

    let db = Database::new(&db_path_str).expect("Failed to initialize database");
    {
        let conn = db.conn.lock().unwrap();
        // Only seed if DB is empty (first run)
        let template_count: i64 = conn.query_row("SELECT count(*) FROM templates", [], |r| r.get(0)).unwrap_or(0);
        if template_count == 0 {
            db::seeds::seed_all(&conn).expect("Failed to seed database");
        }
    }
    let rate_limiter = RateLimiter::new(30.0, 5.0);

    // Add to Windows startup on first run
    setup_startup_auto();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState { db, rate_limiter, mcp_process: Mutex::new(None) })
        .setup(|app| {
            // Spawn MCP server as background process
            spawn_mcp_server(app.handle())?;

            // Create system tray with menu
            let handle = app.handle();
            let open_item = MenuItem::with_id(handle, "open", "Open DevStar", true, None::<&str>)?;
            let live_item = MenuItem::with_id(handle, "live", "Live Mode", true, None::<&str>)?;
            let sep = PredefinedMenuItem::separator(handle)?;
            let quit_item = MenuItem::with_id(handle, "quit", "Stop DevStar", true, None::<&str>)?;
            let menu = Menu::with_items(handle, &[&open_item, &live_item, &sep, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("DevStar")
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        if let Some(w) = app.get_webview_window("management") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                    "live" => {
                        let app_clone = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let _ = toggle_mode(app_clone, WindowMode::Active).await;
                        });
                    }
                    "quit" => {
                        // Kill MCP server
                        let state = app.state::<AppState>();
                        let mut proc_lock = state.mcp_process.lock().unwrap();
                        if let Some(mut child) = proc_lock.take() {
                            let _ = child.kill();
                            eprintln!("[MCP] Server stopped");
                        }
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("management") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_templates,
            create_template,
            update_template,
            delete_template,
            list_template_sprints,
            get_template_sprint_with_sections,
            add_template_sprint,
            update_template_sprint,
            delete_template_sprint,
            add_template_sprint_section,
            delete_template_sprint_section,
            list_shared_sections,
            get_shared_section_with_items,
            create_shared_section,
            update_shared_section,
            delete_shared_section,
            add_shared_section_item,
            update_shared_section_item,
            delete_shared_section_item,
            list_shared_sprints,
            get_shared_sprint_with_sections,
            create_shared_sprint,
            update_shared_sprint,
            delete_shared_sprint,
            add_shared_sprint_section,
            delete_shared_sprint_section,
            list_projects,
            create_project_from_template,
            delete_project,
            list_project_sprints,
            set_sprint_status,
            get_active_sprint,
            get_project_progress,
            update_project_item,
            toggle_project_item,
            check_and_advance_sprint,
            complete_sprint,
            add_project_section,
            add_project_item,
            delete_project_item,
            delete_project_section,
            add_project_sprint,
            add_shared_sprint_to_project,
            toggle_mode,
            get_window_label,
            resize_active_window,
            close_window,
            minimize_window,
            toggle_maximize_window,
            set_active_window_compact,
            set_active_window_full,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "management" {
                    // Don't quit, just hide the window
                    api.prevent_close();
                    let _ = window.hide();
                }
                // Active window can close normally
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Spawn the MCP server as a background child process
fn spawn_mcp_server(app: &tauri::AppHandle) -> Result<(), String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;
    let exe_dir = exe_path.parent()
        .ok_or("Failed to get exe directory")?;
    let mcp_path = exe_dir.join("devstar-mcp.exe");

    if !mcp_path.exists() {
        eprintln!("[MCP] devstar-mcp.exe not found at {:?}", mcp_path);
        return Ok(());
    }

    // On Windows, use CREATE_NO_WINDOW flag to hide the console
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        let mcp = std::process::Command::new(&mcp_path)
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();

        match mcp {
            Ok(child) => {
                eprintln!("[MCP] Server spawned with PID: {}", child.id());
                let state = app.state::<AppState>();
                let mut proc_lock = state.mcp_process.lock().unwrap();
                *proc_lock = Some(child);
            }
            Err(e) => {
                eprintln!("[MCP] Failed to spawn: {}", e);
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mcp = std::process::Command::new(&mcp_path)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();

        match mcp {
            Ok(child) => {
                eprintln!("[MCP] Server spawned with PID: {}", child.id());
                let state = app.state::<AppState>();
                let mut proc_lock = state.mcp_process.lock().unwrap();
                *proc_lock = Some(child);
            }
            Err(e) => {
                eprintln!("[MCP] Failed to spawn: {}", e);
            }
        }
    }

    Ok(())
}

/// Add DevStar to Windows startup
#[cfg(target_os = "windows")]
fn setup_startup_auto() {
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        r"Software\Microsoft\Windows\CurrentVersion\Run",
        winreg::enums::KEY_WRITE
    );

    if let Ok(run) = run_key {
        // Check if already set
        let existing: Result<String, _> = run.get_value("DevStar");
        if existing.is_err() {
            // Get current exe path
            if let Ok(exe_path) = std::env::current_exe() {
                let path_str = exe_path.to_string_lossy().to_string();
                let _ = run.set_value("DevStar", &path_str);
                eprintln!("[Startup] Added DevStar to startup: {}", path_str);
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn setup_startup_auto() {
    // Linux: create XDG autostart .desktop file
    if let Some(config_dir) = dirs::config_dir() {
        let autostart_dir = config_dir.join("autostart");
        let _ = std::fs::create_dir_all(&autostart_dir);
        let desktop_file = autostart_dir.join("devstar.desktop");
        if !desktop_file.exists() {
            if let Ok(exe_path) = std::env::current_exe() {
                let content = format!(
                    "[Desktop Entry]\nType=Application\nName=DevStar\nExec={}\nHidden=false\nNoDisplay=false\nX-GNOME-Autostart-enabled=true\n",
                    exe_path.to_string_lossy()
                );
                let _ = std::fs::write(&desktop_file, content);
                eprintln!("[Startup] Created autostart entry: {:?}", desktop_file);
            }
        }
    }
}
