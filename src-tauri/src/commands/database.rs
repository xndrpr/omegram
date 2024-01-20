use grammers_client::{
    types::{message, Downloadable},
    Client, Config,
};
use grammers_session::Session;
use std::{env, fs};

use crate::{
    constants::{CLIENT, DB, TOKEN},
    models::{dialog::Dialog, message::Message},
};

#[tauri::command]
pub async fn get_setting(key: String) -> String {
    println!("{}", key);
    let db = DB.lock().await;
    db.as_ref()
        .unwrap()
        .get_setting(&key)
        .unwrap_or("".to_string())
}

#[tauri::command]
pub async fn set_setting(key: String, value: String) {
    println!("{}:{}", key, value);

    let db = DB.lock().await;
    db.as_ref().unwrap().set_setting(&key, &value);
}

#[tauri::command]
pub async fn update_dialogs() -> bool {
    println!("Updating dialogs");
    true
}
