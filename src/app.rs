use dioxus::prelude::*;
use crate::models::{MyLifeApp, RuntimeConfig};
use uuid::Uuid;
use crate::config_manager::get_config;
use crate::ui::{TopPanel, BottomPanel, CentralPanel, EditLegendItem, SettingsWindow};
#[cfg(not(target_arch = "wasm32"))]
use crate::config_manager::{get_config_manager, get_available_configs};

#[cfg(not(target_arch = "wasm32"))]
use dioxus_logger::tracing::{Level, info, error, debug};

#[component]
pub fn App() -> Element {
    let mut app_state = use_signal(|| MyLifeApp {
        config: get_config(),
        view: "Lifetime".to_string(),
        selected_yaml: "default.yaml".to_string(),
        selected_legend_item: None,
        original_legend_item: None,
        selected_life_period: None,
        value: 0.0,
        show_settings: false,
        #[cfg(not(target_arch = "wasm32"))]
        yaml_files: get_available_configs(),
        #[cfg(target_arch = "wasm32")]
        yaml_content: String::new(),
        loaded_configs: Vec::new(),
        #[cfg(target_arch = "wasm32")]
        selected_config_index: 0,
        hovered_period: None,
        item_state: None,
        temp_start_date: String::new(),
    });

    use_effect(move || {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let configs = get_config_manager().get_available_configs();
            app_state.write().yaml_files = configs.unwrap_or_default();
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let loaded_configs = load_configs();
            app_state.write().loaded_configs = loaded_configs;
        }
    });

    use_context_provider(|| app_state);

    rsx! {
        style { {include_str!("../assets/main.css")} }
        style { {include_str!("../assets/input.css")} }

        div {
            class: "app-container",
            TopPanel {}
            CentralPanel {}
            BottomPanel {}
            EditLegendItem {}
            SettingsWindow {}
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_configs() -> Vec<(String, RuntimeConfig)> {
    let configs = get_config_manager().get_available_configs();
    configs
        .map(|configs| {
            configs
                .into_iter()
                .filter_map(|name| {
                    get_config_manager()
                        .load_config(&name)
                        .map_err(|e| {
                            error!("Failed to load config {}: {:?}", name, e);
                        })
                        .ok()
                        .map(|config| (name, config))
                })
                .collect()
        })
        .unwrap_or_default()
}