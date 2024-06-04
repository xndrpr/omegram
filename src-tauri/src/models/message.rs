use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub id: String,
    pub text: String,
    pub avatar: Vec<u8>,
}

impl Message {
    pub fn new(id: String, text: String, avatar: Vec<u8>) -> Self {
        Self {
            id: id,
            text: text,
            avatar: avatar,
        }
    }
}