use dioxus::prelude::*;
use crate::models::{MyLifeApp, RuntimeConfig, RuntimeLifePeriodEvent};
use chrono::{NaiveDate, Datelike, Duration, Local};
use uuid::Uuid;

#[component]
pub fn EventView(selected_life_period_id: Uuid) -> Element {
    let app_state = use_context::<Signal<MyLifeApp>>();

    let life_period = use_memo(move || {
        app_state().config.life_periods
            .iter()
            .find(|p| p.id == selected_life_period_id)
            .cloned()
    });

    match life_period() {
        Some(period) => {
            let events = &period.events;
            
            if events.is_empty() {
                return rsx! {
                    div { class: "event-view-empty",
                        "No events in this life period."
                    }
                };
            }

            let start_date = events.iter()
                .filter_map(|event| NaiveDate::parse_from_str(&event.start, "%Y-%m-%d").ok())
                .min()
                .unwrap_or_else(|| {
                    NaiveDate::parse_from_str(&period.start, "%Y-%m").unwrap_or(Local::now().date_naive())
                });

            let end_date = app_state().config.life_periods.iter()
                .find(|p| p.start > period.start)
                .and_then(|next_period| NaiveDate::parse_from_str(&format!("{}-01", next_period.start), "%Y-%m-%d").ok())
                .unwrap_or_else(|| Local::now().date_naive());

            let total_days = (end_date - start_date).num_days() as usize;
            let cols = 28;

            rsx! {
                div {
                    class: "event-view",
                    style: "grid-template-columns: repeat({cols}, 1fr);",
                    {(0..total_days).map(|day| {
                        let date = start_date + Duration::days(day as i64);
                        let color = get_color_for_event(&date, events, &end_date);
                        rsx! {
                            div {
                                key: "{day}",
                                class: "event-cell",
                                style: "background-color: {color};",
                                title: "{date}"
                            }
                        }
                    })}
                }
            }
        },
        None => rsx! {
            div { class: "event-view-not-found",
                "Selected life period not found."
            }
        }
    }
}

fn get_color_for_event(date: &NaiveDate, events: &[RuntimeLifePeriodEvent], period_end: &NaiveDate) -> String {
    for (i, event) in events.iter().enumerate() {
        let event_start = NaiveDate::parse_from_str(&event.start, "%Y-%m-%d")
            .expect("Invalid start date format in event");
        let event_end = if i < events.len() - 1 {
            NaiveDate::parse_from_str(&events[i + 1].start, "%Y-%m-%d")
                .expect("Invalid start date format in next event")
        } else {
            *period_end
        };

        if date >= &event_start && date < &event_end {
            return event.color.clone();
        }
    }
    "transparent".to_string()
}