use dioxus::prelude::*;
use crate::models::{MyLifeApp, LegendItem, RuntimeConfig, RuntimeLifePeriod, RuntimeLifePeriodEvent};
use crate::utils::date_utils::is_valid_date;
use crate::config_manager::save_config;
use uuid::Uuid;

#[component]
pub fn EditLegendItem() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();

    let update_config_item = move |_| {
        if let Some(item) = app_state().item_state {
            let mut new_config = app_state().config.clone();
            if item.is_event {
                if let Some(period) = new_config.life_periods.iter_mut().find(|p| p.id == app_state().selected_life_period.unwrap()) {
                    if let Some(event) = period.events.iter_mut().find(|e| e.id == item.id) {
                        event.name = item.name.clone();
                        event.color = item.color.clone();
                        event.start = item.start.clone();
                    } else {
                        period.events.push(RuntimeLifePeriodEvent {
                            id: item.id,
                            name: item.name.clone(),
                            color: item.color.clone(),
                            start: item.start.clone(),
                        });
                    }
                }
            } else if let Some(period) = new_config.life_periods.iter_mut().find(|p| p.id == item.id) {
                period.name = item.name.clone();
                period.start = item.start.clone();
                period.color = item.color.clone();
            } else {
                new_config.life_periods.push(RuntimeLifePeriod {
                    id: item.id,
                    name: item.name.clone(),
                    start: item.start.clone(),
                    color: item.color.clone(),
                    events: Vec::new(),
                });
            }
            app_state.write().config = new_config;
            let _ = save_config(&app_state().config, &app_state().selected_yaml);
        }
        app_state.write().item_state = None;
        app_state.write().temp_start_date = String::new();
    };

    rsx! {
        {app_state().item_state.map(|item| {
            rsx! {
                div {
                    class: "edit-legend-item",
                    h2 { "Edit Legend Item" }
                    input {
                        placeholder: "Name",
                        value: "{item.name}",
                        oninput: move |evt| {
                            app_state().item_state.unwrap().name = evt.value().to_string();
                        }
                    }
                    input {
                        placeholder: "Start Date",
                        value: "{app_state().temp_start_date}",
                        oninput: move |evt| {
                            app_state.write().temp_start_date = evt.value().to_string();
                            if is_valid_date(&evt.value(), !item.is_event) {
                                app_state().item_state.unwrap().start = evt.value().to_string();
                            }
                        }
                    }
                    // Color picker would go here
                    // You might need to implement a custom color picker component for Dioxus
                    button {
                        onclick: update_config_item,
                        "Close"
                    }
                }
            }
        })}
    }
}