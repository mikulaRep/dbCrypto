use crate::error::CryptoResult;
use rusqlite::Connection;

pub struct DbManager {
    conn: Connection,
}

impl DbManager {
    pub fn new(name: &str) -> CryptoResult<Self> {
        let conn = Connection::open(name)?;
        Ok(Self { conn })
    }

    pub fn create_tables(&self) -> CryptoResult<()> {
        self.conn.execute(
            "CREATE TABLE if not exists USER (
            id                  INTEGER PRIMARY KEY,
            name                TEXT NOT NULL,
            surname             TEXT NOT NULL,
            birth               TEXT NOT NULL
        )",
            (),
        )?;

        Ok(())
    }

    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}
