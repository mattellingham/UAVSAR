#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let proj_path = exe_dir.join("proj");
            let proj_str = proj_path.to_string_lossy().to_string();

            // Set env vars as fallback
            std::env::set_var("PROJ_LIB", &proj_str);
            std::env::set_var("PROJ_DATA", &proj_str);

            // Force path directly via C API on the default context (NULL = default)
            unsafe {
                let c_str = std::ffi::CString::new(proj_str).unwrap();
                let ptrs = [c_str.as_ptr()];
                proj_sys::proj_context_set_search_paths(
                    std::ptr::null_mut(),
                    1,
                    ptrs.as_ptr(),
                );
            }
        }
    }
    uavsar_lib::run()
}
