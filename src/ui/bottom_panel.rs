use dioxus::prelude::*;
use crate::models::{LegendItem, MyLifeApp};
use chrono::{Datelike, Local};
use uuid::Uuid;
use crate::ui::{Legend, EditLegendItem};

#[component]
pub fn BottomPanel() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();

    let add_new_item = move |_| {
        let now = Local::now();
        let current_view = &app_state().view;
        let default_start = if *current_view == "Lifetime" {
            format!("{}-{:02}", now.year(), now.month())
        } else {
            format!("{}-{:02}-{:02}", now.year(), now.month(), now.day())
        };

        let new_item = if *current_view == "Lifetime" {
            LegendItem {
                id: Uuid::new_v4(),
                name: "New Period".to_string(),
                start: default_start,
                color: "#000000".to_string(),
                is_event: false,
            }
        } else {
            LegendItem {
                id: Uuid::new_v4(),
                name: "New Event".to_string(),
                start: default_start,
                color: "#000000".to_string(),
                is_event: true,
            }
        };

        app_state.write().selected_legend_item = Some(new_item);
    };

    rsx! {
        div {
            class: "bottom-panel",
            div {
                class: "legend-header",
                h3 { "Legend:" }
                button {
                    onclick: add_new_item,
                    "Add New Item"
                }
            }
            div {
                class: "legend-items",
                Legend {}
            }
            // Add this block to render the EditLegendItem component
            {app_state().item_state.is_some().then(|| rsx!{
                EditLegendItem {}
            })}
        }
    }
}