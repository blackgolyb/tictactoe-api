use std::sync::{Arc, Mutex};

use rusqlite::Connection;

pub fn get_connection(db_path: String) -> Connection {
    Connection::open(db_path).unwrap()
}

pub type DBConnection = Arc<Mutex<Connection>>;
