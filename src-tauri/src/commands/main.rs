use grammers_client::{types::Downloadable, Client, Config};
use grammers_session::Session;
use std::{env, fs};

use crate::{
    constants::{CLIENT, DB, TOKEN},
    models::{dialog::Dialog, message::Message},
};

pub async fn get_photo(client: &Client, photo: &Downloadable) -> Vec<u8> {
    let mut download = client.iter_download(&photo);
    let mut bytes = Vec::new();
    while let Some(chunk) = download.next().await.unwrap() {
        bytes.extend(chunk);
    }

    return bytes;
}

#[tauri::command]
pub async fn check_auth() -> bool {
    let db = DB.lock().await;
    db.as_ref()
        .unwrap()
        .get_setting("auth")
        .unwrap_or("false".to_string())
        == "true"
}

#[tauri::command]
pub async fn request_code(phone: String) {
    let client = CLIENT.lock().await;
    let mut token = TOKEN.lock().await;

    *token = Some(
        match client
            .as_ref()
            .unwrap()
            .request_login_code(phone.as_str())
            .await
        {
            Ok(cl) => cl,
            Err(_) => {
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
                client
                    .as_ref()
                    .unwrap()
                    .request_login_code(phone.as_str())
                    .await
                    .unwrap()
            }
        },
    );
}

#[tauri::command]
pub async fn sign_in(code: String) -> usize {
    let client = CLIENT.lock().await;
    let token = TOKEN.lock().await;

    client
        .as_ref()
        .unwrap()
        .sign_in(token.as_ref().unwrap(), code.as_str())
        .await
        .unwrap();
    match client
        .as_ref()
        .unwrap()
        .session()
        .save_to_file("omegram.session")
    {
        Ok(_) => {}
        Err(_e) => {}
    }
    let mut dialogs = client.as_ref().unwrap().iter_dialogs();

    return dialogs.total().await.unwrap();
}

#[tauri::command]
pub async fn get_dialogs() -> Vec<Dialog> {
    let client = CLIENT.lock().await;

    if client.as_ref().unwrap().is_authorized().await.unwrap() {
        let mut result = vec![];
        let mut dialogs = client.as_ref().unwrap().iter_dialogs();

        while let Some(dialog) = dialogs.next().await.unwrap() {
            let chat = dialog.chat().clone();
            if let Some(photo) = &chat.photo_downloadable(false) {
                let bytes = get_photo(client.as_ref().unwrap(), &photo).await;

                result.push(Dialog::new(
                    chat.id().to_string(),
                    chat.name().to_string(),
                    bytes.clone(),
                ));
            }
        }

        return result;
    }
    vec![]
}

#[tauri::command]
pub async fn logout() {
    let _ = fs::remove_file("omegram.session");

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

#[tauri::command]
pub async fn get_messages(id: String, offset: usize, limit: usize) -> Vec<Message> {
    let client = CLIENT.lock().await;
    let db = DB.lock().await;

    let mut chats = client.as_ref().unwrap().iter_dialogs();

    while let Some(dialog) = chats.next().await.unwrap() {
        if dialog.chat().id().to_string() == id {
            let mut messages = client.as_ref().unwrap().iter_messages(dialog.chat());
            let mut result: Vec<Message> = vec![];
            let mut skipped = 0;

            while let Some(message) = messages.next().await.unwrap() {
                if skipped < offset {
                    skipped += 1;
                    continue;
                }

                if result.len() >= limit {
                    break;
                }
                if message.text().to_string().len() > 1 {
                    if let Some(sender) = message.sender() {
                        if let Some(photo) = sender.photo_downloadable(false) {
                            let bytes;
                            if let Some(b) = db
                                .as_ref()
                                .unwrap()
                                .get_photo(&message.sender().unwrap().id().to_string())
                            {
                                bytes = b;
                            } else {
                                bytes = get_photo(client.as_ref().unwrap(), &photo).await;
                                db.as_ref().unwrap().set_photo(
                                    &message.sender().unwrap().id().to_string(),
                                    bytes.clone(),
                                );
                            }

                            let msg = Message::new(
                                message.id().to_string(),
                                message.text().to_string(),
                                bytes.clone(),
                            );

                            result.push(msg);
                        }
                    }
                }
            }

            result.reverse();
            return result;
        }
    }

    vec![]
}
