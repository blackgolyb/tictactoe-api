use rusqlite::Connection;

use super::MigrationInterface;

pub struct Migration;

impl MigrationInterface for Migration {
    fn metadata(&self) -> (i32, &'static str) {
        (2, "0002_remove_game_id")
    }

    fn up(&self, conn: &Connection) -> rusqlite::Result<()> {
        // SQLite doesn't support dropping columns or modifying primary keys directly
        // We need to recreate the table

        // 1. Create a new table with the correct schema
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Game_new (
                name            TEXT PRIMARY KEY,
                board           TEXT NOT NULL,
                current_player  INTEGER NOT NULL,
                rules           TEXT NOT NULL
            )",
            (),
        )?;

        // 2. Copy data from old table to new table (if old table exists)
        // We ignore the id column and use name as the key
        let _ = conn.execute(
            "INSERT INTO Game_new (name, board, current_player, rules)
             SELECT name, board, current_player, rules FROM Game
             WHERE name IS NOT NULL",
            (),
        );

        // 3. Drop the old table
        conn.execute("DROP TABLE IF EXISTS Game", ())?;

        // 4. Rename new table to original name
        conn.execute("ALTER TABLE Game_new RENAME TO Game", ())?;

        Ok(())
    }

    fn down(&self, conn: &Connection) -> rusqlite::Result<()> {
        // Recreate the old schema with id column
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Game_new (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL UNIQUE,
                board           TEXT NOT NULL,
                current_player  INTEGER NOT NULL,
                rules           TEXT NOT NULL
            )",
            (),
        )?;

        // Copy data back, generating new IDs
        let _ = conn.execute(
            "INSERT INTO Game_new (name, board, current_player, rules)
             SELECT name, board, current_player, rules FROM Game
             WHERE name IS NOT NULL",
            (),
        );

        // Drop the current table
        conn.execute("DROP TABLE IF EXISTS Game", ())?;

        // Rename new table to original name
        conn.execute("ALTER TABLE Game_new RENAME TO Game", ())?;

        Ok(())
    }
}
