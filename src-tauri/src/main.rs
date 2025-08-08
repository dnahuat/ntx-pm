#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

fn main() {
    // Initialize configuration at startup so it is available application-wide.
    config::init();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
