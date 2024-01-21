#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod constants;
pub mod models;
pub mod services;

use std::env;

use constants::{CLIENT, DB};
use grammers_client::{Client, Config};
use grammers_session::Session;
use services::db::Database;

async fn initialize() {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "error");

    pretty_env_logger::init();

    let api_id = env::var("APP_ID").unwrap().parse().unwrap();
    let api_hash = env::var("APP_HASH").unwrap().to_string();

    let mut client = CLIENT.lock().await;
    *client = Some(
        Client::connect(Config {
            session: Session::load_file_or_create("omegram.session").unwrap(),
            api_id,
            api_hash: api_hash.clone(),
            params: Default::default(),
        })
        .await
        .unwrap(),
    );

    let mut db = DB.lock().await;
    *db = Some(Database::new())
}
#[tokio::main]
async fn main() {
    initialize().await;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::main::check_auth,
            commands::main::request_code,
            commands::main::sign_in,
            commands::main::get_dialogs,
            commands::main::logout,
            commands::main::get_messages,
            commands::database::get_setting,
            commands::database::set_setting,
            commands::database::update_dialogs,
            commands::database::get_chat_history,
            commands::database::set_chat_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
