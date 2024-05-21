use crate::db_manager::DbManager;
use crate::error::CryptoResult;
use crate::from_row::FromRow;
use rusqlite::params;

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub birth: Option<String>,
}

impl User {
    pub fn new(id: Option<i32>, name: &str, surname: &str, birth: Option<&str>) -> Self {
        Self {
            id: id.unwrap_or(-1),
            name: name.to_string(),
            surname: surname.to_string(),
            birth: birth.map(str::to_string),
        }
    }
}

pub trait UserManager {
    fn insert_user(&self, user: User) -> CryptoResult<usize>;
    fn query_users(&self) -> CryptoResult<Vec<User>>;
    fn delete_user(&self, user_id: i32) -> CryptoResult<usize>;
}

impl UserManager for DbManager {
    fn insert_user(&self, user: User) -> CryptoResult<usize> {
        let User {
            name,
            surname,
            birth,
            ..
        } = user;

        const STMT: &str = "INSERT INTO User (name, surname, birth) VALUES (?1, ?2, ?3)";

        self.conn()
            .execute(STMT, params![name, surname, birth])
            .map_err(Into::into)
    }

    fn query_users(&self) -> CryptoResult<Vec<User>> {
        self.conn()
            .prepare("SELECT * FROM User")?
            .query_map([], User::from_row)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn delete_user(&self, id: i32) -> CryptoResult<usize> {
        self.conn()
            .prepare("DELETE FROM User WHERE id = ?1")?
            .execute([id])
            .map_err(Into::into)
    }
}
