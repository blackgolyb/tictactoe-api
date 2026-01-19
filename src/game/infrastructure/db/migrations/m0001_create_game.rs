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

/*
# Check current player
curl "http://localhost:8128/api/v1/big_game/get_current_player" -o current_player.png

# Make a move at position (0,0)
curl "http://localhost:8128/api/v1/big_game/update_field/0,0"

# Get the field image
curl "http://localhost:8128/api/v1/big_game/get_field/0,0" -o field.png

# Make another move at position (1,1)
curl "http://localhost:8128/api/v1/big_game/update_field/1,1"
 */
