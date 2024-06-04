use crate::constants::DB;

#[tauri::command]
pub async fn get_setting(key: String) -> String {
    let db = DB.lock().await;
    db.as_ref()
        .unwrap()
        .get_setting(&key)
        .unwrap_or("".to_string())
}

#[tauri::command]
pub async fn set_setting(key: String, value: String) {
    let db = DB.lock().await;
    db.as_ref().unwrap().set_setting(&key, &value);
}

#[tauri::command]
pub async fn update_dialogs() -> bool {
    true
}

#[tauri::command]
pub async fn get_chat_history(id: String) -> String {
    let db = DB.lock().await;
    db.as_ref()
        .unwrap()
        .get_chat_history(&id)
        .unwrap_or("".to_string())
}

#[tauri::command]
pub async fn set_chat_history(id: String, history: String) {
    let db = DB.lock().await;
    db.as_ref().unwrap().set_chat_history(&id, &history);
}