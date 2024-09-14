use crate::models::{MyLifeApp, SizeInfo, Yaml};
use crate::routes::Route;
use crate::utils::screenshot::{save_screenshot, take_screenshot};
use crate::yaml_manager::{export_yaml, get_available_yamls, get_yaml_manager, import_yaml};
use arboard::Clipboard;
use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use qrcode::render::svg;
use qrcode::QrCode;

#[cfg(all(target_os = "linux", not(target_arch = "wasm32")))]
use wl_clipboard_rs::copy::{MimeType, Options as WlOptions, Source};

use crate::utils::compression::compress_and_encode;
#[cfg(target_arch = "wasm32")]
use crate::utils::screenshot::share_screenshot;

#[component]
pub fn TopPanel() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    let mut yaml_state = use_context::<Signal<Yaml>>();
    let mut show_screenshot_modal = use_signal(|| false);
    let mut screenshot_data = use_signal(String::new);
    let size_info = use_context::<Signal<SizeInfo>>();
    let mut show_share_modal = use_signal(|| false);
    let mut share_url = use_signal(String::new);

    let options = get_available_yamls();

    let take_screenshot = move |_| {
        let is_landscape = size_info().window_width > size_info().window_height;
        match take_screenshot(is_landscape) {
            Ok(data) => {
                screenshot_data.set(data);
                show_screenshot_modal.set(true);
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

    let share_yaml = move |_: MouseEvent| {
        let yaml_content = serde_yaml::to_string(&yaml_state()).unwrap_or_default();
        let encoded_yaml = compress_and_encode(&yaml_content);

        let base_url = "https://mylife.waozi.xyz";
        let url = format!("{}?y={}", base_url, encoded_yaml);

        share_url.set(url);
        show_share_modal.set(true);
    };

    let copy_to_clipboard = move |_: MouseEvent| {
        let url = share_url();

        #[cfg(target_arch = "wasm32")]
        {
            let _ = web_sys::window()
                .unwrap()
                .navigator()
                .clipboard()
                .unwrap()
                .write_text(&url);
        }

        #[cfg(all(target_os = "linux", not(target_arch = "wasm32")))]
        {
            let opts = WlOptions::new();
            if let Err(e) = opts.copy(
                Source::Bytes(url.clone().into_bytes().into()),
                MimeType::Text,
            ) {
                error!(
                    "Failed to copy URL to clipboard using wl-clipboard-rs: {}",
                    e
                );
                // Fallback to arboard
                if let Ok(mut clipboard) = Clipboard::new() {
                    if let Err(e) = clipboard.set_text(&url) {
                        error!("Failed to copy URL to clipboard using arboard: {}", e);
                    }
                }
            }
        }

        #[cfg(all(not(target_os = "linux"), not(target_arch = "wasm32")))]
        {
            if let Ok(mut clipboard) = Clipboard::new() {
                if let Err(e) = clipboard.set_text(&url) {
                    error!("Failed to copy URL to clipboard: {}", e);
                }
            }
        }
    };

    let generate_qr_code = move |url: &str| -> String {
        let code = QrCode::new(url).unwrap();
        code.render::<svg::Color<'_>>()
            .min_dimensions(200, 200)
            .max_dimensions(250, 250)
            .build()
    };
    rsx! {
        div {
            class: "top-panel",
            if app_state().view == "EventView" {

                button {
                    onclick: move |_| {
                        app_state.write().view = "Lifetime".to_string();
                    },
                    span { "⬅" },
                }
            } else {
                Link {
                    class: "button",
                    to: Route::HomePage { y: String::new() },
                    span { "⬅" },
                }
            }

            if app_state().view == "Lifetime" {
                div {
                    class: "action-buttons",
                    button { onclick: load_yaml, "📥 Import" }
                    button { onclick: export_yaml_action, "📤 Export" }
                    button { onclick: share_yaml, "🔗 Share" }
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

        // Share Modal
        {if show_share_modal() {
            rsx! {
                div {
                    class: "modal-overlay",
                    div {
                        class: "modal-content",
                        h2 { "Share Your YAML" }
                        div {
                            class: "qr-code-container",
                            dangerous_inner_html: "{generate_qr_code(&share_url())}"
                        }
                        div {
                            class: "url-container",
                            input {
                                readonly: true,
                                value: "{share_url()}",
                            }
                            button {
                                onclick: copy_to_clipboard,
                                class: "copy-button",
                                "Copy"
                            }
                        }
                        button {
                            onclick: move |_| show_share_modal.set(false),
                            class: "close-button",
                            "Close"
                        }
                    }
                }
            }
        } else {
            rsx! { div {} }
        }}
    }
}
