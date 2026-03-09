mod flight_path;
mod writer;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let proj_data = app.path().resource_dir()
                .expect("Failed to get resource dir")
                .join("proj");
    
            // Strip \\?\ prefix - Windows extended path format that PROJ can't handle
            let proj_str = proj_data.to_string_lossy().to_string();
            let proj_clean = proj_str.strip_prefix(r"\\?\").unwrap_or(&proj_str).to_string();
    
            std::env::set_var("PROJ_LIB", proj_clean);
            std::env::set_var("PROJ_DATA", &proj_clean);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![flight_path::generate_flightpath])
        .run(context)
        .expect("error while running tauri application");
}
