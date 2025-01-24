use rusqlite::{Connection, OpenFlags};
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new() -> Self {
        let conn = Connection::open_with_flags(
            "employees.db",
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )
        .expect("Failed to open database");

        // Ensure the `employees` table exists
        conn.execute(
            "CREATE TABLE IF NOT EXISTS employees (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                position TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL
            );",
            [],
        )
        .expect("Failed to create table");

        Database {
            conn: Mutex::new(conn),
        }
    }

    pub fn get_conn(&self) -> std::sync::MutexGuard<Connection> {
        self.conn.lock().expect("Failed to lock the database connection")
    }
}

// Initialize the database
lazy_static::lazy_static! {
    pub static ref DB: Database = Database::new();
}
