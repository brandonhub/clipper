use std::thread;
use crate::monitor_processes::monitor_processes;
use crate::commands::greet;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            // Spawn the monitor in a separate thread so it doesn't block
            thread::spawn(|| {
                monitor_processes();
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())

        // TODO: Add auto start plugin set up here
        // https://v2.tauri.app/plugin/autostart/

        // Register all commands to be exposed to the web side
        .invoke_handler(tauri::generate_handler![greet])

        // Run the application
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}