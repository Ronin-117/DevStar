pub mod db;
pub mod rate_limit;

use db::{Database, TemplateInput, TemplateSectionInput, TemplateItemInput, ProjectInput, ProjectItemUpdate, ProjectSectionInput, ProjectItemInput};
use rate_limit::RateLimiter;
use tauri::Manager;

struct AppState {
    db: Database,
    rate_limiter: RateLimiter,
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
) -> Result<Vec<db::Template>, String> {
    check_rate_limit(&state, window.label())?;
    state.db.list_templates().map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_template(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: TemplateInput,
) -> Result<db::Template, String> {
    check_rate_limit(&state, window.label())?;
    state.db.create_template(input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_template(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
    name: Option<String>,
    description: Option<String>,
    color: Option<String>,
) -> Result<db::Template, String> {
    check_rate_limit(&state, window.label())?;
    state.db.update_template(id, name, description, color).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_template(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    state.db.delete_template(id).map_err(|e| e.to_string())
}

// --- Template section commands ---

#[tauri::command]
async fn list_template_sections_with_items(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    template_id: i64,
) -> Result<Vec<db::SectionWithItems>, String> {
    check_rate_limit(&state, window.label())?;
    state.db.list_template_sections_with_items(template_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_template_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: TemplateSectionInput,
) -> Result<db::TemplateSection, String> {
    check_rate_limit(&state, window.label())?;
    state.db.add_template_section(input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_template_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
    name: Option<String>,
    description: Option<String>,
) -> Result<db::TemplateSection, String> {
    check_rate_limit(&state, window.label())?;
    state.db.update_template_section(id, name, description).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_template_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    state.db.delete_template_section(id).map_err(|e| e.to_string())
}

// --- Template item commands ---

#[tauri::command]
async fn add_template_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: TemplateItemInput,
) -> Result<db::TemplateItem, String> {
    check_rate_limit(&state, window.label())?;
    state.db.add_template_item(input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_template_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
    title: Option<String>,
    description: Option<String>,
) -> Result<db::TemplateItem, String> {
    check_rate_limit(&state, window.label())?;
    state.db.update_template_item(id, title, description).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_template_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    state.db.delete_template_item(id).map_err(|e| e.to_string())
}

// --- Project commands ---

#[tauri::command]
async fn list_projects(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<Vec<db::Project>, String> {
    check_rate_limit(&state, window.label())?;
    state.db.list_projects().map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_project_from_template(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: ProjectInput,
) -> Result<db::Project, String> {
    check_rate_limit(&state, window.label())?;
    state.db.create_project_from_template(input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_project(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    state.db.delete_project(id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_project_progress(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    project_id: i64,
) -> Result<(i64, i64), String> {
    check_rate_limit(&state, window.label())?;
    state.db.get_project_progress(project_id).map_err(|e| e.to_string())
}

// --- Project section/item commands ---

#[tauri::command]
async fn list_project_sections_with_items(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    project_id: i64,
) -> Result<Vec<db::ProjectSectionWithItems>, String> {
    check_rate_limit(&state, window.label())?;
    state.db.list_project_sections_with_items(project_id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_project_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: ProjectItemUpdate,
) -> Result<db::ProjectItem, String> {
    check_rate_limit(&state, window.label())?;
    state.db.update_project_item(input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_project_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: ProjectSectionInput,
) -> Result<db::ProjectSection, String> {
    check_rate_limit(&state, window.label())?;
    state.db.add_project_section(input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_project_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    input: ProjectItemInput,
) -> Result<db::ProjectItem, String> {
    check_rate_limit(&state, window.label())?;
    state.db.add_project_item(input).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_project_item(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    state.db.delete_project_item(id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_project_section(
    state: tauri::State<'_, AppState>,
    window: tauri::Window,
    id: i64,
) -> Result<(), String> {
    check_rate_limit(&state, window.label())?;
    state.db.delete_project_section(id).map_err(|e| e.to_string())
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
                // Get primary monitor for positioning
                let primary_monitor = app.primary_monitor().ok().flatten();
                let initial_position: Option<(i32, i32)> = primary_monitor.as_ref().map(|m| {
                    let size = m.size();
                    let padding = 16;
                    (
                        (size.width as i32) - 340 - padding,
                        padding,
                    )
                });

                let mut builder = tauri::WebviewWindowBuilder::new(
                    &app,
                    "active",
                    tauri::WebviewUrl::App("index.html".into()),
                )
                .title("ProjectTracker - Live")
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
        .join("com.njne2.projecttracker");
    std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
    let db_path = app_data_dir.join("projecttracker.db");
    let db_path_str = db_path.to_string_lossy().to_string();

    let db = Database::new(&db_path_str).expect("Failed to initialize database");
    db.seed_if_empty().expect("Failed to seed database");
    let rate_limiter = RateLimiter::new(30.0, 5.0);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { db, rate_limiter })
        .invoke_handler(tauri::generate_handler![
            list_templates,
            create_template,
            update_template,
            delete_template,
            list_template_sections_with_items,
            add_template_section,
            update_template_section,
            delete_template_section,
            add_template_item,
            update_template_item,
            delete_template_item,
            list_projects,
            create_project_from_template,
            delete_project,
            get_project_progress,
            list_project_sections_with_items,
            update_project_item,
            add_project_section,
            add_project_item,
            delete_project_item,
            delete_project_section,
            toggle_mode,
            get_window_label,
            resize_active_window,
            close_window,
            minimize_window,
            toggle_maximize_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
