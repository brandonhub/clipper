use sysinfo::System;
use std::{thread, time::Duration};
use crate::is_game::is_game;

pub fn monitor_processes() {
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