use grammers_client::{types::UserProfilePhoto, Client, Config};
use grammers_session::Session;
use grammers_tl_types::types::messages::DialogsSlice;
use std::{env, fs};
use tauri::utils::config::FsAllowlistConfig;

use crate::{
    models::dialog::Dialog,
    tg::{CLIENT, TOKEN},
};

/* !!! TODO: HANDLE ALL UNWRAPS !!! */
#[tauri::command]
pub async fn check_auth() -> bool {
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

    client.as_ref().unwrap().is_authorized().await.unwrap()
}

#[tauri::command]
pub async fn request_code(phone: String) {
    let client = CLIENT.lock().await;
    let mut token = TOKEN.lock().await;

    *token = Some(
        client
            .as_ref()
            .unwrap()
            .request_login_code(phone.as_str())
            .await
            .unwrap(),
    );

    println!("{}", phone);
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
        Err(e) => {
            println!(
                "NOTE: failed to save the session, will sign out when done: {}",
                e
            );
        }
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

        let mut x = 0;
        while let Some(dialog) = dialogs.next().await.unwrap() {
            let chat = dialog.chat().clone();
            if let Some(photo) = &chat.photo_downloadable(false) {
                let mut download = client.as_ref().unwrap().iter_download(&photo);
                let mut bytes = Vec::new();
                while let Some(chunk) = download.next().await.unwrap() {
                    bytes.extend(chunk);
                }

                result.push(Dialog::new(
                    chat.id().to_string(),
                    chat.name().to_string(),
                    bytes.clone(),
                ));
                x += 1;
            }
        }

        return result;
    }
    vec![]
}

#[tauri::command]
pub async fn logout() {
    fs::remove_file("omegram.session").unwrap();
}
