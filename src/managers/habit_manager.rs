// managers/habit_manager.rs
use crate::models::habit::{Habit, HabitData};
use crate::storage::{get_path_manager, JsonStorage, StorageResult};
use chrono::NaiveDate;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HabitsStore {
    habits: HashMap<Uuid, HabitData>,
}

pub struct HabitManager {
    storage: JsonStorage<HabitsStore>,
}

impl HabitManager {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            storage: JsonStorage::new(get_path_manager().habits_file())
                .map_err(|e| e.to_string())?,
        })
    }

    pub async fn get_all_habits(&self) -> Result<Vec<(Uuid, HabitData)>, String> {
        debug!("Getting all habits");
        self.storage
            .read(|store| store.habits.iter().map(|(k, v)| (*k, v.clone())).collect())
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn get_habit(&self, id: Uuid) -> Result<Option<HabitData>, String> {
        debug!("Getting habit with id: {}", id);
        self.storage
            .read(|store| store.habits.get(&id).cloned())
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn create_habit(&self, id: Uuid, data: HabitData) -> Result<(), String> {
        debug!("Creating habit with id: {}", id);
        self.storage
            .write(|store| {
                store.habits.insert(id, data);
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_habit(&self, id: Uuid, data: HabitData) -> Result<(), String> {
        debug!("Updating habit with id: {}", id);
        self.storage
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
        self.storage
            .write(|store| {
                store.habits.remove(&id);
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn mark_day(&self, id: Uuid, date: NaiveDate) -> Result<(), String> {
        debug!("Marking day {} for habit {}", date, id);
        self.storage
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
        self.storage
            .write(|store| {
                if let Some(habit) = store.habits.get_mut(&id) {
                    habit.completed_days.retain(|&d| d != date);
                }
            })
            .await
            .map_err(|e| e.to_string())
    }

    // Additional helper methods could go here
    pub async fn force_save(&self) -> Result<(), String> {
        self.storage.force_save().await.map_err(|e| e.to_string())
    }

    pub async fn reload(&self) -> Result<(), String> {
        self.storage.reload().await.map_err(|e| e.to_string())
    }
}

static HABIT_MANAGER: Lazy<HabitManager> =
    Lazy::new(|| HabitManager::new().expect("Failed to create habit manager"));

pub fn get_habit_manager() -> &'static HabitManager {
    &*HABIT_MANAGER
}
