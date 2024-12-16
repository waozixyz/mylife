use crate::models::habit::Habit;
use chrono::NaiveDate;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rusqlite::OptionalExtension;
use crate::server::state::with_db;
use tracing::{debug, error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHabitRequest {
    pub title: Option<String>,
    pub start_date: Option<String>,
    pub color: Option<String>,
    pub week_start: Option<String>,
}

pub async fn get_habits() -> Result<Vec<Habit>, ServerFnError> {
    with_db(|conn| {
        let mut stmt = conn
            .prepare("SELECT id, title, start_date, color, week_start FROM habits")
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let habits = stmt
            .query_map([], |row| {
                Ok(Habit {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    title: row.get(1)?,
                    start_date: NaiveDate::parse_from_str(&row.get::<_, String>(2)?, "%Y-%m-%d")
                        .unwrap(),
                    color: row.get(3)?,
                    week_start: row.get(4)?,
                })
            })
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .filter_map(Result::ok)
            .collect();

        Ok(habits)
    })
}

pub async fn get_habit(id: Uuid) -> Result<Habit, ServerFnError> {
    with_db(|conn| {
        conn.query_row(
            "SELECT id, title, start_date, color, week_start FROM habits WHERE id = ?1",
            [id.to_string()],
            |row| {
                Ok(Habit {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    title: row.get(1)?,
                    start_date: NaiveDate::parse_from_str(&row.get::<_, String>(2)?, "%Y-%m-%d")
                        .unwrap(),
                    color: row.get(3)?,
                    week_start: row.get(4)?,
                })
            },
        )
        .map_err(|e| ServerFnError::new(e.to_string()))
    })
}

pub async fn get_first_habit() -> Result<Option<Habit>, ServerFnError> {
    with_db(|conn| {
        conn.query_row(
            "SELECT id, title, start_date, color, week_start FROM habits LIMIT 1",
            [],
            |row| {
                Ok(Habit {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    title: row.get(1)?,
                    start_date: NaiveDate::parse_from_str(&row.get::<_, String>(2)?, "%Y-%m-%d")
                        .unwrap(),
                    color: row.get(3)?,
                    week_start: row.get(4)?,
                })
            },
        )
        .optional()
        .map_err(|e| ServerFnError::new(e.to_string()))
    })
}

pub async fn create_habit(habit: Habit) -> Result<(), ServerFnError> {
    with_db(|conn| {
        conn.execute(
            "INSERT INTO habits (id, title, start_date, color, week_start) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                habit.id.to_string(),
                &habit.title,
                habit.start_date.to_string(),
                &habit.color,
                &habit.week_start,
            ),
        )
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
    })
}

pub async fn update_habit(id: Uuid, updates: UpdateHabitRequest) -> Result<(), ServerFnError> {
    debug!(?id, ?updates, "Updating habit");
    
    with_db(|conn| {
        if let Some(title) = updates.title {
            debug!("Updating title to: {}", title);
            conn.execute(
                "UPDATE habits SET title = ?1 WHERE id = ?2",
                [&title, &id.to_string()],
            )
            .map_err(|e| {
                error!("Failed to update title: {}", e);
                ServerFnError::new(e.to_string())
            })?;
        }

        if let Some(start_date) = updates.start_date {
            debug!("Updating start_date to: {}", start_date);
            conn.execute(
                "UPDATE habits SET start_date = ?1 WHERE id = ?2",
                [&start_date, &id.to_string()],
            )
            .map_err(|e| {
                error!("Failed to update start_date: {}", e);
                ServerFnError::new(e.to_string())
            })?;
        }

        if let Some(color) = updates.color {
            debug!("Updating color to: {}", color);
            conn.execute(
                "UPDATE habits SET color = ?1 WHERE id = ?2",
                [&color, &id.to_string()],
            )
            .map_err(|e| {
                error!("Failed to update color: {}", e);
                ServerFnError::new(e.to_string())
            })?;
        }

        if let Some(week_start) = updates.week_start {
            debug!("Updating week_start to: {}", week_start);
            conn.execute(
                "UPDATE habits SET week_start = ?1 WHERE id = ?2",
                [&week_start, &id.to_string()],
            )
            .map_err(|e| {
                error!("Failed to update week_start: {}", e);
                ServerFnError::new(e.to_string())
            })?;
        }

        info!("Successfully updated habit {}", id);
        Ok(())
    })
}

