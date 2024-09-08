use dioxus::prelude::*;
use crate::models::{MyLifeApp, LegendItem, Config, LifePeriod, LifePeriodEvent};
use crate::utils::date_utils::is_valid_date;
use crate::config_manager::save_config;
use uuid::Uuid;

fn is_valid_hex_color(color: &str) -> bool {
    color.len() == 7 && color.starts_with('#') && color[1..].chars().all(|c| c.is_digit(16))
}


#[component]
pub fn EditLegendItem() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    let mut color_input = use_signal(|| String::new());

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
                        period.events.push(LifePeriodEvent {
                            id: item.id,
                            name: item.name.clone(),
                            color: item.color.clone(),
                            start: item.start.clone(),
                        });
                    }
                }
            } else {
                if let Some(period) = new_config.life_periods.iter_mut().find(|p| p.id == item.id) {
                    period.name = item.name.clone();
                    period.start = item.start.clone();
                    period.color = item.color.clone();
                } else {
                    new_config.life_periods.push(LifePeriod {
                        id: item.id,
                        name: item.name.clone(),
                        start: item.start.clone(),
                        color: item.color.clone(),
                        events: Vec::new(),
                    });
                }
            }
            app_state.write().config = new_config;
            let _ = save_config(&app_state().config, &app_state().selected_yaml);
        }
        app_state.write().item_state = None;
        app_state.write().temp_start_date = String::new();
    };

    let close_modal = move |_| {
        app_state.write().item_state = None;
        app_state.write().temp_start_date = String::new();
    };


    let update_color = move |evt: Event<FormData>| {
        let new_color = evt.value().to_string();
        color_input.set(new_color.clone());
        if is_valid_hex_color(&new_color) {
            if let Some(mut item) = app_state.write().item_state.as_mut() {
                item.color = new_color;
            }
        }
    };

    let color_preview = move || {
        if is_valid_hex_color(&color_input()) {
            color_input().to_string()
        } else {
            app_state().item_state.as_ref().map_or("#000000".to_string(), |item| item.color.clone())
        }
    };
    rsx! {
        {app_state().item_state.is_some().then(|| rsx!{
            div {
                class: "modal-overlay",
                onclick: close_modal,
                div {
                    class: "modal-content edit-legend-item",
                    onclick: move |evt| evt.stop_propagation(),
                    h2 { "Edit Legend Item" }
                    input {
                        placeholder: "Name",
                        value: "{app_state().item_state.as_ref().unwrap().name}",
                        oninput: move |evt| {
                            if let Some(mut item) = app_state.write().item_state.as_mut() {
                                item.name = evt.value().to_string();
                            }
                        }
                    }
                    input {
                        placeholder: "Start Date",
                        value: "{app_state().temp_start_date}",
                        oninput: move |evt| {
                            app_state.write().temp_start_date = evt.value().to_string();
                            if is_valid_date(&evt.value(), !app_state().item_state.as_ref().unwrap().is_event) {
                                if let Some(mut item) = app_state.write().item_state.as_mut() {
                                    item.start = evt.value().to_string();
                                }
                            }
                        }
                    }
                    div {
                        class: "color-picker",
                        label { "Color: " }
                        input {
                            placeholder: "#RRGGBB",
                            value: "{color_input}",
                            oninput: update_color,
                        }
                        div {
                            class: "color-preview",
                            style: "background-color: {color_preview()}; width: 32px; height: 32px; margin-left: 10px; border: 1px solid #000;"
                        }
                    }
                    div {
                        class: "modal-buttons",
                        button {
                            onclick: update_config_item,
                            "Save"
                        }
                        button {
                            onclick: close_modal,
                            "Cancel"
                        }
                    }
                }
            }
        })}
    }
}

