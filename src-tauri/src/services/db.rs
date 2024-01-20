use std::path;

use rusqlite::Connection;

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
        Self { conn: conn }
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
}