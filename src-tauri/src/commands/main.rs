use crate::{ constants::{ DB, TELEGRAM }, helpers::telegram_helper, models::{ dialog::Dialog, message::Message } };

#[tauri::command]
pub async fn check_auth() -> bool {
    let db = DB.lock().await;
    db.as_ref().unwrap().get_setting("auth").unwrap_or("false".to_string()) == "true"
}

#[tauri::command]
pub async fn get_dialogs() -> Vec<Dialog> {
    let telegram = TELEGRAM.lock().await;

    return telegram.as_ref().unwrap().get_dialogs().await;
}

#[tauri::command]
pub async fn get_messages(id: String, offset: usize, limit: usize) -> Vec<Message> {
    let telegram = TELEGRAM.lock().await;

    return telegram.as_ref().unwrap().get_messages(id, offset, limit).await;
}


#[tauri::command]
pub async fn send_message(id: String, message: String) -> Result<Message, bool> {
    let telegram = TELEGRAM.lock().await;

    return telegram.as_ref().unwrap().send_message(id, message).await;
}