// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod autogen{
    pub mod constants;
}

mod states{
    pub mod states;
}

use crate::states::states::subscribe_state_events;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            subscribe_state_events(app.handle());
            Ok(())
        })
        .manage(states::states::set_default_state())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
