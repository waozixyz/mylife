// src/pages/timeline_page.rs

use dioxus::prelude::*;
use crate::state::life_state::initialize_state;

#[component]
pub fn LifePage(y: String) -> Element {
    let (yaml_state, app_state) = initialize_state(&y);
    let yaml_state = use_signal(|| yaml_state);
    let app_state = use_signal(|| app_state);

    use_context_provider(|| yaml_state);
    use_context_provider(|| app_state);

    rsx! {
        div {
            class: "app-container"
        }
    }
}

#[component]
pub fn TimelinePageNoParam() -> Element {
    rsx! {
        TimelinePage { y: String::new() }
    }
}