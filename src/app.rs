use dioxus::prelude::*;
use crate::models::{MyLifeApp, Yaml};
use crate::yaml_manager::get_yaml;
use crate::ui::{TopPanel, BottomPanel, CentralPanel};
#[cfg(not(target_arch = "wasm32"))]
use crate::yaml_manager::{get_yaml_manager, get_available_yamls};

#[cfg(not(target_arch = "wasm32"))]
use dioxus_logger::tracing::error;

#[component]
pub fn App() -> Element {
    let mut app_state = use_signal(|| MyLifeApp {
        yaml: get_yaml(),
        view: "Lifetime".to_string(),
        selected_yaml: "default.yaml".to_string(),
        selected_legend_item: None,
        original_legend_item: None,
        selected_life_period: None,
        value: 0.0,
        #[cfg(not(target_arch = "wasm32"))]
        yaml_files: get_available_yamls(),
        #[cfg(target_arch = "wasm32")]
        yaml_content: String::new(),
        loaded_yamls: Vec::new(),
        #[cfg(target_arch = "wasm32")]
        selected_yaml_index: 0,
        item_state: None,
        temp_start_date: String::new(),
        data_folder: "data".to_string(),
    });

    use_effect(move || {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let yamls = get_yaml_manager().get_available_yamls();
            app_state.write().yaml_files = yamls.unwrap_or_default();
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            let loaded_yamls = load_yamls();
            app_state.write().loaded_yamls = loaded_yamls;
        }
    });

    use_context_provider(|| app_state);

    rsx! {
        style { {include_str!("../assets/input.css")} }
        style { {include_str!("../assets/modal.css")} }
        style { {include_str!("../assets/views.css")} }
        style { {include_str!("../assets/items.css")} }
        style { {include_str!("../assets/main.css")} }

        div {
            class: "app-container",
            TopPanel {}
            CentralPanel {}
            BottomPanel {}
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_yamls() -> Vec<(String, Yaml)> {
    let yamls = get_yaml_manager().get_available_yamls();
    yamls
        .map(|yamls| {
            yamls
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
        })
        .unwrap_or_default()
}