use std::env;

use grammers_client::{ types::{ Downloadable, LoginToken }, Client, Config, InitParams };
use grammers_session::Session;
use grammers_tl_types as tl;

use crate::{ constants::DB, models::{dialog::Dialog, message::Message} };

pub struct TelegramHelper {
    pub client: Client,
    pub token: Option<LoginToken>,
}

impl TelegramHelper {
    pub async fn new(api_id: i32, api_hash: String) -> Self {
        let client = Client::connect(Config {
            session: Session::load_file_or_create("notelegram.session").unwrap(),
            api_hash: api_hash,
            api_id: api_id,
            params: InitParams::default(),
        }).await.unwrap();

        Self {
            client: client,
            token: None,
        }
    }

    pub async fn export_qrtoken(&mut self) -> grammers_tl_types::enums::auth::LoginToken {
        let request = tl::functions::auth::ExportLoginToken {
            api_id: env::var("API_ID").unwrap().parse().unwrap(),
            api_hash: env::var("API_HASH").unwrap(),
            except_ids: vec![],
        };

        let s = self.client.invoke(&request).await.unwrap();

        return s;
    }

    pub async fn get_update(&self) -> usize {
        let request = tl::functions::auth::ExportLoginToken {
            api_id: env::var("API_ID").unwrap().parse().unwrap(),
            api_hash: env::var("API_HASH").unwrap(),
            except_ids: vec![],
        };

        let s = self.client.invoke(&request).await.unwrap();
        println!("{:#?}", s);
        if format!("{:#?}", s).contains("Succ") {
            println!("Success");

            match self.client.session().save_to_file("notelegram.session") {
                Ok(_) => {}
                Err(_e) => {}
            }

            let mut dialogs = self.client.iter_dialogs();
            return dialogs.total().await.unwrap();
        }
        0
    }

    pub async fn request_code(&mut self, phone: &str) {
        self.token = Some(match self.client.request_login_code(phone).await {
            Ok(t) => t,
            Err(_) => {
                let api_id = env::var("API_ID").unwrap().parse().unwrap();
                let api_hash = env::var("API_HASH").unwrap().to_string();

                self.client = Client::connect(Config {
                    session: Session::load_file_or_create("omegram.session").unwrap(),
                    api_id,
                    api_hash: api_hash.clone(),
                    params: Default::default(),
                }).await.unwrap();
                self.client.request_login_code(phone).await.unwrap()
            }
        });
    }

    pub async fn sign_in(&self, code: &str) -> usize {
        self.client.sign_in(self.token.as_ref().unwrap(), code).await.unwrap();

        match self.client.session().save_to_file("notelegram.session") {
            Ok(_) => {}
            Err(_e) => {}
        }

        let mut dialogs = self.client.iter_dialogs();

        dialogs.total().await.unwrap()
    }

    pub async fn get_photo(&self, photo: &Downloadable) -> Vec<u8> {
        let mut download = self.client.iter_download(&photo);
        let mut bytes = Vec::new();
        while let Some(chunk) = download.next().await.unwrap() {
            bytes.extend(chunk);
        }

        return bytes;
    }

    pub async fn get_dialogs(&self) -> Vec<Dialog> {
        if self.client.is_authorized().await.unwrap() {
            let mut result = vec![];
            let mut dialogs = self.client.iter_dialogs();

            while let Some(dialog) = dialogs.next().await.unwrap() {
                let chat = dialog.chat().clone();
                if let Some(photo) = &chat.photo_downloadable(false) {
                    let bytes = self.get_photo(&photo).await;

                    result.push(
                        Dialog::new(chat.id().to_string(), chat.name().to_string(), bytes.clone())
                    );
                }
            }

            return result;
        }
        vec![]
    }

    pub async fn get_messages(&self, id: String, offset: usize, limit: usize) -> Vec<Message> {
        let db = DB.lock().await;

        let mut chats = self.client.iter_dialogs();

        while let Some(dialog) = chats.next().await.unwrap() {
            if dialog.chat().id().to_string() == id {
                let mut messages = self.client.iter_messages(dialog.chat());
                let mut result: Vec<Message> = vec![];
                let mut skipped = offset;

                while let Some(message) = messages.next().await.unwrap() {
                    if skipped <= offset {
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
                                if
                                    let Some(b) = db
                                        .as_ref()
                                        .unwrap()
                                        .get_photo(&message.sender().unwrap().id().to_string())
                                {
                                    bytes = b;
                                } else {
                                    bytes = self.get_photo(&photo).await;
                                    db.as_ref()
                                        .unwrap()
                                        .set_photo(
                                            &message.sender().unwrap().id().to_string(),
                                            bytes.clone()
                                        );
                                }

                                let msg = Message::new(
                                    message.id().to_string(),
                                    message.text().to_string(),
                                    bytes.clone()
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
}
