use crate::models::timeline::{LifePeriodEvent, Yaml};
use chrono::{Duration, Local, NaiveDate};
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn EventView(selected_life_period_id: Uuid) -> Element {
    let yaml_state = use_context::<Signal<Yaml>>();

    let period = yaml_state()
        .life_periods
        .iter()
        .find(|p| p.id == Some(selected_life_period_id))
        .cloned();

    match period {
        Some(period) => {
            if period.events.is_empty() {
                return rsx! {
                    div { class: "event-view-empty",
                        h2 { "No events in this life period" }
                        p { "This life period from {period.start} currently has no events." }
                        p { "You can add events to this period to track important moments or milestones." }
                    }
                };
            }

            let mut sorted_events = period.events.clone();
            sorted_events.sort_by(|a, b| a.start.cmp(&b.start));

            let start_date = sorted_events
                .iter()
                .filter_map(|event| NaiveDate::parse_from_str(&event.start, "%Y-%m-%d").ok())
                .min()
                .unwrap_or_else(|| {
                    NaiveDate::parse_from_str(&format!("{}-01", period.start), "%Y-%m-%d")
                        .unwrap_or(Local::now().date_naive())
                });

            let end_date = yaml_state()
                .life_periods
                .iter()
                .find(|p| p.start > period.start)
                .and_then(|next_period| {
                    NaiveDate::parse_from_str(&format!("{}-01", next_period.start), "%Y-%m-%d").ok()
                })
                .unwrap_or_else(|| Local::now().date_naive());

            let total_days = (end_date - start_date).num_days() as usize;
            let cols = 28;

            rsx! {
                div {
                    class: "event-view",
                    style: "grid-template-columns: repeat({cols}, 1fr);",
                    {(0..total_days).map(|day| {
                        let date = start_date + Duration::days(day as i64);
                        let color = get_color_for_event(&date, &sorted_events, &end_date);
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
        }
        None => rsx! {
            div { class: "event-view-not-found",
                "Selected life period not found."
            }
        },
    }
}

fn get_color_for_event(
    date: &NaiveDate,
    events: &[LifePeriodEvent],
    period_end: &NaiveDate,
) -> String {
    for (i, event) in events.iter().enumerate() {
        if let Ok(event_start) = NaiveDate::parse_from_str(&event.start, "%Y-%m-%d") {
            let event_end = if i < events.len() - 1 {
                if let Ok(next_start) = NaiveDate::parse_from_str(&events[i + 1].start, "%Y-%m-%d")
                {
                    next_start
                } else {
                    *period_end
                }
            } else {
                *period_end
            };

            if date >= &event_start && date < &event_end {
                return event.color.clone();
            }
        }
    }
    "transparent".to_string()
}
