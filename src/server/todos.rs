use crate::models::todo::Todo;
use chrono::NaiveDateTime;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::server::state::with_db;
use tracing::{debug, error, info};

pub async fn get_todos_by_day(day: String) -> Result<Vec<Todo>, ServerFnError> {
    with_db(|conn| {
        let mut stmt = conn
            .prepare(
                "SELECT id, content, day, created_at, position
                 FROM todos
                 WHERE day = ?1
                 ORDER BY position ASC",
            )
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let todos = stmt
            .query_map([day], |row| {
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
            })
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .filter_map(Result::ok)
            .collect();

        Ok(todos)
    })
}

pub async fn create_todo(todo: Todo) -> Result<(), ServerFnError> {
    with_db(|conn| {
        conn.execute(
            "INSERT INTO todos (id, content, day, created_at, position) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                todo.id.to_string(),
                &todo.content,
                &todo.day,
                todo.created_at.to_string(),
                todo.position,
            ),
        )
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
    })
}

pub async fn delete_todo(id: Uuid) -> Result<(), ServerFnError> {
    with_db(|conn| {
        conn.execute("DELETE FROM todos WHERE id = ?1", [id.to_string()])
            .map(|_| ())
            .map_err(|e| ServerFnError::new(e.to_string()))
    })
}

pub async fn update_positions(updates: Vec<(Uuid, i32)>) -> Result<(), ServerFnError> {
    with_db(|conn| {
        for (id, position) in updates {
            conn.execute(
                "UPDATE todos SET position = ?1 WHERE id = ?2",
                [&position.to_string(), &id.to_string()],
            )
            .map(|_| ())
            .map_err(|e| ServerFnError::new(e.to_string()))?;
        }
        Ok(())
    })
}

pub async fn move_todo(id: Uuid, new_day: String) -> Result<(), ServerFnError> {
    with_db(|conn| {
        conn.execute(
            "UPDATE todos SET day = ?1 WHERE id = ?2",
            [&new_day, &id.to_string()],
        )
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
    })
}