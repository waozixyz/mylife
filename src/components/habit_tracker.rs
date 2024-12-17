use crate::managers::habit_manager::get_habit_manager;
use crate::models::habit::{HabitData, WeekStart};
use chrono::{Datelike, Duration, Local, NaiveDate};
use dioxus::prelude::*;
use tracing::info;
use uuid::Uuid;

const HABIT_TRACKER_CSS: Asset = asset!("/assets/styling/habit_tracker.css");

#[derive(Props, Clone, PartialEq)]
pub struct HabitTrackerProps {
    habit_id: Uuid,
    habit_data: HabitData,
    on_data_change: EventHandler<()>,
}

#[component]
pub fn HabitTracker(props: HabitTrackerProps) -> Element {
    let completed_days = props.habit_data.completed_days.clone();

    let toggle_day = {
        let on_data_change = props.on_data_change.clone();
        let completed_days = completed_days.clone();
        move |date: NaiveDate| {
            if date <= Local::now().date_naive() {
                spawn({
                    let on_data_change = on_data_change.clone();
                    let habit_id = props.habit_id;
                    let is_completed = completed_days.contains(&date);
                    async move {
                        let manager = get_habit_manager();
                        let result = if is_completed {
                            manager.unmark_day(habit_id, date).await
                        } else {
                            manager.mark_day(habit_id, date).await
                        };

                        match result {
                            Ok(_) => on_data_change.call(()),
                            Err(e) => println!("Failed to toggle day: {:?}", e),
                        }
                    }
                });
            }
        }
    };

    let color_clone = props.habit_data.color.clone();
    use_effect(move || {
        info!("HabitTracker color changed to: {}", color_clone);
    });

    rsx! {
        document::Link { rel: "stylesheet", href: HABIT_TRACKER_CSS }
        div { class: "habit-tracker",
            div { class: "date-picker",
                label { "Start Date: " }
                input {
                    r#type: "date",
                    value: "{props.habit_data.start_date}",
                    oninput: {
                        let on_data_change = props.on_data_change.clone();
                        let habit_id = props.habit_id;
                        move |evt: Event<FormData>| {
                            if let Ok(date) = NaiveDate::parse_from_str(&evt.data.value(), "%Y-%m-%d") {
                                spawn({
                                    let on_data_change = on_data_change.clone();
                                    let habit_id = habit_id;
                                    async move {
                                        let manager = get_habit_manager();
                                        let mut current_data = match manager.get_habit(habit_id).await {
                                            Ok(Some(data)) => data,
                                            _ => return,
                                        };
                                        current_data.start_date = date;
                                        match manager.update_habit(habit_id, current_data).await {
                                            Ok(_) => on_data_change.call(()),
                                            Err(e) => println!("Failed to update start date: {:?}", e),
                                        }
                                    }
                                });
                            }
                        }
                    }
                }

                label { "Week Starts On: " }
                select {
                    value: props.habit_data.week_start.to_string(),
                    onchange: {
                        let on_data_change = props.on_data_change.clone();
                        let habit_id = props.habit_id;
                        move |evt: Event<FormData>| {
                            let week_start = match evt.data.value().as_str() {
                                "monday" => WeekStart::Monday,
                                "tuesday" => WeekStart::Tuesday,
                                "wednesday" => WeekStart::Wednesday,
                                "thursday" => WeekStart::Thursday,
                                "friday" => WeekStart::Friday,
                                "saturday" => WeekStart::Saturday,
                                _ => WeekStart::Sunday,
                            };
                            spawn({
                                let on_data_change = on_data_change.clone();
                                let habit_id = habit_id;
                                async move {
                                    let manager = get_habit_manager();
                                    match manager.get_habit(habit_id).await {
                                        Ok(Some(mut current_data)) => {
                                            current_data.week_start = week_start;
                                            match manager.update_habit(habit_id, current_data).await {
                                                Ok(_) => on_data_change.call(()),
                                                Err(e) => println!("Failed to update week start: {:?}", e),
                                            }
                                        }
                                        _ => println!("Failed to get habit"),
                                    }
                                }
                            });
                        }
                    },
                    option { value: "sunday", "Sunday" },
                    option { value: "monday", "Monday" },
                    option { value: "tuesday", "Tuesday" },
                    option { value: "wednesday", "Wednesday" },
                    option { value: "thursday", "Thursday" },
                    option { value: "friday", "Friday" },
                    option { value: "saturday", "Saturday" }
                }

                label { "Color: " }
                input {
                    r#type: "color",
                    value: "{props.habit_data.color}",
                    class: "color-input",
                    oninput: {
                        let on_data_change = props.on_data_change.clone();
                        let habit_id = props.habit_id;
                        move |evt: Event<FormData>| {
                            let color = evt.data.value();
                            spawn({
                                let color = color.clone();
                                let habit_id = habit_id;
                                let on_data_change = on_data_change.clone();
                                async move {
                                    let manager = get_habit_manager();
                                    match manager.get_habit(habit_id).await {
                                        Ok(Some(mut current_data)) => {
                                            current_data.color = color;
                                            match manager.update_habit(habit_id, current_data).await {
                                                Ok(_) => on_data_change.call(()),
                                                Err(e) => println!("Failed to update color: {:?}", e),
                                            }
                                        }
                                        _ => println!("Failed to get habit"),
                                    }
                                }
                            });
                        }
                    }
                }
            }
            br {}

            div { class: "calendar-header",
                {["S", "M", "T", "W", "T", "F", "S"].iter().cycle()
                    .skip(props.habit_data.week_start.to_weekday().num_days_from_sunday() as usize)
                    .take(7)
                    .map(|day| rsx! { div { "{day}" } })}
            }

            div { class: "calendar-grid",
                {render_calendar(
                    props.habit_data.start_date,
                    Local::now().date_naive(),
                    &completed_days,
                    props.habit_data.color.clone(),
                    props.habit_data.week_start.clone(),
                    toggle_day
                )}
            }
        }
    }
}

fn render_calendar<F>(
    start_date: NaiveDate,
    current_date: NaiveDate,
    completed_days: &[NaiveDate],
    color: String,
    week_start: WeekStart,
    on_click: F,
) -> Element
where
    F: FnMut(NaiveDate) + Clone + 'static,
{
    let days_to_start = week_start.get_days_from_start(start_date);
    let week_start_date = start_date - Duration::days(days_to_start);

    let min_end_date = start_date + Duration::days(66);
    let days_to_complete_week = 6 - week_start.get_days_from_start(min_end_date);
    let total_days = (min_end_date - week_start_date).num_days() + days_to_complete_week + 1;

    let days = (0..total_days).map(|i| week_start_date + Duration::days(i));

    rsx! {
        Fragment {
            {days.map(|date| {
                let is_completed = completed_days.contains(&date);
                let is_future = date > current_date;
                let is_before_start = date < start_date;
                let mut on_click = on_click.clone();

                rsx! {
                    div {
                        key: "{date}",
                        class: format!(
                            "calendar-cell {} {} {} {}",
                            if is_completed { "completed" } else { "" },
                            if is_future { "future" } else { "" },
                            if date == current_date { "current-day" } else { "" },
                            if is_before_start { "past" } else { "" }
                        ),
                        style: "--selected-color: {color}; --selected-color-light: {color}44;",
                        onclick: move |_| on_click(date),
                        "{date.day()}"
                    }
                }
            })}
        }
    }
}
