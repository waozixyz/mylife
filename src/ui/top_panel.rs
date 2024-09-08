use crate::models::{MyLifeApp, Yaml};
use crate::yaml_manager::{get_available_yamls, get_yaml_manager, save_yaml};
use dioxus::prelude::*;

use crate::yaml_manager::import_yaml;

use dioxus_logger::tracing::{error, info};

#[component]
pub fn TopPanel() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    let mut yaml_state = use_context::<Signal<Yaml>>();

    let options = get_available_yamls();

    let load_yaml = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use_future(move || async move {
                if let Some((name, new_yaml)) = import_yaml().await {
                    yaml_state.set(new_yaml.clone());
                    app_state.write().selected_yaml = name.clone();
                    app_state.write().loaded_yamls.push((name, new_yaml));
                    info!("YAML loaded and state updated");
                } else {
                    error!("Failed to import YAML");
                }
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some((name, new_yaml)) = import_yaml() {
                yaml_state.set(new_yaml.clone());
                app_state.write().selected_yaml = name.clone();
                app_state.write().loaded_yamls.push((name, new_yaml));
                info!("YAML loaded and state updated");
            } else {
                error!("Failed to import YAML");
            }
        }
    };

    let buttons = {
        #[cfg(target_arch = "wasm32")]
        rsx! {
            button {
                onclick: load_yaml,
                "📥 Load YAML"
            }
            button {
                onclick: move |_| {
                    if let Err(e) = save_yaml(&yaml_state(), &app_state().selected_yaml) {
                        error!("Failed to save YAML: {}", e);
                    }
                },
                "💾 Save YAML"
            }
            button {
                onclick: move |_| {
                    let yaml_content = serde_yaml::to_string(&yaml_state()).unwrap_or_default();
                    let encoded_yaml = js_sys::encode_uri_component(&yaml_content);
                    let current_url = web_sys::window().unwrap().location().href().unwrap();
                    let base_url = web_sys::Url::new(&current_url).unwrap();
                    let share_url = format!("{}?yaml={}", base_url.origin(), encoded_yaml);

                    web_sys::window().unwrap().open_with_url_and_target(&share_url, "_blank").unwrap();
                },
                "📤 Share"
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        rsx! {
            button {
                onclick: load_yaml,
                "📥 Load YAML"
            }
            button {
                onclick: move |_| {
                    if let Err(e) = save_yaml(&yaml_state(), &app_state().selected_yaml) {
                        error!("Failed to save YAML: {}", e);
                    }
                },
                "💾 Save YAML"
            }
            button {
                onclick: move |_| {
                    std::process::exit(0);
                },
                "🚪 Quit"
            }
        }
    };

    rsx! {
        div {
            class: "top-panel",
            // Back button (only in EventView)
            if app_state().view == "EventView" {
                button {
                    onclick: move |_| {
                        app_state.write().view = "Lifetime".to_string();
                    },
                    span { "⬅" },
                }
            }

            // Configuration selector and Life Expectancy (only in Lifetime view)
            if app_state().view == "Lifetime" {
                div {
                    class: "flex",
                    select {
                        value: "{app_state().selected_yaml}",
                        onchange: move |evt| {
                            let selected_yaml = evt.value().to_string();
                            app_state.write().selected_yaml = selected_yaml.clone();
                            if let Ok(new_yaml) = get_yaml_manager().load_yaml(&selected_yaml) {
                                yaml_state.set(new_yaml);
                            } else {
                                error!("Failed to load yaml: {}", selected_yaml);
                            }
                        },
                        for option in options.iter() {
                            option {
                                value: "{option}",
                                selected: *option == app_state().selected_yaml,
                                "{option}"
                            }
                        }
                    }

                    select {
                        value: "{yaml_state().life_expectancy}",
                        onchange: move |evt| {
                            if let Ok(value) = evt.value().parse() {
                                yaml_state().life_expectancy = value;
                            } else {
                                error!("Failed to parse life expectancy: {}", evt.value());
                            }
                        },
                        for year in 60..=120 {
                            option {
                                value: "{year}",
                                selected: year == yaml_state().life_expectancy,
                                "{year}"
                            }
                        }
                    }
                }
                div {
                    { buttons }
                }
            }
        }
    }
}