pub async fn delete_habit(id: Uuid) -> Result<(), ServerFnError> {
    with_db(|conn| {
        conn.execute("DELETE FROM habits WHERE id = ?1", [id.to_string()])
            .map(|_| ())
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        conn.execute(
            "DELETE FROM completed_days WHERE habit_id = ?1",
            [id.to_string()],
        )
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(())
    })
}

pub async fn get_completed_days(id: Uuid) -> Result<Vec<NaiveDate>, ServerFnError> {
    with_db(|conn| {
        let mut stmt = conn
            .prepare("SELECT date FROM completed_days WHERE habit_id = ?1")
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let dates = stmt
            .query_map([id.to_string()], |row| {
                let date_str: String = row.get(0)?;
                Ok(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap())
            })
            .map_err(|e| ServerFnError::new(e.to_string()))?
            .filter_map(Result::ok)
            .collect();

        Ok(dates)
    })
}

pub async fn save_completed_day(id: Uuid, date: NaiveDate) -> Result<(), ServerFnError> {
    with_db(|conn| {
        conn.execute(
            "INSERT OR REPLACE INTO completed_days (habit_id, date) VALUES (?1, ?2)",
            [&id.to_string(), &date.to_string()],
        )
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
    })
}

pub async fn delete_completed_day(id: Uuid, date: NaiveDate) -> Result<(), ServerFnError> {
    with_db(|conn| {
        conn.execute(
            "DELETE FROM completed_days WHERE habit_id = ?1 AND date = ?2",
            [&id.to_string(), &date.to_string()],
        )
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
    })
}

pub async fn update_habit_title(id: Uuid, title: String) -> Result<(), ServerFnError> {
    with_db(|conn| {
        conn.execute(
            "UPDATE habits SET title = ?1 WHERE id = ?2",
            [&title, &id.to_string()],
        )
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
    })
}

pub async fn update_habit_color(id: Uuid, color: String) -> Result<(), ServerFnError> {
    debug!("BEFORE COLOR UPDATE: Updating color to {} for habit {}", color, id);
    
    with_db(|conn| {
        let result = conn.execute(
            "UPDATE habits SET color = ?1 WHERE id = ?2",
            [&color, &id.to_string()],
        );
        
        match result {
            Ok(rows) => {
                debug!("COLOR UPDATE SUCCESS: Updated {} rows for habit {}", rows, id);
                if rows == 0 {
                    error!("COLOR UPDATE WARNING: No rows were updated for habit {}", id);
                }
                Ok(())
            }
            Err(e) => {
                error!("COLOR UPDATE ERROR: Failed to update color: {}", e);
                Err(ServerFnError::new(e.to_string()))
            }
        }
    })
}

pub async fn update_habit_start_date(id: Uuid, start_date: NaiveDate) -> Result<(), ServerFnError> {
    with_db(|conn| {
        conn.execute(
            "UPDATE habits SET start_date = ?1 WHERE id = ?2",
            [&start_date.to_string(), &id.to_string()],
        )
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
    })
}

pub async fn update_habit_week_start(id: Uuid, week_start: String) -> Result<(), ServerFnError> {
    with_db(|conn| {
        conn.execute(
            "UPDATE habits SET week_start = ?1 WHERE id = ?2",
            [&week_start, &id.to_string()],
        )
        .map(|_| ())
        .map_err(|e| ServerFnError::new(e.to_string()))
    })
}