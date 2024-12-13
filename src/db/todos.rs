use crate::models::todo::Todo;
use chrono::NaiveDateTime;
use rusqlite::{Connection, Result};
use uuid::Uuid;

pub fn init_todos_table(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            day TEXT NOT NULL,
            created_at TEXT NOT NULL,
            position INTEGER NOT NULL
        )",
    )?;
    stmt.execute([])?;
    Ok(())
}

pub fn save_todo(conn: &Connection, todo: &Todo) -> Result<()> {
    let mut stmt = conn.prepare(
        "INSERT INTO todos (id, content, day, created_at, position)
         VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;
    stmt.execute((
        todo.id.to_string(),
        &todo.content,
        &todo.day,
        todo.created_at.to_string(),
        todo.position,
    ))?;
    Ok(())
}
// Remove async and just return Result
pub fn load_todos_by_day(conn: &Connection, day: &str) -> Result<Vec<Todo>> {
    let mut stmt = conn.prepare(
        "SELECT id, content, day, created_at, position
         FROM todos
         WHERE day = ?1
         ORDER BY position ASC",
    )?;
    let todos_iter = stmt.query_map([day], |row| {
        Ok(Todo {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            content: row.get(1)?,
            day: row.get(2)?,
            created_at: NaiveDateTime::parse_from_str(
                &row.get::<_, String>(3)?,
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            position: row.get(4)?,
        })
    })?;
    let mut todos = Vec::new();
    for todo in todos_iter {
        todos.push(todo?);
    }
    Ok(todos)
}

pub fn delete_todo(conn: &Connection, id: Uuid) -> Result<()> {
    let mut stmt = conn.prepare("DELETE FROM todos WHERE id = ?1")?;
    stmt.execute((id.to_string(),))?;
    Ok(())
}

pub fn update_todo_positions(conn: &Connection, updates: &[(Uuid, i32)]) -> Result<()> {
    let mut stmt = conn.prepare("UPDATE todos SET position = ?1 WHERE id = ?2")?;
    for (id, position) in updates {
        stmt.execute((position, id.to_string()))?;
    }
    Ok(())
}

pub fn move_todo_to_day(conn: &Connection, id: Uuid, new_day: &str) -> Result<()> {
    let mut stmt = conn.prepare("UPDATE todos SET day = ?1 WHERE id = ?2")?;
    stmt.execute((new_day, id.to_string()))?;
    Ok(())
}
