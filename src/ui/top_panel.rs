use crate::models::{MyLifeApp, Yaml};
use crate::yaml_manager::{get_available_yamls, get_yaml_manager, save_yaml};
use dioxus::prelude::*;
use crate::yaml_manager::import_yaml;
#[cfg(target_arch = "wasm32")]
use crate::utils::compression::compress_and_encode;
use dioxus_logger::tracing::{error, info};
use resvg::usvg::{Tree, Options};
use resvg::render;
use tiny_skia::Pixmap;
use base64::{engine::general_purpose, Engine as _};
use wasm_bindgen::JsCast;

#[component]
pub fn TopPanel() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    let mut yaml_state = use_context::<Signal<Yaml>>();
    let mut show_screenshot_modal = use_signal(|| false);
    let mut screenshot_data = use_signal(String::new);

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
    
    let take_screenshot = move |_| {
        info!("Screenshot button clicked");
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        info!("Searching for SVG element");
        let svg = match document.query_selector(".lifetime-view-svg") {
            Ok(Some(element)) => {
                info!("SVG element found");
                element
            },
            Ok(None) => {
                error!("SVG element not found");
                return;
            },
            Err(e) => {
                error!("Error querying SVG element: {:?}", e);
                return;
            }
        };
        
        info!("Getting SVG content");
        let svg_content = svg.outer_html();
        info!("SVG content length: {}", svg_content.len());
        info!("SVG content preview: {}", &svg_content[..std::cmp::min(100, svg_content.len())]);
        
        info!("Creating usvg Options");
        let opt = Options::default();
        info!("Parsing SVG content");

        let svg_content_cleaned = svg_content.replace(r#" data-dioxus-id="\d+""#, "");
        info!("Cleaned SVG content length: {}", svg_content_cleaned.len());

        // Add namespace if it's missing
        let svg_with_namespace = if !svg_content_cleaned.contains("xmlns=") {
            svg_content_cleaned.replace("<svg", r#"<svg xmlns="http://www.w3.org/2000/svg""#)
        } else {
            svg_content_cleaned
        };

        let tree = match Tree::from_str(&svg_with_namespace, &opt) {            
            Ok(tree) => {
                info!("SVG parsed successfully");
                tree
            },
            Err(e) => {
                error!("Failed to parse SVG: {:?}", e);
                info!("Full SVG content: {}", svg_with_namespace);
                return;
            }
        };
        info!("Getting pixmap size");
        let size = tree.size();
        let pixmap_width = size.width().round() as u32;
        let pixmap_height = size.height().round() as u32;
        info!("Pixmap size: {}x{}", pixmap_width, pixmap_height);
        let mut pixmap = match Pixmap::new(pixmap_width, pixmap_height) {
            Some(pixmap) => {
                info!("Pixmap created successfully");
                pixmap
            },
            None => {
                error!("Failed to create Pixmap");
                return;
            }
        };
        
        info!("Rendering SVG to pixmap");
        render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
        
        info!("Encoding PNG");
        let png_data = match pixmap.encode_png() {
            Ok(data) => {
                info!("PNG encoded successfully, size: {} bytes", data.len());
                data
            },
            Err(e) => {
                error!("Failed to encode PNG: {:?}", e);
                return;
            }
        };
        
        info!("Encoding PNG to base64");
        let base64_png = general_purpose::STANDARD.encode(&png_data);
        info!("Base64 encoded PNG length: {}", base64_png.len());
        
        info!("Setting screenshot data");
        screenshot_data.set(format!("data:image/png;base64,{}", base64_png));
        info!("Setting modal visibility");
        show_screenshot_modal.set(true);
        info!("Screenshot process completed");
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

                    info!("Original YAML content: {}", yaml_content);
                    let encoded_yaml = compress_and_encode(&yaml_content);
                    info!("Compressed and encoded YAML: {}", encoded_yaml);

                    let current_url = web_sys::window().unwrap().location().href().unwrap();
                    let base_url = web_sys::Url::new(&current_url).unwrap();
                    let share_url = format!("{}?y={}", base_url.origin(), encoded_yaml);
                    let yaml_content = serde_yaml::to_string(&yaml_state()).unwrap_or_default();

                    web_sys::window().unwrap().open_with_url_and_target(&share_url, "_blank").unwrap();
                },
                "📤 Share"
            }
            button {
                onclick: take_screenshot,
                "📸 Screenshot"
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
            button {
                onclick: take_screenshot,
                "📸 Screenshot"
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

        // Screenshot Modal
        if show_screenshot_modal() {
            div {
                class: "modal-overlay",
                div {
                    class: "modal-content",
                    img {
                        src: "{screenshot_data}",
                        alt: "Screenshot",
                        style: "max-width: 100%; height: auto;"
                    }
                    div {
                        class: "modal-buttons",
                        button {
                            onclick: move |_| {
                                save_screenshot(&screenshot_data);
                            },
                            "Download"
                        }
                        button {
                            onclick: move |_| show_screenshot_modal.set(false),
                            "Close"
                        }
                    }
                }
            }
        }
    }
}


fn save_screenshot(data: &Signal<String>) {
    let document = web_sys::window().unwrap().document().unwrap();
    let a: web_sys::HtmlAnchorElement = document.create_element("a")
        .unwrap()
        .dyn_into::<web_sys::HtmlAnchorElement>()
        .unwrap();
    a.set_href(&data());
    a.set_attribute("download", "lifetime_screenshot.png").unwrap();
    
    document.body().unwrap().append_child(&a).unwrap();
    a.click();
    document.body().unwrap().remove_child(&a).unwrap();
}