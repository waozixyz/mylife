use crate::models::{MyLifeApp, Yaml};
use crate::ui::{BottomPanel, CentralPanel, TopPanel};
use crate::yaml_manager::{get_yaml, get_yaml_manager};
use dioxus::prelude::*;

use dioxus_logger::tracing::{error, info};
use crate::utils::compression::decode_and_decompress;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/?:y")]
    Home { y: String },
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
fn Home(y: String) -> Element {
    let yaml_state = use_signal(|| {
        #[cfg(target_arch = "wasm32")]
        {
            if !y.is_empty() {
                info!("Received compressed YAML parameter");
                if let Some(decompressed_str) = decode_and_decompress(&y) {
                    info!("Successfully decompressed YAML. Length: {}", decompressed_str.len());
                    info!("First 100 characters of decompressed YAML: {}", &decompressed_str[..100.min(decompressed_str.len())]);
                    
                    match serde_yaml::from_str::<Yaml>(&decompressed_str) {
                        Ok(new_yaml) => {
                            info!("Successfully parsed YAML");
                            return new_yaml;
                        },
                        Err(e) => {
                            error!("Failed to parse YAML: {}", e);
                            // Log the first few lines of the YAML for context
                            let context = decompressed_str.lines().take(5).collect::<Vec<_>>().join("\n");
                            error!("YAML parsing error context:\n{}", context);
                        }
                    }
                } else {
                    error!("Failed to decompress YAML");
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
        if !y.is_empty() {
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
