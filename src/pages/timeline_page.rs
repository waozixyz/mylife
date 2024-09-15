// src/pages/timeline_page.rs

use crate::state_manager::initialize_state;
use crate::ui::{BottomPanel, CentralPanel, TopPanel};
use dioxus::prelude::*;

#[component]
pub fn TimelinePage(y: String) -> Element {
    let (yaml_state, app_state) = initialize_state(&y);
    let yaml_state = use_signal(|| yaml_state);
    let app_state = use_signal(|| app_state);

    use_context_provider(|| yaml_state);
    use_context_provider(|| app_state);

    rsx! {
        div {
            class: "app-container",
            TopPanel {y}
            CentralPanel {}
            BottomPanel {}
        }
    }
}

#[component]
pub fn TimelinePageNoParam() -> Element {
    rsx! {
        TimelinePage { y: String::new() }
    }
}
