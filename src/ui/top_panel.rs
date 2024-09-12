use crate::models::{MyLifeApp, Yaml, SizeInfo};
use crate::utils::screenshot::{save_screenshot, take_screenshot};
use crate::yaml_manager::{get_available_yamls, get_yaml_manager, import_yaml, export_yaml};
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};

#[cfg(target_arch = "wasm32")]
use crate::utils::compression::compress_and_encode;
#[cfg(target_arch = "wasm32")]
use crate::utils::screenshot::share_screenshot;

#[component]
pub fn TopPanel() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    let mut yaml_state = use_context::<Signal<Yaml>>();
    let mut show_screenshot_modal = use_signal(|| false);
    let mut screenshot_data = use_signal(String::new);
    let mut size_info = use_context::<Signal<SizeInfo>>();

    let options = get_available_yamls();

    let take_screenshot = move |_| {
        info!("Screenshot button clicked");
        let is_landscape = size_info().window_width > size_info().window_height;
        match take_screenshot(is_landscape) {
            Ok(data) => {
                screenshot_data.set(data);
                show_screenshot_modal.set(true);
                info!("Screenshot process completed");
            }
            Err(e) => error!("Failed to take screenshot: {}", e),
        }
    };

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

    let export_yaml_action = move |_| {
        if let Err(e) = export_yaml(&yaml_state(), &app_state().selected_yaml) {
            error!("Failed to save YAML: {}", e);
        }
    };

    #[cfg(target_arch = "wasm32")]
    let share_yaml = move |_: MouseEvent| {
        let yaml_content = serde_yaml::to_string(&yaml_state()).unwrap_or_default();
        info!("Original YAML content: {}", yaml_content);
        let encoded_yaml = compress_and_encode(&yaml_content);
        info!("Compressed and encoded YAML: {}", encoded_yaml);

        let current_url = web_sys::window().unwrap().location().href().unwrap();
        let base_url = web_sys::Url::new(&current_url).unwrap();
        let share_url = format!("{}?y={}", base_url.origin(), encoded_yaml);

        web_sys::window().unwrap().open_with_url_and_target(&share_url, "_blank").unwrap();
    
    };
    rsx! {
        div {
            class: "top-panel",
            
            // Back button (only in EventView)
            if app_state().view == "EventView" {
                button {
                    class: "back-button",
                    onclick: move |_| {
                        app_state.write().view = "Lifetime".to_string();
                    },
                    span { "⬅" },
                }
            }
    
            // Quit button
            button {
                class: "quit-button",
                onclick: move |_| {
                    #[cfg(not(target_arch = "wasm32"))]
                    std::process::exit(0);
                },
                "✖"
            }
    
            if app_state().view == "Lifetime" {
                div {
                    class: "action-buttons",
                    button { onclick: load_yaml, "📥 Import YAML" }
                    button { onclick: export_yaml_action, "📤 Export YAML" }
                    {
                        #[cfg(target_arch = "wasm32")]
                        rsx! {
                            button { onclick: share_yaml, "🔗 Share" }
                        }
                    }
                    button { onclick: take_screenshot, "📸 Screenshot" }
                }
    
                div {
                    class: "config-selectors",
                    
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
                        {
                            options.iter().map(|option| {
                                rsx! {
                                    option {
                                        value: "{option}",
                                        selected: *option == app_state().selected_yaml,
                                        "{option}"
                                    }
                                }
                            })
                        }
                    }
                    select {
                        value: "{yaml_state().life_expectancy}",
                        onchange: move |evt| {
                            if let Ok(value) = evt.value().parse() {
                                yaml_state.write().life_expectancy = value;
                            } else {
                                error!("Failed to parse life expectancy: {}", evt.value());
                            }
                        },
                        {
                            (40..=120).map(|year| {
                                rsx! {
                                    option {
                                        value: "{year}",
                                        selected: year == yaml_state().life_expectancy,
                                        "{year}"
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    

        // Screenshot Modal

        {if show_screenshot_modal() {
            rsx! {
                div {
                    class: "modal-overlay",
                    div {
                        class: "modal-content",
                        img {
                            src: "{screenshot_data()}",
                            alt: "Screenshot",
                            style: "max-width: 100%; height: auto; margin-bottom: 16px;"
                        }
                        div {
                            class: "modal-buttons",
                            button {
                                onclick: move |_| {
                                    save_screenshot(&screenshot_data);
                                },
                                "Download"
                            }
                            {
                                #[cfg(target_arch = "wasm32")]
                                rsx! {
                                    button {
                                        onclick: move |_| {
                                            share_screenshot(&screenshot_data);
                                        },
                                        "Share"
                                    }
                                }
                            }
                            button {
                                onclick: move |_| show_screenshot_modal.set(false),
                                class: "close-button",
                                "Close"
                            }
                        }
                    }
                }
            }
        } else {
            rsx! { div {} }
        }}

    }
}