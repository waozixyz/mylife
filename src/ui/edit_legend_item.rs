use dioxus::prelude::*;
use crate::models::{MyLifeApp, LifePeriod, LifePeriodEvent};
use crate::utils::date_utils::is_valid_date;
use crate::config_manager::save_config;
use chrono::{NaiveDate, Local};

fn is_valid_hex_color(color: &str) -> bool {
    color.len() == 7 && color.starts_with('#') && color[1..].chars().all(|c| c.is_ascii_hexdigit())
}


#[component]
pub fn EditLegendItem() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    let mut color_input = use_signal(String::new);
    let mut date_error = use_signal(|| String::new());
    let mut current_date = use_signal(|| String::new());

    let (min_date, max_date) = use_memo(move || {
        if let Some(item) = &app_state().item_state {
            if item.is_event {
                if let Some(period) = app_state().config.life_periods.iter().find(|p| p.id == app_state().selected_life_period.unwrap()) {
                    let period_start = NaiveDate::parse_from_str(&format!("{}-01", period.start), "%Y-%m-%d").unwrap_or_default();
                    let period_end = app_state().config.life_periods.iter()
                        .find(|p| p.start > period.start)
                        .map(|next_period| NaiveDate::parse_from_str(&format!("{}-01", next_period.start), "%Y-%m-%d").unwrap_or_default())
                        .unwrap_or_else(|| chrono::Local::now().date_naive());
                    return (Some(period_start), Some(period_end));
                }
            }
        }
        (None, None)
    })();


    use_effect(move || {
        if let Some(item) = &app_state().item_state {
            if !item.start.is_empty() {
                current_date.set(item.start.clone());
            } else {
                let default_date = if item.is_event {
                    max_date.map(|d| d.format("%Y-%m-%d").to_string())
                } else {
                    Some(Local::now().format("%Y-%m").to_string())
                };
                if let Some(date) = default_date {
                    current_date.set(date);
                }
            }
            color_input.set(item.color.clone());
        }
    });

    
    let update_config_item = move |_| {
        if date_error().is_empty() {

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
                } else if let Some(period) = new_config.life_periods.iter_mut().find(|p| p.id == item.id) {
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
                app_state.write().config = new_config;
                let _ = save_config(&app_state().config, &app_state().selected_yaml);
            }
            app_state.write().item_state = None;
            app_state.write().temp_start_date = String::new();
        }
    };

    let close_modal = move |_| {
        if date_error().is_empty() {
            app_state.write().item_state = None;
            app_state.write().temp_start_date = String::new();
        }
    };



    let update_color = move |evt: Event<FormData>| {
        let new_color = evt.value().to_string();
        color_input.set(new_color.clone());
        if is_valid_hex_color(&new_color) {
            if let Some(item) = app_state.write().item_state.as_mut() {
                item.color = new_color;
            }
        }
    };
    
    let update_date = move |evt: Event<FormData>| {
        let new_date = evt.value().to_string();
        current_date.set(new_date.clone());
        
        let is_event = app_state().item_state.as_ref().map_or(false, |item| item.is_event);
        if is_valid_date(&new_date, !is_event) {
            if is_event {
                if let (Some(min), Some(max)) = (min_date, max_date) {
                    let input_date = NaiveDate::parse_from_str(&new_date, "%Y-%m-%d").unwrap();
                    if input_date >= min && input_date < max {
                        if let Some(item) = app_state.write().item_state.as_mut() {
                            item.start = new_date.clone();
                        }
                        app_state.write().temp_start_date = new_date;
                        date_error.set(String::new());
                    } else {
                        date_error.set(format!("Date must be between {} and {}", min.format("%Y-%m-%d"), max.format("%Y-%m-%d")));
                    }
                }
            } else {
                // For lifetime view, only check if the format is correct (YYYY-MM)
                if let Some(item) = app_state.write().item_state.as_mut() {
                    item.start = new_date.clone();
                }
                app_state.write().temp_start_date = new_date;
                date_error.set(String::new());
            }
        } else {
            date_error.set("Invalid date format".to_string());
        }
    };

    let color_preview = move || {
        if is_valid_hex_color(&color_input()) {
            color_input().to_string()
        } else {
            app_state().item_state.as_ref().map_or("#000000".to_string(), |item| item.color.clone())
        }
    };


    let delete_item = move |_| {
        if let Some(item) = app_state().item_state {
            let mut new_config = app_state().config.clone();
            if item.is_event {
                if let Some(period) = new_config.life_periods.iter_mut().find(|p| p.id == app_state().selected_life_period.unwrap()) {
                    period.events.retain(|e| e.id != item.id);
                }
            } else {
                new_config.life_periods.retain(|p| p.id != item.id);
            }
            app_state.write().config = new_config;
            let _ = save_config(&app_state().config, &app_state().selected_yaml);
        }
        app_state.write().item_state = None;
        app_state.write().temp_start_date = String::new();
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
                            if let Some(item) = app_state.write().item_state.as_mut() {
                                item.name = evt.value().to_string();
                            }
                        }
                    }

                    input {
                        placeholder: "Start Date",
                        value: "{current_date}",
                        oninput: update_date,
                    }
                    {(!date_error().is_empty()).then(|| rsx!(
                        span { class: "error", "{date_error}" }
                    ))}
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
                            style: "background-color: {color_preview()};"
                        }
                    }
                    div {
                        class: "modal-buttons",
                        button {
                            onclick: update_config_item,
                            disabled: !date_error().is_empty(),
                            "Save"
                        }
                        button {
                            onclick: delete_item,
                            class: "delete-button",
                            "Delete"
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