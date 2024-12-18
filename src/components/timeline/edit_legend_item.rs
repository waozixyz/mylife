use crate::managers::timeline_manager::get_timeline_manager;
use crate::models::timeline::{LifePeriod, LifePeriodEvent, MyLifeApp, Yaml};
use crate::utils::date_utils::is_valid_date;
use chrono::NaiveDate;
use dioxus::prelude::*;
use tracing::{debug, warn};
use uuid::Uuid;

fn is_valid_hex_color(color: &str) -> bool {
    color.len() == 7 && color.starts_with('#') && color[1..].chars().all(|c| c.is_ascii_hexdigit())
}

#[component]
pub fn EditLegendItem() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    let mut yaml_state = use_context::<Signal<Yaml>>();
    let mut color_input = use_signal(String::new);
    let mut date_error = use_signal(String::new);
    let mut current_date = use_signal(String::new);
    let mut pending_update =
        use_signal(|| None::<(Option<LifePeriod>, Option<(Uuid, LifePeriodEvent)>)>);

    let (min_date, max_date) = use_memo(move || {
        if let Some(item) = &app_state().item_state {
            if item.is_event {
                if let Some(period) = yaml_state()
                    .life_periods
                    .iter()
                    .find(|p| p.id == Some(app_state().selected_life_period.unwrap()))
                {
                    let period_start =
                        NaiveDate::parse_from_str(&format!("{}-01", period.start), "%Y-%m-%d")
                            .unwrap_or_default();
                    let period_end = yaml_state()
                        .life_periods
                        .iter()
                        .find(|p| p.start > period.start)
                        .map(|next_period| {
                            NaiveDate::parse_from_str(
                                &format!("{}-01", next_period.start),
                                "%Y-%m-%d",
                            )
                            .unwrap_or_default()
                        })
                        .unwrap_or_else(|| chrono::Local::now().date_naive());
                    return (Some(period_start), Some(period_end));
                }
            }
        }
        (None, None)
    })();
    use_effect(move || {
        to_owned![pending_update, yaml_state];

        spawn(async move {
            if let Some((period, event)) = pending_update() {
                // Handle period updates
                if let Some(period) = period {
                    let result = if period.id.is_some() {
                        get_timeline_manager().update_life_period(period).await
                    } else {
                        get_timeline_manager().add_life_period(period).await
                    };

                    if let Err(e) = result {
                        warn!("Failed to update/add life period: {}", e);
                    }
                }

                // Handle event updates
                if let Some((period_id, event)) = event {
                    let result = if event.id.is_some() {
                        get_timeline_manager().update_event(period_id, event).await
                    } else {
                        get_timeline_manager().add_event(period_id, event).await
                    };

                    if let Err(e) = result {
                        warn!("Failed to update/add event: {}", e);
                    }
                }

                // Update the entire timeline
                if let Err(e) = get_timeline_manager().update_timeline(&yaml_state()).await {
                    warn!("Failed to update timeline: {}", e);
                }
            }
        });
    });

    let update_yaml_item = move |_| {
        if date_error().is_empty() {
            if let Some(item) = app_state().item_state {
                let new_yaml = yaml_state();

                if item.is_event {
                    let mut new_yaml_event = new_yaml.clone();
                    if let Some(period) = new_yaml_event
                        .life_periods
                        .iter_mut()
                        .find(|p| p.id == Some(app_state().selected_life_period.unwrap()))
                    {
                        let event = if let Some(event) =
                            period.events.iter_mut().find(|e| e.id == Some(item.id))
                        {
                            event.name = item.name.clone();
                            event.color = item.color.clone();
                            event.start = item.start.clone();
                            event.clone()
                        } else {
                            let new_event = LifePeriodEvent {
                                id: Some(item.id),
                                name: item.name.clone(),
                                color: item.color.clone(),
                                start: item.start.clone(),
                            };
                            period.events.push(new_event.clone());
                            new_event
                        };
                        period.events.sort_by(|a, b| a.start.cmp(&b.start));

                        // Queue the update
                        pending_update.set(Some((
                            None,
                            Some((app_state().selected_life_period.unwrap(), event)),
                        )));
                    }
                    yaml_state.set(new_yaml_event);
                } else {
                    let mut new_yaml_period = new_yaml.clone();
                    let period = if let Some(period) = new_yaml_period
                        .life_periods
                        .iter_mut()
                        .find(|p| p.id == Some(item.id))
                    {
                        period.name = item.name.clone();
                        period.start = item.start.clone();
                        period.color = item.color.clone();
                        period.clone()
                    } else {
                        let new_period = LifePeriod {
                            id: Some(item.id),
                            name: item.name.clone(),
                            start: item.start.clone(),
                            color: item.color.clone(),
                            events: Vec::new(),
                        };
                        new_yaml_period.life_periods.push(new_period.clone());
                        new_period
                    };
                    new_yaml_period
                        .life_periods
                        .sort_by(|a, b| a.start.cmp(&b.start));

                    // Queue the update
                    pending_update.set(Some((Some(period), None)));
                    yaml_state.set(new_yaml_period);
                }
            }
            app_state.write().item_state = None;
            app_state.write().temp_start_date = String::new();
        }
    };

    let close_modal = move |_| {
        app_state.write().item_state = None;
        app_state.write().temp_start_date = String::new();
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

        let is_event = app_state()
            .item_state
            .as_ref()
            .map_or(false, |item| item.is_event);
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
                        debug!("Valid event date set");
                    } else {
                        date_error.set(format!(
                            "Date must be between {} and {}",
                            min.format("%Y-%m-%d"),
                            max.format("%Y-%m-%d")
                        ));
                        warn!("Invalid event date: {}", new_date);
                    }
                }
            } else {
                if let Some(item) = app_state.write().item_state.as_mut() {
                    item.start = new_date.clone();
                }
                app_state.write().temp_start_date = new_date;
                date_error.set(String::new());
                debug!("Valid life period date set");
            }
        } else {
            date_error.set("Invalid date format".to_string());
            warn!("Invalid date format: {}", new_date);
        }
    };

    let color_preview = move || {
        if is_valid_hex_color(&color_input()) {
            color_input().to_string()
        } else {
            app_state()
                .item_state
                .as_ref()
                .map_or("#000000".to_string(), |item| item.color.clone())
        }
    };

    let delete_item = move |_| {
        if let Some(item) = app_state().item_state {
            let mut new_yaml = yaml_state();
            if item.is_event {
                if let Some(period) = new_yaml
                    .life_periods
                    .iter_mut()
                    .find(|p| p.id == Some(app_state().selected_life_period.unwrap()))
                {
                    period.events.retain(|e| e.id != Some(item.id));

                    // Clone required data for the async operation
                    let period_id = app_state().selected_life_period.unwrap();
                    let event_id = item.id;

                    use_future(move || async move {
                        if let Err(e) = get_timeline_manager()
                            .delete_event(period_id, event_id)
                            .await
                        {
                            warn!("Failed to delete event: {}", e);
                        }
                    });
                }
            } else {
                new_yaml.life_periods.retain(|p| p.id != Some(item.id));

                // Clone required data for the async operation
                let period_id = item.id;

                use_future(move || async move {
                    if let Err(e) = get_timeline_manager().delete_life_period(period_id).await {
                        warn!("Failed to delete life period: {}", e);
                    }
                });
            }
            yaml_state.set(new_yaml);
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
                            onclick: update_yaml_item,
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
