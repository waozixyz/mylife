// src/server/state.rs
use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use dioxus::prelude::*;
use uuid::Uuid;
use tracing::{debug, error, info, instrument, warn};

#[cfg(feature = "server")]
static DB_INSTANCE: Lazy<Arc<Mutex<Connection>>> = Lazy::new(|| {
    // Use a platform-specific path for mobile
    #[cfg(target_os = "android")]
    let db_path = std::env::var("HOME").unwrap_or_else(|_| ".".to_string()) + "/habits.db";
    #[cfg(not(target_os = "android"))]
    let db_path = "habits.db";

    println!("Opening database at: {}", db_path);
    let conn = Connection::open(&db_path).expect("Failed to open database");
    init_db(&conn).expect("Failed to initialize database");
    Arc::new(Mutex::new(conn))
});

#[cfg(feature = "server")]
pub fn with_db<F, T>(f: F) -> Result<T, ServerFnError>
where
    F: FnOnce(&Connection) -> Result<T, ServerFnError>
{
    let db = DB_INSTANCE.clone();
    let conn = db.lock().map_err(|e| {
        error!("Failed to acquire database lock: {:?}", e);
        ServerFnError::new("Database lock error")
    })?;
    
    match f(&conn) {
        Ok(result) => {
            debug!("Database operation completed successfully");
            Ok(result)
        }
        Err(e) => {
            error!("Database operation failed: {:?}", e);
            Err(e)
        }
    }
}

#[cfg(feature = "server")]
pub fn get_db() -> Arc<Mutex<Connection>> {
    DB_INSTANCE.clone()
}
#[cfg(feature = "server")]
fn init_db(conn: &Connection) -> rusqlite::Result<()> {
    info!("Initializing database tables");

    // Create tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS habits (
         id TEXT PRIMARY KEY,
         title TEXT NOT NULL,
         start_date TEXT NOT NULL,
         color TEXT DEFAULT '#800080',
         week_start TEXT DEFAULT 'sunday'
         )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS completed_days (
         habit_id TEXT,
         date TEXT NOT NULL,
         FOREIGN KEY(habit_id) REFERENCES habits(id),
         PRIMARY KEY (habit_id, date)
         )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
         id TEXT PRIMARY KEY,
         content TEXT NOT NULL,
         day TEXT NOT NULL,
         created_at TEXT NOT NULL,
         position INTEGER NOT NULL
         )",
        [],
    )?;

    // Check if there are any habits
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM habits",
        [],
        |row| row.get(0),
    )?;

    // If no habits exist, create default habits
    if count == 0 {
        let today = chrono::Local::now().date_naive();
        
        // Create meditation habit
        conn.execute(
            "INSERT INTO habits (id, title, start_date, color, week_start) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                Uuid::new_v4().to_string(),
                "Meditation",
                today.to_string(),
                "#4A90E2",  // Blue color
                "monday",
            ),
        )?;

        // Create push-ups habit
        conn.execute(
            "INSERT INTO habits (id, title, start_date, color, week_start) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                Uuid::new_v4().to_string(),
                "Push-ups",
                today.to_string(),
                "#50C878",  // Green color
                "monday",
            ),
        )?;
    }

    Ok(())
}
#[cfg(feature = "server")]
pub fn initialize_db() {
    let _ = &*DB_INSTANCE;
}