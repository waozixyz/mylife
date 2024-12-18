use crate::managers::timeline_manager::get_timeline_manager;
use crate::models::timeline::{LifePeriodEvent, Yaml};
use chrono::{Duration, Local, NaiveDate};
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn EventView(selected_life_period_id: Uuid) -> Element {
    let mut events = use_signal(|| None::<Result<Vec<LifePeriodEvent>, String>>);
    let mut timeline = use_signal(|| None::<Result<Yaml, String>>);

    // Load data effect
    {
        let mut events = events.clone();
        let mut timeline = timeline.clone();
        use_effect(move || {
            spawn(async move {
                let events_result = get_timeline_manager()
                    .get_period_events(selected_life_period_id)
                    .await;
                events.set(Some(events_result));

                let timeline_result = get_timeline_manager().get_timeline().await;
                timeline.set(Some(timeline_result));
            });
        });
    }

    let events_ref = events.read();
    let timeline_ref = timeline.read();

    match (&*events_ref, &*timeline_ref) {
        (Some(Ok(events)), Some(Ok(yaml))) => {
            let period = yaml
                .life_periods
                .iter()
                .find(|p| p.id == Some(selected_life_period_id))
                .cloned();

            match period {
                Some(period) => {
                    if events.is_empty() {
                        return rsx! {
                            div { class: "event-view-empty",
                                h2 { "No events in this life period" }
                                p { "This life period from {period.start} currently has no events." }
                                p { "You can add events to this period to track important moments or milestones." }
                            }
                        };
                    }

                    let start_date = events
                        .iter()
                        .filter_map(|event| {
                            NaiveDate::parse_from_str(&event.start, "%Y-%m-%d").ok()
                        })
                        .min()
                        .unwrap_or_else(|| {
                            NaiveDate::parse_from_str(&period.start, "%Y-%m")
                                .unwrap_or(Local::now().date_naive())
                        });

                    let end_date = yaml
                        .life_periods
                        .iter()
                        .find(|p| p.start > period.start)
                        .and_then(|next_period| {
                            NaiveDate::parse_from_str(
                                &format!("{}-01", next_period.start),
                                "%Y-%m-%d",
                            )
                            .ok()
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
                                let color = get_color_for_event(&date, &events, &end_date);
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
        (Some(Err(ref e)), _) | (_, Some(Err(ref e))) => rsx! {
            div { class: "error-message",
                "Failed to load data: {e}"
            }
        },
        _ => rsx! {
            div { class: "loading-message",
                "Loading..."
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
