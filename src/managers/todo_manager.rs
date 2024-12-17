// managers/todo_manager.rs
use crate::models::todo::Todo;
use crate::storage::{get_path_manager, JsonStorage};
use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DayTodos {
    pub todos: Vec<Todo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TodoStore {
    monday: DayTodos,
    tuesday: DayTodos,
    wednesday: DayTodos,
    thursday: DayTodos,
    friday: DayTodos,
    saturday: DayTodos,
    sunday: DayTodos,
}

impl TodoStore {
    fn get_day_mut(&mut self, day: &str) -> &mut DayTodos {
        match day.to_lowercase().as_str() {
            "monday" => &mut self.monday,
            "tuesday" => &mut self.tuesday,
            "wednesday" => &mut self.wednesday,
            "thursday" => &mut self.thursday,
            "friday" => &mut self.friday,
            "saturday" => &mut self.saturday,
            "sunday" => &mut self.sunday,
            _ => panic!("Invalid day: {}", day),
        }
    }

    fn get_day(&self, day: &str) -> &DayTodos {
        match day.to_lowercase().as_str() {
            "monday" => &self.monday,
            "tuesday" => &self.tuesday,
            "wednesday" => &self.wednesday,
            "thursday" => &self.thursday,
            "friday" => &self.friday,
            "saturday" => &self.saturday,
            "sunday" => &self.sunday,
            _ => panic!("Invalid day: {}", day),
        }
    }
}

pub struct TodoManager {
    storage: JsonStorage<TodoStore>,
}

impl TodoManager {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            storage: JsonStorage::new(get_path_manager().todos_file())
                .map_err(|e| e.to_string())?,
        })
    }

    pub async fn get_todos_by_day(&self, day: &str) -> Result<Vec<Todo>, String> {
        debug!("Getting todos for day: {}", day);
        self.storage
            .read(|store| {
                let day_todos = store.get_day(day);
                let mut todos = day_todos.todos.clone();
                todos.sort_by_key(|t| t.position);
                todos
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn create_todo(&self, content: String, day: String) -> Result<(), String> {
        debug!("Creating new todo for day: {}", day);
        let day_clone = day.clone();
        let created_at = chrono::Local::now().naive_local();

        self.storage
            .write(|store| {
                let day_todos = store.get_day_mut(&day_clone);
                let position = day_todos.todos.len() as i32 + 1;
                let todo = Todo {
                    id: Uuid::new_v4(),
                    content,
                    day: day_clone,
                    created_at,
                    position,
                };
                day_todos.todos.push(todo);
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_todo(&self, id: Uuid) -> Result<(), String> {
        debug!("Deleting todo with id: {}", id);
        self.storage
            .write(|store| {
                let days = vec![
                    "monday",
                    "tuesday",
                    "wednesday",
                    "thursday",
                    "friday",
                    "saturday",
                    "sunday",
                ];

                for day in days {
                    let day_todos = store.get_day_mut(day);
                    if let Some(pos) = day_todos.todos.iter().position(|t| t.id == id) {
                        day_todos.todos.remove(pos);
                        // Reorder remaining todos
                        for (i, todo) in day_todos.todos.iter_mut().enumerate() {
                            todo.position = (i + 1) as i32;
                        }
                        break;
                    }
                }
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_positions(
        &self,
        day: &str,
        updates: Vec<(Uuid, i32)>,
    ) -> Result<(), String> {
        debug!("Updating positions for day: {}", day);
        let day_clone = day.to_string();
        self.storage
            .write(|store| {
                let day_todos = store.get_day_mut(&day_clone);
                let position_map: HashMap<_, _> = updates.into_iter().collect();

                for todo in day_todos.todos.iter_mut() {
                    if let Some(&new_position) = position_map.get(&todo.id) {
                        todo.position = new_position;
                    }
                }
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn move_todo(&self, id: Uuid, new_day: String) -> Result<(), String> {
        debug!("Moving todo {} to day: {}", id, new_day);
        let new_day_clone = new_day.clone();
        self.storage
            .write(|store| {
                let days = vec![
                    "monday",
                    "tuesday",
                    "wednesday",
                    "thursday",
                    "friday",
                    "saturday",
                    "sunday",
                ];

                // Find and remove todo from current day
                let mut todo_to_move = None;
                for day in &days {
                    let day_todos = store.get_day_mut(day);
                    if let Some(pos) = day_todos.todos.iter().position(|t| t.id == id) {
                        let mut todo = day_todos.todos.remove(pos);
                        todo.day = new_day_clone.clone(); // Update the day field
                        todo_to_move = Some(todo);
                        // Reorder remaining todos
                        for (i, todo) in day_todos.todos.iter_mut().enumerate() {
                            todo.position = (i + 1) as i32;
                        }
                        break;
                    }
                }

                // Add todo to new day
                if let Some(mut todo) = todo_to_move {
                    let new_day_todos = store.get_day_mut(&new_day_clone);
                    todo.position = new_day_todos.todos.len() as i32 + 1;
                    new_day_todos.todos.push(todo);
                }
            })
            .await
            .map_err(|e| e.to_string())
    }

    // Additional helper methods
    pub async fn force_save(&self) -> Result<(), String> {
        self.storage.force_save().await.map_err(|e| e.to_string())
    }

    pub async fn reload(&self) -> Result<(), String> {
        self.storage.reload().await.map_err(|e| e.to_string())
    }
}

static TODO_MANAGER: Lazy<TodoManager> =
    Lazy::new(|| TodoManager::new().expect("Failed to create todo manager"));

pub fn get_todo_manager() -> &'static TodoManager {
    &*TODO_MANAGER
}
