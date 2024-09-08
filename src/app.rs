use crate::models::{MyLifeApp, Yaml};
use crate::ui::{BottomPanel, CentralPanel, TopPanel};
use crate::yaml_manager::{get_yaml, get_yaml_manager};
use dioxus::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use dioxus_logger::tracing::error;

#[component]
pub fn App() -> Element {
    let mut yaml = use_signal(|| get_yaml());

    let app_state = use_signal(|| {
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
    });
    use_effect(move || {
        let mut app_state = use_context::<Signal<MyLifeApp>>();

        let loaded_yamls = load_yamls();
        app_state.write().loaded_yamls = loaded_yamls;
    });

    use_context_provider(|| app_state);
    use_context_provider(|| yaml);

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
                    log::error!("Failed to load yaml {}: {:?}", name, e);
                })
                .ok()
                .map(|yaml| (name, yaml))
        })
        .collect()
}