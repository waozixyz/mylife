use crate::components::timeline::bottom_panel::BottomPanel;
use crate::components::timeline::central_panel::CentralPanel;
use crate::components::timeline::top_panel::TopPanel;
use crate::managers::timeline_manager::get_timeline_manager;
use crate::state::life_state::initialize_state;
use dioxus::prelude::*;
use tokio::time::Duration;
use tracing::{debug, error};
const TIMELINE_VIEW_CSS: Asset = asset!("/assets/styling/timeline_view.css");
const TIMELINE_ITEMS_CSS: Asset = asset!("/assets/styling/timeline_items.css");
const TIMELINE_MODAL_CSS: Asset = asset!("/assets/styling/timeline_modal.css");

#[component]
pub fn TimelinePage(y: String) -> Element {
    let loading = use_signal(|| true);
    let yaml_state = use_signal(Default::default);
    let app_state = use_signal(Default::default);
    let y_two = y.clone();
    // Initialize state using use_future

    // Initialize state using use_future
    use_future(move || {
        to_owned![y, yaml_state, app_state, loading];
        async move {
            debug!("Initializing timeline state");
            let (yaml, app) = initialize_state(&y).await;
            yaml_state.set(yaml);
            app_state.set(app);
            loading.set(false);
            debug!("Timeline state initialized");
        }
    });

    use_effect(move || {
        to_owned![yaml_state];
        spawn(async move {
            let timeline_manager = get_timeline_manager();
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                match timeline_manager.check_for_file_changes().await {
                    Ok(Some(new_yaml)) => {
                        debug!("File changes detected, updating state");
                        yaml_state.set(new_yaml);
                    }
                    Ok(None) => (),
                    Err(e) => error!("Error checking for file changes: {}", e),
                }
            }
        });
        (|| ())()
    });
    // Show loading state while initializing
    if loading() {
        return rsx! {
            div {
                class: "loading-container",
                div { class: "loading-spinner" }
                div { "Loading timeline..." }
            }
        };
    }

    // Set up context providers
    use_context_provider(|| yaml_state);
    use_context_provider(|| app_state);

    rsx! {
        // Stylesheet links
        document::Link {
            rel: "stylesheet",
            href: TIMELINE_VIEW_CSS
        }
        document::Link {
            rel: "stylesheet",
            href: TIMELINE_ITEMS_CSS
        }
        document::Link {
            rel: "stylesheet",
            href: TIMELINE_MODAL_CSS
        }

        // Main app container
        div {
            class: "app-container",
            TopPanel { y: y_two.clone() }
            CentralPanel {}
            BottomPanel {}
        }
    }
}

#[component]
pub fn TimelinePageNoParam() -> Element {
    rsx! {
        TimelinePage {
            y: String::new()
        }
    }
}
