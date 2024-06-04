use std::path;

use rusqlite::{params, Connection};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new() -> Self {
        log::info!("Creating new database");
        let conn = Connection::open(path::Path::new("db.sqlite")).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (key TEXT UNIQUE, value TEXT);",
            (),
        )
        .unwrap_or(0);

        conn.execute(
            "CREATE TABLE IF NOT EXISTS chats (chat_id TEXT UNIQUE, history TEXT);",
            (),
        )
        .unwrap_or(0);

        conn.execute(
            "CREATE TABLE IF NOT EXISTS photos (id TEXT UNIQUE, photo BLOB);",
            (),
        )
        .unwrap_or(0);
        Self { conn: conn }
    }

    pub fn get_photo(&self, id: &str) -> Option<Vec<u8>> {
        let mut stmt = self
            .conn
            .prepare("SELECT photo FROM photos WHERE id = ?")
            .unwrap();
        stmt.query_row([id], |row| row.get(0)).unwrap_or(None)
    }

    pub fn set_photo(&self, id: &str, photo: Vec<u8>) {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO photos (id, photo) VALUES (?, ?)",
                params![id, &photo],
            )
            .unwrap_or(0);
    }

    pub fn get_setting(&self, key: &str) -> Option<String> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM settings WHERE key = ?")
            .unwrap();
        stmt.query_row([key], |row| row.get(0)).unwrap_or(None)
    }

    pub fn set_setting(&self, key: &str, value: &str) {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)",
                [key, value],
            )
            .unwrap_or(0);
    }

    pub fn get_chat_history(&self, chat_id: &str) -> Option<String> {
        let mut stmt = self
            .conn
            .prepare("SELECT history FROM chats WHERE chat_id = ?")
            .unwrap();
        stmt.query_row([chat_id], |row| row.get(0)).unwrap_or(None)
    }

    pub fn set_chat_history(&self, chat_id: &str, history: &str) {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO chats (chat_id, history) VALUES (?, ?)",
                [chat_id, history],
            )
            .unwrap_or(0);
    }
}
