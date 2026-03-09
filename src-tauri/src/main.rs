// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Set PROJ paths before Tauri initializes - must happen before any PROJ call
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let proj_path = exe_dir.join("proj");
            std::env::set_var("PROJ_LIB", &proj_path);
            std::env::set_var("PROJ_DATA", &proj_path);
        }
    }
    uavsar_lib::run()
}
