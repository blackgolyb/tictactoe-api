use rusqlite::Connection;

use super::MigrationInterface;

pub struct Migration;

impl MigrationInterface for Migration {
    fn metadata(&self) -> (i32, &'static str) {
        (1, "0001_create_game")
    }

    fn up(&self, conn: &Connection) -> rusqlite::Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Game (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL UNIQUE,
                board           TEXT NOT NULL,
                current_player  INTEGER NOT NULL,
                rules           TEXT NOT NULL
            )",
            (),
        )?;
        Ok(())
    }

    fn down(&self, conn: &Connection) -> rusqlite::Result<()> {
        conn.execute("DROP TABLE EXISTS Game;", ())?;
        Ok(())
    }
}
