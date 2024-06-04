use std::sync::Arc;

use grammers_client::types::LoginToken;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

use crate::{ helpers::telegram_helper::TelegramHelper, services::db::Database };

lazy_static! {
    pub static ref TOKEN: Arc<Mutex<Option<LoginToken>>> = Arc::new(Mutex::new(None));
}

lazy_static! {
    pub static ref DB: Arc<Mutex<Option<Database>>> = Arc::new(Mutex::new(None));
}

lazy_static! {
    pub static ref TELEGRAM: tokio::sync::Mutex<Option<TelegramHelper>> = tokio::sync::Mutex::new(
        None
    );
}
