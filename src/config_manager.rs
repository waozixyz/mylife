use crate::models::{Config, RuntimeConfig, RuntimeLifePeriod, RuntimeYearlyEvent};
use std::io;
use uuid::Uuid;

#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{Blob, HtmlAnchorElement, Url};

pub trait ConfigManager {
    #[cfg(not(target_arch = "wasm32"))]
    fn load_config(&self, yaml_file: &str) -> io::Result<RuntimeConfig>;
    fn save_config(&self, config: &RuntimeConfig, yaml_file: &str) -> io::Result<()>;
    #[cfg(not(target_arch = "wasm32"))]
    fn get_available_configs(&self) -> io::Result<Vec<String>>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeConfigManager {
    data_folder: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeConfigManager {
    pub fn new(data_folder: String) -> Self {
        Self { data_folder }
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl ConfigManager for NativeConfigManager {
    fn load_config(&self, yaml_file: &str) -> io::Result<RuntimeConfig> {
        let yaml_path = std::path::Path::new(&self.data_folder).join(yaml_file);
        let yaml_content = std::fs::read_to_string(yaml_path)?;
        let config: Config = serde_yaml::from_str(&yaml_content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(config_to_runtime_config(config))
    }
    fn save_config(&self, config: &RuntimeConfig, yaml_file: &str) -> io::Result<()> {
        let config = runtime_config_to_config(config);
        let yaml_content = serde_yaml::to_string(&config)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let yaml_path = std::path::Path::new(&self.data_folder).join(yaml_file);
        std::fs::write(yaml_path, yaml_content)
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn get_available_configs(&self) -> io::Result<Vec<String>> {
        let data_folder = Path::new(&self.data_folder);
        Ok(fs::read_dir(data_folder)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()? == "yaml" {
                    Some(path.file_name()?.to_string_lossy().into_owned())
                } else {
                    None
                }
            })
            .collect())
    }
}

#[cfg(target_arch = "wasm32")]
pub struct WasmConfigManager;

#[cfg(target_arch = "wasm32")]
impl ConfigManager for WasmConfigManager {
    fn save_config(&self, config: &RuntimeConfig, _yaml_file: &str) -> io::Result<()> {
        let config = Config::from(config);
        let yaml_content = serde_yaml::to_string(&config)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let blob = Blob::new_with_str_sequence(&js_sys::Array::of1(&yaml_content.into()))
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create Blob"))?;
        let url = Url::create_object_url_with_blob(&blob)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create object URL"))?;

        let document = web_sys::window()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get window"))?
            .document()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get document"))?;
        let anchor: HtmlAnchorElement = document
            .create_element("a")
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create anchor element"))?
            .dyn_into()
            .map_err(|_| {
                io::Error::new(io::ErrorKind::Other, "Failed to cast to HtmlAnchorElement")
            })?;

        anchor.set_href(&url);
        anchor.set_download("config.yaml");
        anchor.click();

        Url::revoke_object_url(&url)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to revoke object URL"))?;

        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn get_available_configs(&self) -> io::Result<Vec<String>> {
        Ok(vec!["Default".to_string()])
    }
}

pub fn get_config_manager() -> Box<dyn ConfigManager> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::new(NativeConfigManager::new("data".to_string()))
    }
    #[cfg(target_arch = "wasm32")]
    {
        Box::new(WasmConfigManager)
    }
}

pub fn config_to_runtime_config(config: Config) -> RuntimeConfig {
    RuntimeConfig {
        name: config.name,
        date_of_birth: config.date_of_birth,
        life_expectancy: config.life_expectancy,
        life_periods: config
            .life_periods
            .into_iter()
            .map(|p| RuntimeLifePeriod {
                id: Uuid::new_v4(),
                name: p.name,
                start: p.start,
                color: p.color,
            })
            .collect(),
        yearly_events: config
            .yearly_events
            .into_iter()
            .map(|(year, events)| {
                (
                    year,
                    events
                        .into_iter()
                        .map(|e| RuntimeYearlyEvent {
                            id: Uuid::new_v4(),
                            name: e.name,
                            color: e.color,
                            start: e.start,
                        })
                        .collect(),
                )
            })
            .collect(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn runtime_config_to_config(runtime_config: &RuntimeConfig) -> Config {
    Config {
        name: runtime_config.name.clone(),
        date_of_birth: runtime_config.date_of_birth.clone(),
        life_expectancy: runtime_config.life_expectancy,
        life_periods: runtime_config
            .life_periods
            .iter()
            .map(|p| crate::models::LifePeriod {
                name: p.name.clone(),
                start: p.start.clone(),
                color: p.color.clone(),
            })
            .collect(),
        yearly_events: runtime_config
            .yearly_events
            .iter()
            .map(|(year, events)| {
                (
                    *year,
                    events
                        .iter()
                        .map(|e| crate::models::YearlyEvent {
                            name: e.name.clone(),
                            color: e.color.clone(),
                            start: e.start.clone(),
                        })
                        .collect(),
                )
            })
            .collect(),
    }
}
