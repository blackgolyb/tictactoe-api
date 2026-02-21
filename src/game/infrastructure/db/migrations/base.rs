use rusqlite::Connection;

pub trait MigrationInterface {
    /// Returns migration metadata: (id, name)
    fn metadata(&self) -> (i32, &'static str);

    /// Apply the migration (upgrade)
    fn up(&self, conn: &Connection) -> rusqlite::Result<()>;

    /// Revert the migration (downgrade)
    fn down(&self, conn: &Connection) -> rusqlite::Result<()>;
}
