// src/pages/timeline_page.rs

use dioxus::prelude::*;
use crate::state::life_state::initialize_state;
use crate::components::timeline::top_panel::TopPanel;
use crate::components::timeline::central_panel::CentralPanel;
use crate::components::timeline::bottom_panel::BottomPanel;

const TIMELINE_VIEW_CSS: Asset = asset!("/assets/styling/timeline_view.css");
const TIMELINE_INPUT_CSS: Asset = asset!("/assets/styling/timeline_input.css");
const TIMELINE_ITEMS_CSS: Asset = asset!("/assets/styling/timeline_items.css");
const TIMELINE_MODAL_CSS: Asset = asset!("/assets/styling/timeline_modal.css");

#[component]
pub fn TimelinePage(y: String) -> Element {
    let (yaml_state, app_state) = initialize_state(&y);
    let yaml_state = use_signal(|| yaml_state);
    let app_state = use_signal(|| app_state);

    use_context_provider(|| yaml_state);
    use_context_provider(|| app_state);

    rsx! {
        document::Link { rel: "stylesheet", href: TIMELINE_VIEW_CSS }
        document::Link { rel: "stylesheet", href: TIMELINE_INPUT_CSS }
        document::Link { rel: "stylesheet", href: TIMELINE_ITEMS_CSS }
        document::Link { rel: "stylesheet", href: TIMELINE_MODAL_CSS }

        div {
            class: "app-container",
            TopPanel {y},
            CentralPanel {},
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