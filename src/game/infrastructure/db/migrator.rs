use super::{connection::DBConnection, migrations::get_migrations};
use rusqlite::{params, Connection};

pub struct Migrator<'a> {
    conn: &'a Connection,
}

impl<'a> Migrator<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Migrator { conn }
    }

    pub fn init_metadata_table(&self) {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
                [],
            )
            .unwrap();
    }

    pub fn applied_migrations(&self) -> Vec<i32> {
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM migrations ORDER BY id")
            .unwrap();
        let rows = stmt.query_map([], |row| row.get(0)).unwrap();
        rows.map(|r| r.unwrap()).collect()
    }

    pub fn up(&self) {
        self.init_metadata_table();
        let applied = self.applied_migrations();
        for migration in get_migrations() {
            let (id, name) = migration.metadata();
            if !applied.contains(&id) {
                migration.up(&self.conn).unwrap();
                self.conn
                    .execute(
                        "INSERT INTO migrations (id, name) VALUES (?, ?)",
                        params![id, name],
                    )
                    .unwrap();
            }
        }
    }

    pub fn down(&self) {
        self.init_metadata_table();
        let applied = self.applied_migrations();
        if let Some(&last_id) = applied.last() {
            for migration in get_migrations() {
                let (id, _) = migration.metadata();
                if id == last_id {
                    migration.down(&self.conn).unwrap();
                    self.conn
                        .execute("DELETE FROM migrations WHERE id = ?", params![id])
                        .unwrap();
                    break;
                }
            }
        }
    }
}

/// Helper function to run all pending migrations
pub fn run_migrations(db_connection: &DBConnection) {
    let conn = db_connection
        .lock()
        .expect("Failed to acquire database lock");
    let migrator = Migrator::new(&conn);
    migrator.up();
}
