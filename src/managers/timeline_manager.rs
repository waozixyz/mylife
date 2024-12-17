use crate::models::timeline::{LifePeriod, LifePeriodEvent, Yaml};
use crate::storage::{get_path_manager, StorageConfig, YamlStorage};
use chrono::{Local, NaiveDate};
use once_cell::sync::Lazy;
use rfd::FileDialog;
use tracing::{debug, error};
use uuid::Uuid;

const DEFAULT_TIMELINE: &str = r#"
name: John Doe
date_of_birth: 2000-01
life_expectancy: 92
life_periods:
- name: Childhood
  start: 2000-01
  color: '#5100FF'
  events: []
- name: Teenage Years
  start: 2013-01
  color: '#00FF77'
  events: []
- name: University
  start: 2018-01
  color: '#00BEFF'
  events: []
- name: Career Growth
  start: 2022-01
  color: '#FFFF00'
  events: []
- name: Working
  start: 2024-01
  color: '#FF9E00'
  events: []
routines: []
"#;

pub struct TimelineManager {
    storage: YamlStorage<Yaml>,
}

impl TimelineManager {
    pub fn new() -> Result<Self, String> {
        debug!("Initializing TimelineManager with default timeline");
        let default_yaml: Yaml = serde_yaml::from_str(DEFAULT_TIMELINE)
            .map_err(|e| format!("Failed to parse default timeline: {}", e))?;

        let config = StorageConfig {
            extension: String::from("yaml"),
            ..Default::default()
        };
        debug!("Storage config: {:?}", config);

        let path = get_path_manager().timeline_file("default");
        debug!("Timeline file path: {:?}", path);

        let storage =
            YamlStorage::with_config_and_default(path.clone(), config, Some(default_yaml))
                .map_err(|e| {
                    error!("Failed to create storage at {:?}: {}", path, e);
                    e.to_string()
                })?;

        debug!("Storage created successfully");
        Ok(Self { storage })
    }
    pub async fn get_available_timelines(&self) -> Vec<String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(entries) = std::fs::read_dir(get_path_manager().timelines_dir()) {
                // Changed from timeline_dir to timelines_dir
                entries
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        if path.extension()?.to_str()? == "yaml" {
                            path.file_stem()?.to_str().map(String::from)
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                vec!["default".to_string()]
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            vec!["default".to_string()]
        }
    }

    // Add this new function
    pub async fn get_timeline_by_name(&self, name: &str) -> Result<Yaml, String> {
        let path = get_path_manager().timeline_file(name);
        let config = StorageConfig {
            extension: String::from("yaml"),
            ..Default::default()
        };

        let storage =
            YamlStorage::with_config_and_default(path.clone(), config, None).map_err(|e| {
                error!("Failed to create storage for {}: {}", name, e);
                e.to_string()
            })?;

        storage.get_data().await.map_err(|e| e.to_string())
    }

    pub async fn get_timeline(&self) -> Result<Yaml, String> {
        self.storage.get_data().await.map_err(|e| e.to_string())
    }

    pub async fn update_timeline(&self, yaml: &Yaml) -> Result<(), String> {
        debug!("Updating timeline");
        self.storage
            .write(|store| *store = yaml.clone())
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn add_life_period(&self, period: LifePeriod) -> Result<(), String> {
        debug!("Adding life period: {:?}", period);
        self.storage
            .write(|store| {
                store.life_periods.push(period);
                store.life_periods.sort_by(|a, b| a.start.cmp(&b.start));
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_life_period(&self, period: LifePeriod) -> Result<(), String> {
        debug!("Updating life period: {:?}", period);
        self.storage
            .write(|store| {
                if let Some(existing) = store.life_periods.iter_mut().find(|p| p.id == period.id) {
                    *existing = period;
                    store.life_periods.sort_by(|a, b| a.start.cmp(&b.start));
                }
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_life_period(&self, id: Uuid) -> Result<(), String> {
        debug!("Deleting life period: {}", id);
        self.storage
            .write(|store| {
                store.life_periods.retain(|p| p.id != Some(id));
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn add_event(&self, period_id: Uuid, event: LifePeriodEvent) -> Result<(), String> {
        debug!("Adding event to period {}: {:?}", period_id, event);
        self.storage
            .write(|store| {
                if let Some(period) = store
                    .life_periods
                    .iter_mut()
                    .find(|p| p.id == Some(period_id))
                {
                    period.events.push(event);
                    period.events.sort_by(|a, b| a.start.cmp(&b.start));
                }
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_event(
        &self,
        period_id: Uuid,
        event: LifePeriodEvent,
    ) -> Result<(), String> {
        debug!("Updating event in period {}: {:?}", period_id, event);
        self.storage
            .write(|store| {
                if let Some(period) = store
                    .life_periods
                    .iter_mut()
                    .find(|p| p.id == Some(period_id))
                {
                    if let Some(existing) = period.events.iter_mut().find(|e| e.id == event.id) {
                        *existing = event;
                        period.events.sort_by(|a, b| a.start.cmp(&b.start));
                    }
                }
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_event(&self, period_id: Uuid, event_id: Uuid) -> Result<(), String> {
        debug!("Deleting event {} from period {}", event_id, period_id);
        self.storage
            .write(|store| {
                if let Some(period) = store
                    .life_periods
                    .iter_mut()
                    .find(|p| p.id == Some(period_id))
                {
                    period.events.retain(|e| e.id != Some(event_id));
                }
            })
            .await
            .map_err(|e| e.to_string())
    }

    // Helper methods for event view
    pub async fn get_period_events(&self, period_id: Uuid) -> Result<Vec<LifePeriodEvent>, String> {
        self.storage
            .read(|store| {
                store
                    .life_periods
                    .iter()
                    .find(|p| p.id == Some(period_id))
                    .map(|p| p.events.clone())
                    .unwrap_or_default()
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn force_save(&self) -> Result<(), String> {
        self.storage.force_save().await.map_err(|e| e.to_string())
    }

    pub async fn reload(&self) -> Result<(), String> {
        self.storage.reload().await.map_err(|e| e.to_string())
    }

    pub async fn import_timeline(&self) -> Option<(String, Yaml)> {
        #[cfg(target_arch = "wasm32")]
        {
            // Web import logic
            // You'll need to implement this based on your requirements
            None
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Desktop import logic using file dialog
            if let Some(file_path) = FileDialog::new()
                .add_filter("YAML", &["yaml", "yml"])
                .pick_file()
            {
                let content = std::fs::read_to_string(&file_path).ok()?;
                let yaml: Yaml = serde_yaml::from_str(&content).ok()?;
                let name = file_path.file_name()?.to_str()?.to_string();
                Some((name, yaml))
            } else {
                None
            }
        }
    }

    pub async fn export_timeline(&self, yaml: &Yaml) -> Result<(), String> {
        #[cfg(target_arch = "wasm32")]
        {
            // Web export logic
            // Implement based on your requirements
            Ok(())
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Desktop export logic
            if let Some(file_path) = FileDialog::new()
                .set_file_name("timeline.yaml")
                .add_filter("YAML", &["yaml", "yml"])
                .save_file()
            {
                let content = serde_yaml::to_string(yaml).map_err(|e| e.to_string())?;
                std::fs::write(file_path, content).map_err(|e| e.to_string())
            } else {
                Ok(()) // User cancelled
            }
        }
    }
}

static TIMELINE_MANAGER: Lazy<TimelineManager> =
    Lazy::new(|| TimelineManager::new().expect("Failed to create timeline manager"));

pub fn get_timeline_manager() -> &'static TimelineManager {
    &*TIMELINE_MANAGER
}
