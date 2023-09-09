// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;
mod queries;
mod recorder;
mod schema;
mod server;

use std::thread;

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let boxed_handle = Box::new(handle);

            thread::spawn(move || {
                db::init();
                server::spectator::init(*boxed_handle).unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::record_commands::record,
            commands::record_commands::record_custom_endpoint,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
