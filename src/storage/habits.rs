use crate::models::habit::{Habit, HabitData};
use crate::storage::{file_manager::FileManager, paths::get_path_manager};
use chrono::NaiveDate;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{debug, error, info};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HabitsStore {
    habits: HashMap<Uuid, HabitData>,
}

pub struct HabitManager {
    file_manager: FileManager<HabitsStore>,
}

impl HabitManager {
    pub fn new(file_path: PathBuf) -> Result<Self, String> {
        Ok(Self {
            file_manager: FileManager::new(file_path).map_err(|e| e.to_string())?,
        })
    }

    pub async fn get_all_habits(&self) -> Result<Vec<(Uuid, HabitData)>, String> {
        debug!("Getting all habits");
        self.file_manager
            .read(|store| store.habits.iter().map(|(k, v)| (*k, v.clone())).collect())
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_habit(&self, id: Uuid) -> Result<Option<HabitData>, String> {
        debug!("Getting habit with id: {}", id);
        self.file_manager
            .read(|store| store.habits.get(&id).cloned())
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn create_habit(&self, id: Uuid, data: HabitData) -> Result<(), String> {
        debug!("Creating habit with id: {}", id);
        self.file_manager
            .write(|store| {
                store.habits.insert(id, data);
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_habit(&self, id: Uuid, data: HabitData) -> Result<(), String> {
        debug!("Updating habit with id: {}", id);
        self.file_manager
            .write(|store| {
                if store.habits.contains_key(&id) {
                    store.habits.insert(id, data);
                }
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_habit(&self, id: Uuid) -> Result<(), String> {
        debug!("Deleting habit with id: {}", id);
        self.file_manager
            .write(|store| {
                store.habits.remove(&id);
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn mark_day(&self, id: Uuid, date: NaiveDate) -> Result<(), String> {
        debug!("Marking day {} for habit {}", date, id);
        self.file_manager
            .write(|store| {
                if let Some(habit) = store.habits.get_mut(&id) {
                    if !habit.completed_days.contains(&date) {
                        habit.completed_days.push(date);
                        habit.completed_days.sort();
                    }
                }
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn unmark_day(&self, id: Uuid, date: NaiveDate) -> Result<(), String> {
        debug!("Unmarking day {} for habit {}", date, id);
        self.file_manager
            .write(|store| {
                if let Some(habit) = store.habits.get_mut(&id) {
                    habit.completed_days.retain(|&d| d != date);
                }
            })
            .await
            .map_err(|e| e.to_string())
    }
}
static HABIT_MANAGER: Lazy<HabitManager> = Lazy::new(|| HabitManager {
    file_manager: FileManager::new(get_path_manager().habits_file())
        .expect("Failed to create habit manager"),
});

pub fn get_habit_manager() -> &'static HabitManager {
    &*HABIT_MANAGER
}
