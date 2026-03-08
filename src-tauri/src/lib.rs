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
            std::env::set_var("PROJ_LIB", proj_data);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![flight_path::generate_flightpath])
        .run(context)
        .expect("error while running tauri application");
}
