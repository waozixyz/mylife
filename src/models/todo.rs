use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: Uuid,
    pub content: String,
    pub day: String,
    pub created_at: NaiveDateTime,
    pub position: i32,
}

impl Todo {
    pub fn new(content: String, day: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            day,
            created_at: chrono::Local::now().naive_local(),
            position: 0,
        }
    }
}
