#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;
pub mod models;
pub mod tg;

use std::env;

use grammers_client::{Client, Config};
use grammers_session::Session;
use tg::CLIENT;

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
}
#[tokio::main]
async fn main() {
    initialize().await;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::check_auth,
            commands::request_code,
            commands::sign_in,
            commands::get_dialogs,
            commands::logout,
            commands::get_messages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
