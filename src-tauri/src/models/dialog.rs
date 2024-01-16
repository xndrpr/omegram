use serde::Serialize;

#[derive(Serialize)]
pub struct Dialog {
    pub id: String,
    pub name: String,
    pub photo: Vec<u8>,
}

impl Dialog {
    pub fn new(id: String, name: String, photo: Vec<u8>) -> Self {
        Self {
            id: id,
            name: name,
            photo: photo,
        }
    }
}
