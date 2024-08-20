use crate::models::{RuntimeLifePeriod, RuntimeYearlyEvent, Config, RuntimeConfig};
#[cfg(not(target_arch = "wasm32"))]
use crate::models::{LifePeriod, YearlyEvent};
#[cfg(target_arch = "wasm32")]
use crate::models::MyLifeApp;
use eframe::egui;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use uuid::Uuid;
use chrono::NaiveDate;



pub fn hex_to_color32(hex: &str) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    egui::Color32::from_rgb(r, g, b)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_yaml_files_in_data_folder() -> Vec<String> {
    let data_folder = Path::new("data");
    fs::read_dir(data_folder)
        .expect("Failed to read data folder")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "yaml" {
                Some(path.file_name()?.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_config(yaml_file: &str) -> RuntimeConfig {
    let yaml_path = Path::new("data").join(yaml_file);
    let yaml_content = fs::read_to_string(yaml_path)
        .unwrap_or_else(|_| panic!("Failed to read YAML file: {}", yaml_file));
    let config: Config = serde_yaml::from_str(&yaml_content)
        .unwrap_or_else(|_| panic!("Failed to parse YAML file: {}", yaml_file));

    config_to_runtime_config(config)
}

#[cfg(target_arch = "wasm32")]
pub fn load_config(yaml_content: &str) -> RuntimeConfig {
    let config: Config = serde_yaml::from_str(yaml_content).unwrap_or_default();
    config_to_runtime_config(config)
}

pub fn config_to_runtime_config(config: Config) -> RuntimeConfig {
    let runtime_life_periods = config
        .life_periods
        .into_iter()
        .map(|p| RuntimeLifePeriod {
            id: Uuid::new_v4(),
            name: p.name,
            start: p.start,
            color: p.color,
        })
        .collect();

    let runtime_yearly_events = config
        .yearly_events
        .into_iter()
        .map(|(year, events)| {
            let runtime_events = events
                .into_iter()
                .map(|e| RuntimeYearlyEvent {
                    id: Uuid::new_v4(),
                    color: e.color,
                    start: e.start,
                })
                .collect();
            (year, runtime_events)
        })
        .collect();

    RuntimeConfig {
        name: config.name,
        date_of_birth: config.date_of_birth,
        life_expectancy: config.life_expectancy,
        life_periods: runtime_life_periods,
        yearly_events: runtime_yearly_events,
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
            .map(|p| LifePeriod {
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
                        .map(|e| YearlyEvent {
                            color: e.color.clone(),
                            start: e.start.clone(),
                        })
                        .collect(),
                )
            })
            .collect(),
    }
}


pub fn calculate_centered_rect(available: egui::Rect, desired_size: egui::Vec2) -> egui::Rect {
    let size = egui::Vec2::new(
        desired_size.x.min(available.width()),
        desired_size.y.min(available.height()),
    );
    let pos = available.center() - (size / 2.0);
    egui::Rect::from_min_size(pos, size)
}

pub fn is_valid_date(date_str: &str) -> bool {
    NaiveDate::parse_from_str(&format!("{}-01", date_str), "%Y-%m-%d").is_ok()
}

pub fn color32_to_hex(color: egui::Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}


#[cfg(target_arch = "wasm32")]
pub async fn load_yaml() -> Option<MyLifeApp> {
    log::debug!("Entering load_yaml function");
    let file = rfd::AsyncFileDialog::new()
        .add_filter("YAML", &["yaml", "yml"])
        .pick_file()
        .await;

    log::debug!("File picked: {:?}", file.is_some());

    let file = file?;

    log::debug!("Reading file content");
    let content = file.read().await;
    log::debug!("File content read, length: {} bytes", content.len());

    match String::from_utf8(content) {
        Ok(yaml_content) => {
            log::debug!(
                "YAML content (first 100 chars): {}",
                &yaml_content[..yaml_content.len().min(100)]
            );
            match serde_yaml::from_str::<Config>(&yaml_content) {
                Ok(config) => {
                    log::debug!("YAML parsed successfully");
                    let new_config = config_to_runtime_config(config);
                    let config_name = file.file_name();
                    log::debug!("New config name: {}", config_name);
                    let mut new_app = MyLifeApp::default();
                    new_app
                        .loaded_configs
                        .push((config_name.clone(), new_config.clone()));
                    new_app.selected_config_index = new_app.loaded_configs.len() - 1;
                    new_app.config = new_config;
                    new_app.selected_yaml = config_name;
                    log::info!("YAML file loaded successfully");
                    Some(new_app)
                }
                Err(e) => {
                    log::error!("Failed to parse YAML content: {}. Using default config.", e);
                    None
                }
            }
        }
        Err(e) => {
            log::error!("Invalid UTF-8 in file: {}. Using default config.", e);
            None
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn save_yaml(app: &MyLifeApp) {
    let config = Config::from(&app.config);
    let yaml_content = serde_yaml::to_string(&config).unwrap();

    // Use the web_sys crate to create a Blob and download it
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, HtmlAnchorElement, Url};

    let blob = Blob::new_with_str_sequence(&js_sys::Array::of1(&yaml_content.into()))
        .expect("Failed to create Blob");
    let url = Url::create_object_url_with_blob(&blob).expect("Failed to create object URL");

    let document = web_sys::window().unwrap().document().unwrap();
    let anchor: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();

    anchor.set_href(&url);
    anchor.set_download("config.yaml");
    anchor.click();

    Url::revoke_object_url(&url).expect("Failed to revoke object URL");
}
