use crate::models::{MyLifeApp, Yaml};
use crate::ui::{BottomPanel, CentralPanel, TopPanel};
use crate::yaml_manager::{get_yaml, get_yaml_manager};
use dioxus::prelude::*;

use dioxus_logger::tracing::{error, info};

#[cfg(target_arch = "wasm32")]
use js_sys;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/?:yaml")]
    Home { yaml: String },
}

#[component]
pub fn App() -> Element {
    rsx! {
        style { {include_str!("../assets/input.css")} }
        style { {include_str!("../assets/modal.css")} }
        style { {include_str!("../assets/views.css")} }
        style { {include_str!("../assets/items.css")} }
        style { {include_str!("../assets/main.css")} }

        Router::<Route> {}
    }
}

#[component]
fn Home(yaml: String) -> Element {
    let yaml_state = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        {
            if !yaml.is_empty() {
                info!("Received YAML parameter: {}", yaml);
                if let Ok(decoded_yaml) = js_sys::decode_uri_component(&yaml) {
                    info!("Decoded YAML: {}", decoded_yaml);
                    if let Ok(new_yaml) =
                        serde_yaml::from_str(&decoded_yaml.as_string().unwrap_or_default())
                    {
                        info!("Successfully parsed YAML");
                        return new_yaml;
                    } else {
                        info!("Failed to parse YAML");
                    }
                } else {
                    info!("Failed to decode YAML");
                }
            }
        }

        info!("Using default YAML");
        get_yaml()
    });

    let app_state = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        let mut state = initialize_app_state();
        #[cfg(not(target_arch = "wasm32"))]
        let state = initialize_app_state();

        #[cfg(target_arch = "wasm32")]
        if !yaml.is_empty() {
            state.selected_yaml = "Shared YAML".to_string();
            info!("Initialized app state with Shared YAML");
        } else {
            info!("Initialized app state with default YAML");
        }
        state
    });

    use_context_provider(|| app_state);
    use_context_provider(|| yaml_state);

    rsx! {
        div {
            class: "app-container",
            TopPanel {}
            CentralPanel {}
            BottomPanel {}
        }
    }
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
                    #[cfg(not(target_arch = "wasm32"))]
                    error!("Failed to load yaml {}: {:?}", name, e);
                    #[cfg(target_arch = "wasm32")]
                    error!("Failed to load yaml {}: {:?}", name, e);
                })
                .ok()
                .map(|yaml| (name, yaml))
        })
        .collect()
}
