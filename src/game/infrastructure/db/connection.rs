use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, OpenFlags};

pub type DBConnection = Pool<SqliteConnectionManager>;

pub fn establish_connection(db_path: String) -> DBConnection {
    let manager = SqliteConnectionManager::file(&db_path)
        .with_flags(
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .with_init(|conn| {
            // Enable WAL mode for better concurrency
            conn.execute_batch(
                "PRAGMA journal_mode = WAL;
                 PRAGMA synchronous = NORMAL;
                 PRAGMA busy_timeout = 5000;
                 PRAGMA cache_size = -64000;",
            )?;
            Ok(())
        });

    Pool::builder()
        .max_size(15) // Maximum number of connections in the pool
        .connection_timeout(std::time::Duration::from_secs(5))
        .build(manager)
        .expect("Failed to create database connection pool")
}

pub fn establish_connection_direct(db_path: String) -> Connection {
    let conn = Connection::open(db_path).expect("Failed to open database connection");

    // Enable WAL mode for the direct connection too
    conn.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = NORMAL;
         PRAGMA busy_timeout = 5000;",
    )
    .expect("Failed to set SQLite pragmas");

    conn
}
