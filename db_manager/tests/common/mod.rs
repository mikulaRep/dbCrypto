use db_manager::db_manager::DbManager;
use names::Generator;

pub fn init_db() -> (DbManager, String) {
    let db_name = Generator::default().next().unwrap() + ".db";
    let db_manager = DbManager::new(&db_name).unwrap();
    db_manager.create_tables().unwrap();
    (db_manager, db_name)
}
