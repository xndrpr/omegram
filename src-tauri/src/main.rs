#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod models;
pub mod tg;

use std::env;

fn initialize() {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "error");

    pretty_env_logger::init();
}
fn main() {
    initialize();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::check_auth,
            commands::request_code,
            commands::sign_in,
            commands::get_dialogs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
