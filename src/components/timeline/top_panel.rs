use crate::managers::timeline_manager::get_timeline_manager;
use crate::models::timeline::{MyLifeApp, SizeInfo, Yaml};
use crate::utils::screenshot::{save_screenshot, take_screenshot};
#[cfg(not(target_arch = "wasm32"))]
use arboard::Clipboard;
use dioxus::prelude::*;
use qrcode::render::svg;
use qrcode::QrCode;
use tracing::error;

#[cfg(all(target_os = "linux", not(target_arch = "wasm32")))]
use wl_clipboard_rs::copy::{MimeType, Options as WlOptions, Source};

use crate::utils::compression::compress_and_encode;
#[cfg(target_arch = "wasm32")]
use crate::utils::screenshot::share_screenshot;

#[component]
fn YamlSelector(
    app_state: Signal<MyLifeApp>,
    yaml_state: Signal<Yaml>,
    available_timelines: Signal<Vec<String>>,
) -> Element {
    rsx! {
        select {
            value: "{app_state().selected_yaml}",
            onchange: {
                move |evt: Event<FormData>| {
                    let selected_yaml = evt.value().to_string();
                    use_future(move || {
                        let selected_yaml = selected_yaml.clone();
                        async move {
                            app_state.write().selected_yaml = selected_yaml.clone();
                            if let Ok(new_yaml) = get_timeline_manager().get_timeline_by_name(&selected_yaml).await {
                                yaml_state.set(new_yaml.clone());
                                // Also update the timeline in the manager
                                if let Err(e) = get_timeline_manager().update_timeline(&new_yaml.clone()).await {
                                    error!("Failed to update timeline: {}", e);
                                }
                            }
                        }
                    });
                }
            },
            if available_timelines().is_empty() {
                option {
                    value: "default",
                    "default"
                }
            } else {
                { available_timelines.read().iter().map(|name| {
                    rsx! {
                        option {
                            value: "{name}",
                            selected: name == &app_state().selected_yaml,
                            "{name}"
                        }
                    }
                })}
            }
        }
    }
}

#[component]
pub fn TopPanel(y: String) -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    let mut yaml_state = use_context::<Signal<Yaml>>();
    let mut show_screenshot_modal = use_signal(|| false);
    let mut screenshot_data = use_signal(String::new);
    let size_info = use_context::<Signal<SizeInfo>>();
    let mut show_share_modal = use_signal(|| false);
    let mut share_url = use_signal(String::new);
    let available_timelines = use_signal(Vec::new);

    // Load timeline functionality
    let load_timeline = move |_| {
        use_future(move || async move {
            if let Some((name, new_yaml)) = get_timeline_manager().import_timeline().await {
                yaml_state.set(new_yaml.clone());
                app_state.write().selected_yaml = name;
            } else {
                error!("Failed to import timeline");
            }
        });
    };

    // Export timeline functionality
    let export_timeline = move |_| {
        use_future(move || async move {
            if let Err(e) = get_timeline_manager().export_timeline(&yaml_state()).await {
                error!("Failed to export timeline: {}", e);
            }
        });
    };

    // Screenshot functionality remains unchanged
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

    // Share functionality
    let share_timeline = move |_: MouseEvent| {
        let yaml_content = serde_yaml::to_string(&yaml_state()).unwrap_or_default();
        let encoded_yaml = compress_and_encode(&yaml_content);

        let base_url = "https://myquest.waozi.xyz";
        let url = format!("{}?y={}", base_url, encoded_yaml);

        share_url.set(url);
        show_share_modal.set(true);
    };

    let copy_to_clipboard = move |_: MouseEvent| {
        let url = share_url();
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen_futures::JsFuture;

            let window = web_sys::window().expect("No global `window` exists");
            let navigator = window.navigator();

            let clipboard = navigator.clipboard();
            let url_clone = url.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let promise = clipboard.write_text(&url_clone);
                match JsFuture::from(promise).await {
                    Ok(_) => {
                        log::info!("URL copied to clipboard successfully");
                    }
                    Err(e) => {
                        error!("Failed to copy URL to clipboard: {:?}", e);
                    }
                }
            });
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

    use_effect(move || {
        to_owned![available_timelines];
        spawn(async move {
            let timelines = get_timeline_manager().get_available_timelines().await;
            available_timelines.set(timelines);
        });
        (|| ())()
    });

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
            }
            if app_state().view == "Lifetime" {
                div {
                    class: "action-buttons",
                    button { onclick: load_timeline, "📥 Import" }
                    button { onclick: export_timeline, "📤 Export" }
                    button { onclick: share_timeline, "🔗 Share" }
                    button { onclick: take_screenshot, "📸 Screenshot" }
                }


                div {
                    class: "config-selectors",
                    YamlSelector {
                        app_state: app_state,
                        yaml_state: yaml_state,
                        available_timelines: available_timelines
                    },
                    select {
                        value: "{yaml_state().life_expectancy}",
                        onchange: move |evt| {
                            if let Ok(value) = evt.value().parse() {
                                yaml_state.write().life_expectancy = value;
                                // Update timeline after changing life expectancy
                                use_future(move || async move {
                                    if let Err(e) = get_timeline_manager().update_timeline(&yaml_state()).await {
                                        error!("Failed to update timeline: {}", e);
                                    }
                                });
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
