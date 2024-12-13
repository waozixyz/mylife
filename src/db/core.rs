use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("habits.db")?;
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

    Ok(conn)
}
