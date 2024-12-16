use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use dioxus::prelude::*;
use uuid::Uuid;
use tracing::{debug, error, info, warn};
use std::path::PathBuf;

#[cfg(target_os = "android")]
fn find_writable_db_path() -> Option<PathBuf> {
    let potential_paths = vec![
        "/data/user/0/com.example.Myquest/databases",
        "/data/user/0/com.example.Myquest/files",
        "/data/data/com.example.Myquest/databases",
        "/data/data/com.example.Myquest/files",
    ];

    for path in potential_paths {
        let dir_path = PathBuf::from(path);
        
        if !dir_path.exists() {
            if let Err(e) = std::fs::create_dir_all(&dir_path) {
                warn!("Failed to create directory {}: {}", path, e);
                continue;
            }
        }

        let test_file = dir_path.join("test_write");
        match std::fs::File::create(&test_file) {
            Ok(_) => {
                let _ = std::fs::remove_file(test_file);
                info!("Found writable directory at: {}", path);
                return Some(dir_path.join("habits.db"));
            }
            Err(e) => {
                warn!("Directory {} not writable: {}", path, e);
                continue;
            }
        }
    }
    
    None
}

fn get_db_path() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        match find_writable_db_path() {
            Some(path) => {
                info!("Using database path: {}", path.display());
                path
            }
            None => {
                error!("No writable path found for database!");
                PathBuf::from("habits.db")
            }
        }
    }

    #[cfg(not(target_os = "android"))]
    {
        PathBuf::from("habits.db")
    }
}

static DB_INSTANCE: Lazy<Arc<Mutex<Connection>>> = Lazy::new(|| {
    let db_path = get_db_path();
    info!("Opening database at: {}", db_path.display());
    
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                error!("Failed to create database directory: {}", e);
            }
        }
    }

    let conn = Connection::open(&db_path).expect("Failed to open database");
    init_db(&conn).expect("Failed to initialize database");
    Arc::new(Mutex::new(conn))
});

pub fn with_db<F, T>(f: F) -> Result<T, dioxus::prelude::ServerFnError>
where
    F: FnOnce(&Connection) -> Result<T, dioxus::prelude::ServerFnError>
{
    let db = DB_INSTANCE.clone();
    let conn = db.lock().map_err(|e| {
        error!("Failed to acquire database lock: {:?}", e);
        dioxus::prelude::ServerFnError::new("Database lock error")
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

pub fn get_db() -> Arc<Mutex<Connection>> {
    DB_INSTANCE.clone()
}

fn init_db(conn: &Connection) -> rusqlite::Result<()> {
    info!("Initializing database tables");
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

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM habits",
        [],
        |row| row.get(0),
    )?;

    if count == 0 {
        let today = chrono::Local::now().date_naive();
        
        conn.execute(
            "INSERT INTO habits (id, title, start_date, color, week_start)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                Uuid::new_v4().to_string(),
                "Meditation",
                today.to_string(),
                "#4A90E2",
                "monday",
            ),
        )?;

        conn.execute(
            "INSERT INTO habits (id, title, start_date, color, week_start)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                Uuid::new_v4().to_string(),
                "Push-ups",
                today.to_string(),
                "#50C878",
                "monday",
            ),
        )?;
    }
    Ok(())
}

pub fn initialize_db() {
    let _ = &*DB_INSTANCE;
}