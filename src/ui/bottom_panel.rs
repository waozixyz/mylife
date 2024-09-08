use dioxus::prelude::*;
use crate::models::{LegendItem, MyLifeApp};
use chrono::{Datelike, Local};
use uuid::Uuid;
use crate::ui::{Legend, EditLegendItem};

#[component]
pub fn BottomPanel() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    let add_new_item = move |_| {
        let current_view = &app_state().view;
        let default_start = if *current_view == "Lifetime" {
            let now = Local::now();
            format!("{}-{:02}", now.year(), now.month())
        } else {
            // For EventView, use the start date of the selected life period
            if let Some(period_id) = app_state().selected_life_period {
                if let Some(period) = app_state().yaml.life_periods.iter().find(|p| p.id == period_id) {
                    format!("{}-01", period.start) // Append -01 to the YYYY-MM format
                } else {
                    // Fallback to current date if period not found
                    let now = Local::now();
                    format!("{}-{:02}-{:02}", now.year(), now.month(), now.day())
                }
            } else {
                // Fallback to current date if no period is selected
                let now = Local::now();
                format!("{}-{:02}-{:02}", now.year(), now.month(), now.day())
            }
        };
    
        let new_item = if *current_view == "Lifetime" {
            LegendItem {
                id: Uuid::new_v4(),
                name: "New Period".to_string(),
                start: default_start.clone(),
                color: "#6495ED".to_string(),
                is_event: false,
            }
        } else {
            LegendItem {
                id: Uuid::new_v4(),
                name: "New Event".to_string(),
                start: default_start.clone(),
                color: "#6495ED".to_string(),
                is_event: true,
            }
        };
    
        app_state.write().item_state = Some(new_item);
        app_state.write().temp_start_date = default_start;
    };

    rsx! {
        div {
            class: "bottom-panel",
            div {
                class: "legend-header",
                button {
                    class: "legend-item add-new-item",
                    onclick: add_new_item,
                    span {
                        class: "legend-color",
                        style: "background-color: #000000;",
                        "+"
                    }
                    span {
                        class: "legend-name",
                        "Add New Item"
                    }
                }
            
                div {
                    class: "legend-items",
                    Legend {}
                }
            }
            EditLegendItem {}
        }
    }
}