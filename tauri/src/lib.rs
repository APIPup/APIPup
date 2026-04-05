mod commands;
mod services;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(services::project::AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::http::send_request,
            commands::project::open_project,
            commands::project::get_cli_project_path,
            commands::project::scan_openapi_files,
            commands::project::select_files,
            commands::project::save_pup_config,
            commands::spec::load_spec,
            commands::spec::save_operation
        ])
        .run(tauri::generate_context!())
        .expect("error while running APIPup");
}
