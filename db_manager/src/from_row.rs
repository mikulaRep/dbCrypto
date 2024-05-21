use crate::user::User;

use rusqlite::Row;

pub trait FromRow {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error>
    where
        Self: std::marker::Sized;
}

impl FromRow for User {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            surname: row.get(2)?,
            birth: row.get(3)?,
        })
    }
}
