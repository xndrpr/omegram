#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod constants;
pub mod models;
pub mod services;
pub mod helpers;

use std::env;

use constants::{ DB, TELEGRAM };
use grammers_client::{ Client, Config };
use helpers::telegram_helper::TelegramHelper;
use log::error;
use services::db::Database;

async fn initialize() {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "error");

    pretty_env_logger::init();

    let mut telegram = TELEGRAM.lock().await;
    *telegram = Some(
        TelegramHelper::new(
            env
                ::var("API_ID")
                .map_err(|err| {
                    error!("Failed to get API_ID: {}", err);
                })
                .unwrap()
                .parse()
                .unwrap(),
            env
                ::var("API_HASH")
                .map_err(|err| {
                    error!("Failed to get API_HASH: {}", err);
                })
                .unwrap()
                .to_string()
        ).await
    );

    let mut db = DB.lock().await;
    *db = Some(Database::new());
}
#[tokio::main]
async fn main() {
    initialize().await;

    tauri::Builder
        ::default()
        .invoke_handler(
            tauri::generate_handler![
                commands::sign::request_code,
                commands::sign::request_qrcode,
                commands::sign::sign_in,
                commands::sign::get_update,
                commands::main::check_auth,
                commands::main::get_dialogs,
                commands::main::get_messages,
                commands::main::send_message,
                commands::database::get_setting,
                commands::database::set_setting,
                commands::database::update_dialogs,
                commands::database::get_chat_history,
                commands::database::set_chat_history
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
