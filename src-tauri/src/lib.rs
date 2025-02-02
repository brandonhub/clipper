use sysinfo::{System, Process};
use std::{thread, time::Duration};

fn is_game(process: &Process) -> bool {
    process.exe()
        .and_then(|path | path.file_name())
        .and_then(|file_name| file_name.to_str())
        .map(|file_name_str| file_name_str == "exe")
        .unwrap_or(false)
}

fn monitor_processes() {
    loop {
        let system = System::new_all();
        let game_running = system.processes()
            .values()
            .any(|process| is_game(process));

        if game_running {
            println!("Game is running!");
        } else {
            println!("Game is not running.");
        }

        // Wait 5 seconds before pulling processes again
        thread::sleep(Duration::from_secs(5));
    }

}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
