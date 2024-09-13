// src/state_manager.rs

use crate::models::{MyLifeApp, Yaml};
use crate::yaml_manager::{get_yaml, get_yaml_manager};
#[cfg(target_arch = "wasm32")]
use crate::utils::compression::decode_and_decompress;
use dioxus::prelude::*;
use dioxus_logger::tracing::error;

pub fn initialize_state(y: &str) -> (Yaml, MyLifeApp) {
    let yaml_state = if !y.is_empty() {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(decompressed_str) = decode_and_decompress(y) {
                match serde_yaml::from_str::<Yaml>(&decompressed_str) {
                    Ok(new_yaml) => new_yaml,
                    Err(e) => {
                        error!("Failed to parse YAML: {}", e);
                        let context = decompressed_str
                            .lines()
                            .take(5)
                            .collect::<Vec<_>>()
                            .join("\n");
                        error!("YAML parsing error context:\n{}", context);
                        get_yaml()
                    }
                }
            } else {
                error!("Failed to decompress YAML");
                get_yaml()
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        get_yaml()
    } else {
        get_yaml()
    };

    let mut app_state = initialize_app_state();
    if !y.is_empty() {
        app_state.selected_yaml = "Shared YAML".to_string();
    }

    (yaml_state, app_state)
}

fn initialize_app_state() -> MyLifeApp {
    let loaded_yamls = load_yamls();

    MyLifeApp {
        view: "Lifetime".to_string(),
        selected_yaml: "default.yaml".to_string(),
        selected_legend_item: None,
        original_legend_item: None,
        selected_life_period: None,
        value: 0.0,
        loaded_yamls,
        item_state: None,
        temp_start_date: String::new(),
        data_folder: "data".to_string(),
        screenshot_data: None,
    }
}

fn load_yamls() -> Vec<(String, Yaml)> {
    get_yaml_manager()
        .get_available_yamls()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|name| {
            get_yaml_manager()
                .load_yaml(&name)
                .map_err(|e| {
                    error!("Failed to load yaml {}: {:?}", name, e);
                })
                .ok()
                .map(|yaml| (name, yaml))
        })
        .collect()
}