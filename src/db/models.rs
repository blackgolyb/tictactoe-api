use rusqlite::Connection;

pub fn init_db(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS rooms (
            id              INTEGER PRIMARY KEY,
            name            TEXT NOT NULL UNIQUE,
            board           INTEGER,
            current_player  INTEGER
        )",
        (), // empty list of parameters.
    )
    .unwrap();
}
