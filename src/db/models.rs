use rusqlite::Connection;

pub fn init_db(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Room (
            id              INTEGER PRIMARY KEY,
            name            TEXT NOT NULL UNIQUE,
            board           INTEGER,
            current_player  INTEGER
        )",
        (),
    )
    .unwrap();
}
