use crate::db::habits::{
    delete_completed_day, save_completed_day, update_habit_color, update_habit_start_date,
    update_habit_week_start,
};
use crate::models::habit::{HabitData, WeekStart};
use crate::state::AppState;
use chrono::{Datelike, Duration, Local, NaiveDate};
use dioxus::prelude::*;
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
    println!("=== HabitTracker Component Start ===");
    println!("Received props - habit_id: {:?}", props.habit_id);
    println!(
        "Received props - habit_data title: {}",
        props.habit_data.title
    );
    println!(
        "Received props - start_date: {}",
        props.habit_data.start_date
    );
    println!(
        "Received props - completed_days count: {}",
        props.habit_data.completed_days.len()
    );

    let state = use_context::<AppState>();
    let conn = state.conn.clone();
    let completed_days = props.habit_data.completed_days.clone();

    let toggle_day = {
        let conn = conn.clone();
        let on_data_change = props.on_data_change.clone();
        let completed_days = completed_days.clone();
        move |date: NaiveDate| {
            if date <= Local::now().date_naive() {
                if props.habit_data.completed_days.contains(&date) {
                    let _ = delete_completed_day(&conn, props.habit_id, date);
                } else {
                    let _ = save_completed_day(&conn, props.habit_id, date);
                }
                on_data_change.call(());
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: HABIT_TRACKER_CSS }
        div { class: "habit-tracker",
            div { class: "date-picker",
                label { "Start Date: " }
                input {
                    r#type: "date",
                    value: "{props.habit_data.start_date}",
                    oninput: {
                        let conn = conn.clone();
                        let on_data_change = props.on_data_change.clone();
                        move |evt: Event<FormData>| {
                            if let Ok(date) = NaiveDate::parse_from_str(&evt.data.value(), "%Y-%m-%d") {
                                let _ = update_habit_start_date(&conn, props.habit_id, date);
                                on_data_change.call(());
                            }
                        }
                    }
                }

                label { "Week Starts On: " }
                select {
                    value: props.habit_data.week_start.to_string(),
                    onchange: {
                        let conn = conn.clone();
                        let on_data_change = props.on_data_change.clone();
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
                            let _ = update_habit_week_start(&conn, props.habit_id, &week_start.to_string());
                            on_data_change.call(());
                        }
                    },
                    option { value: "sunday", "Sunday" }
                    option { value: "monday", "Monday" }
                    option { value: "tuesday", "Tuesday" }
                    option { value: "wednesday", "Wednesday" }
                    option { value: "thursday", "Thursday" }
                    option { value: "friday", "Friday" }
                    option { value: "saturday", "Saturday" }
                }

                label { "Color: " }
                input {
                    r#type: "color",
                    value: "{props.habit_data.color}",
                    class: "color-input",
                    oninput: {
                        let conn = conn.clone();
                        let on_data_change = props.on_data_change.clone();
                        move |evt: Event<FormData>| {
                            let color = evt.data.value();
                            let _ = update_habit_color(&conn, props.habit_id, &color);
                            on_data_change.call(());
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
