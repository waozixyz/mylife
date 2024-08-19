#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use eframe::egui;
#[cfg(not(target_arch = "wasm32"))]
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use crate::models::{Config, RuntimeConfig};
#[cfg(not(target_arch = "wasm32"))]
use crate::models::{Config, RuntimeConfig, RuntimeLifePeriod, RuntimeYearlyEvent};


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
    RuntimeConfig::from(config)
}

#[cfg(not(target_arch = "wasm32"))]
fn config_to_runtime_config(config: Config) -> RuntimeConfig {
    let runtime_life_periods = config.life_periods
        .into_iter()
        .map(|p| RuntimeLifePeriod {
            id: Uuid::new_v4(),
            name: p.name,
            start: p.start,
            color: p.color,
        })
        .collect();

    let runtime_yearly_events = config.yearly_events
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
