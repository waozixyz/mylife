use dioxus::prelude::*;
use crate::models::{RuntimeConfig, MyLifeApp};
use crate::config_manager::get_config_manager;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
use crate::config_manager::load_config_async;

#[cfg(not(target_arch = "wasm32"))]
use dioxus_logger::tracing::{info, error, debug};

#[component]
pub fn TopPanel() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    
    info!("Current view: {:?}", app_state().view);
    info!("Selected YAML: {:?}", app_state().selected_yaml);

    info!("Rendering TopPanel");

    debug!("Context values in TopPanel:");
    debug!("  config: {:?}", app_state().config);
    debug!("  view: {:?}", app_state().view);
    debug!("  selected_yaml: {:?}", app_state().selected_yaml);
    debug!("  show_settings: {:?}", app_state().show_settings);
    debug!("  loaded_configs: {:?}", app_state().loaded_configs);

    #[cfg(target_arch = "wasm32")]
    let mut selected_config_index = use_context::<Signal<usize>>();

    #[cfg(target_arch = "wasm32")]
    let options = get_config_manager().get_available_configs().unwrap_or_default();
    
    #[cfg(not(target_arch = "wasm32"))]
    let options: Vec<String> = {
        debug!("Creating options for config selector");
        let mut config_names = Vec::new();
        for (name, _) in app_state().loaded_configs.iter() {
            config_names.push(name.clone());
        }
        debug!("Available options: {:?}", config_names);
        config_names
    };

    let buttons = {
        #[cfg(target_arch = "wasm32")]
        rsx! {
            button {
                onclick: move |_| {
                    spawn_local(async move {
                        if let Some((name, new_config)) = load_config_async().await {
                            app_state.write().config = new_config;
                            app_state.write().selected_yaml = name;
                        } else {
                            error!("Failed to load YAML configuration");
                        }
                    });
                },
                "Load YAML"
            }
            button {
                onclick: move |_| {
                    if let Err(e) = get_config_manager().save_config(&app_state().config, &app_state().selected_yaml) {
                        error!("Failed to save config: {}", e);
                    }
                },
                "Save YAML"
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        rsx! {
            button {
                onclick: move |_| {
                    info!("Quit button clicked");
                    std::process::exit(0);
                },
                "Quit"
            }
        }
    };

    info!("Rendering TopPanel RSX");

    rsx! {
        div {
            class: "top-panel",
            
            div {

                class: "top-row",
                // Back button (only in EventView)
                if app_state().view == "EventView" {
                    button {
                        onclick: move |_| {
                            info!("Back button clicked, setting view to Lifetime");
                            app_state.write().view = "Lifetime".to_string();
                        },
                        span { "⬅" },
                    }
                }
                            

                // Configuration selector
                select {
                    value: "{app_state().selected_yaml}",
                    onchange: move |evt| {
                        info!("Config selector changed to: {}", evt.value());
                        app_state.write().selected_yaml = evt.value();
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            match get_config_manager().load_config(&evt.value()) {
                                Ok(new_config) => {
                                    app_state.write().config = new_config;
                                },
                                Err(e) => error!("Failed to load config: {:?}", e),
                            }
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            if let Some(index) = app_state().loaded_configs.iter().position(|(name, _)| name == &evt.value()) {
                                selected_config_index.set(index);
                                app_state.write().config = app_state().loaded_configs[index].1.clone();
                            }
                        }
                    },

                    // Directly generating options with a for loop
                    for option in options.iter() {
                        option { value: "{option}", "{option}" }
                    }
                }

                // Settings button
                button {
                    class: "settings-button",
                    onclick: move |_| {
                        info!("Settings button clicked");
                        app_state.write().show_settings = true;
                    },
                    "⚙"
                }
            }

            div {
                class: "bottom-row",
                { buttons }
            }
        }

        if app_state().show_settings {
            div {
                class: "settings-modal",
                h2 { "Settings" }
            
                if app_state().view == "Lifetime" {
                    select {
                        value: "{app_state().config.life_expectancy}",
                        onchange: move |evt| {
                            if let Ok(value) = evt.value().parse() {
                                info!("Life expectancy changed to: {}", value);
                                app_state.write().config = RuntimeConfig {
                                    life_expectancy: value,
                                    ..app_state().config
                                };
                            } else {
                                error!("Failed to parse life expectancy: {}", evt.value());
                            }
                        },
                        
                        for year in 60..=120 {
                            option { value: "{year}", "{year}" }
                        }
                    }
                }
            
                button {
                    onclick: move |_| {
                        info!("Settings modal closed");
                        app_state.write().show_settings = false;
                    },
                    "Close"
                }
            }
        }
    }
}