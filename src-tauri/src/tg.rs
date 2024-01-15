use std::sync::Arc;

use grammers_client::{types::LoginToken, Client};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref CLIENT: Arc<Mutex<Option<Client>>> = Arc::new(Mutex::new(None));
}

lazy_static! {
    pub static ref TOKEN: Arc<Mutex<Option<LoginToken>>> = Arc::new(Mutex::new(None));
}
