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

    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}
