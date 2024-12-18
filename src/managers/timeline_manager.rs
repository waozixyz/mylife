use crate::models::timeline::{LifePeriod, LifePeriodEvent, Yaml};
use crate::storage::{get_path_manager, StorageConfig, StorageError, YamlStorage};
use once_cell::sync::Lazy;
use rfd::FileDialog;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
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

impl From<StorageError> for String {
    fn from(error: StorageError) -> Self {
        error.to_string()
    }
}

pub struct TimelineManager {
    current_name: Arc<RwLock<String>>,
    storage: Arc<RwLock<YamlStorage<Yaml>>>,

    last_modified: Arc<RwLock<SystemTime>>,
}

fn assign_ids(yaml: &mut Yaml) {
    for period in &mut yaml.life_periods {
        if period.id.is_none() {
            period.id = Some(Uuid::new_v4());
        }
        for event in &mut period.events {
            if event.id.is_none() {
                event.id = Some(Uuid::new_v4());
            }
        }
    }
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

        let current_name = "default".to_string();
        let path = get_path_manager().timeline_file(&current_name);

        // Get the initial last modified time
        let last_modified = path
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .unwrap_or_else(SystemTime::now);

        let storage = YamlStorage::with_config_and_default(path, config, Some(default_yaml))?;

        Ok(Self {
            current_name: Arc::new(RwLock::new(current_name)),
            storage: Arc::new(RwLock::new(storage)),
            last_modified: Arc::new(RwLock::new(last_modified)),
        })
    }

    pub async fn get_available_timelines(&self) -> Vec<String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(entries) = std::fs::read_dir(get_path_manager().timelines_dir()) {
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
    pub async fn get_timeline_by_name(&self, name: &str) -> Result<Yaml, String> {
        let path = get_path_manager().timeline_file(name);
        debug!("Loading timeline '{}' from: {:?}", name, path);

        if !path.exists() {
            error!("Timeline file does not exist at path: {:?}", path);
            return Err(format!("Timeline file '{}' does not exist", name));
        }

        match std::fs::read_to_string(&path) {
            Ok(content) => {
                debug!(
                    "Successfully read file content for '{}', content length: {}",
                    name,
                    content.len()
                );
                match serde_yaml::from_str::<Yaml>(&content) {
                    Ok(mut yaml) => {
                        debug!("Successfully parsed YAML for timeline '{}'", name);
                        // Assign IDs to periods that don't have them
                        for period in &mut yaml.life_periods {
                            if period.id.is_none() {
                                period.id = Some(Uuid::new_v4());
                                debug!("Assigned new ID {:?} to period {}", period.id, period.name);
                            }
                        }
                        Ok(yaml)
                    }
                    Err(e) => {
                        error!("Failed to parse YAML for timeline '{}': {}", name, e);
                        Err(format!("Failed to parse timeline '{}': {}", name, e))
                    }
                }
            }
            Err(e) => {
                error!("Failed to read timeline file '{}': {}", name, e);
                Err(format!("Failed to read timeline '{}': {}", name, e))
            }
        }
    }

    pub async fn select_timeline(&self, name: &str) -> Result<Yaml, String> {
        debug!("Switching storage to timeline: {}", name);
        let config = StorageConfig {
            extension: String::from("yaml"),
            ..Default::default()
        };

        let path = get_path_manager().timeline_file(name);
        debug!("New storage path: {:?}", path);

        // Get or create the yaml content
        let yaml = match self.get_timeline_by_name(name).await {
            Ok(yaml) => yaml,
            Err(_) => {
                let mut default: Yaml = serde_yaml::from_str(DEFAULT_TIMELINE)
                    .map_err(|e| format!("Failed to parse default timeline: {}", e))?;
                default.name = name.to_string();
                default
            }
        };

        // Update last modified time
        if let Ok(metadata) = path.metadata() {
            if let Ok(modified) = metadata.modified() {
                let mut last_modified = self.last_modified.write().await;
                *last_modified = modified;
            }
        }

        // Create new storage
        let new_storage = YamlStorage::with_config_and_default(path, config, Some(yaml.clone()))?;

        // Update the manager state
        {
            let mut current_name = self.current_name.write().await;
            *current_name = name.to_string();
        }
        {
            let mut storage = self.storage.write().await;
            *storage = new_storage;
        }

        Ok(yaml)
    }

    pub async fn check_for_file_changes(&self) -> Result<Option<Yaml>, String> {
        let current_name = self.current_name.read().await;
        let path = get_path_manager().timeline_file(&current_name);

        let file_modified = path.metadata().ok().and_then(|m| m.modified().ok());

        if let Some(file_time) = file_modified {
            let last_check = *self.last_modified.read().await;

            if file_time > last_check {
                debug!("File change detected for timeline: {}", current_name);

                // Load the new content
                let new_yaml = self.get_timeline_by_name(&current_name).await?;

                // Update the storage and last modified time
                {
                    let storage = self.storage.read().await;
                    storage.write(|store| *store = new_yaml.clone()).await?;
                }
                {
                    let mut last_modified = self.last_modified.write().await;
                    *last_modified = file_time;
                }

                return Ok(Some(new_yaml));
            }
        }

        Ok(None)
    }
    pub async fn get_timeline(&self) -> Result<Yaml, String> {
        let storage = self.storage.read().await;
        storage.get_data().await.map_err(|e| e.to_string())
    }

    pub async fn update_timeline(&self, yaml: &Yaml) -> Result<(), String> {
        let storage = self.storage.read().await;
        debug!(
            "Updating timeline {} at {:?}",
            self.current_name.read().await,
            storage.file_path()
        );
        storage
            .write(|store| {
                let mut new_yaml = yaml.clone();
                assign_ids(&mut new_yaml);
                *store = new_yaml;
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn add_life_period(&self, mut period: LifePeriod) -> Result<(), String> {
        if period.id.is_none() {
            period.id = Some(Uuid::new_v4());
        }
        let storage = self.storage.read().await;
        debug!("Adding life period: {:?}", period);
        storage
            .write(|store| {
                store.life_periods.push(period);
                store.life_periods.sort_by(|a, b| a.start.cmp(&b.start));
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn update_life_period(&self, period: LifePeriod) -> Result<(), String> {
        let storage = self.storage.read().await;
        debug!("Updating life period: {:?}", period);
        storage
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
        let storage = self.storage.read().await;
        debug!("Deleting life period: {}", id);
        storage
            .write(|store| {
                store.life_periods.retain(|p| p.id != Some(id));
            })
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn add_event(
        &self,
        period_id: Uuid,
        mut event: LifePeriodEvent,
    ) -> Result<(), String> {
        if event.id.is_none() {
            event.id = Some(Uuid::new_v4());
        }
        let storage = self.storage.read().await;
        debug!("Adding event to period {}: {:?}", period_id, event);
        storage
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
        let storage = self.storage.read().await;
        debug!("Updating event in period {}: {:?}", period_id, event);
        storage
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
        let storage = self.storage.read().await;
        debug!("Deleting event {} from period {}", event_id, period_id);
        storage
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

    pub async fn get_period_events(&self, period_id: Uuid) -> Result<Vec<LifePeriodEvent>, String> {
        let storage = self.storage.read().await;
        storage
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
        let storage = self.storage.read().await;
        storage.force_save().await.map_err(|e| e.to_string())
    }

    pub async fn reload(&self) -> Result<(), String> {
        let storage = self.storage.read().await;
        storage.reload().await.map_err(|e| e.to_string())
    }

    pub async fn import_timeline(&self) -> Option<(String, Yaml)> {
        #[cfg(target_arch = "wasm32")]
        {
            None
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(file_path) = FileDialog::new()
                .add_filter("YAML", &["yaml", "yml"])
                .pick_file()
            {
                let content = std::fs::read_to_string(&file_path).ok()?;
                let yaml: Yaml = serde_yaml::from_str(&content).ok()?;
                let name = file_path.file_stem()?.to_str()?.to_string();
                Some((name, yaml))
            } else {
                None
            }
        }
    }

    pub async fn export_timeline(&self, yaml: &Yaml) -> Result<(), String> {
        #[cfg(target_arch = "wasm32")]
        {
            Ok(())
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(file_path) = FileDialog::new()
                .set_file_name("timeline.yaml")
                .add_filter("YAML", &["yaml", "yml"])
                .save_file()
            {
                let content = serde_yaml::to_string(yaml).map_err(|e| e.to_string())?;
                std::fs::write(file_path, content).map_err(|e| e.to_string())
            } else {
                Ok(())
            }
        }
    }
}

static TIMELINE_MANAGER: Lazy<TimelineManager> =
    Lazy::new(|| TimelineManager::new().expect("Failed to create timeline manager"));

pub fn get_timeline_manager() -> &'static TimelineManager {
    &*TIMELINE_MANAGER
}
