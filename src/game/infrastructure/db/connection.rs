use std::sync::{Arc, Mutex};

use rusqlite::Connection;

pub type DBConnection = Arc<Mutex<Connection>>;

pub fn establish_connection(db_path: String) -> DBConnection {
    Arc::new(Mutex::new(Connection::open(db_path).unwrap()))
}
