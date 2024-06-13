use db_manager::user::{User, UserManager};
use std::{fs, io};

mod common;

#[test]

fn add_user() -> io::Result<()> {
    let user = User::new(Some(1), "Kuba", "Mikula", "1992.03.16");
    let id = 1;
    let (db, db_name) = common::init_db();

    let code = db.insert_user(user.clone()).unwrap();
    assert_eq!(code, 1);

    let users = db.query_users().unwrap();
    assert_eq!(users, vec![user.clone()]);

    let code = db.delete_user(id).unwrap();
    assert_eq!(code, 1);

    let users = db.query_users().unwrap();
    assert_eq!(users, vec![]);

    let _ = fs::remove_file(db_name)?;
    Ok(())
}
