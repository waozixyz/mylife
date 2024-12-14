use crate::db::habits::{
    delete_completed_day, load_completed_days, load_habit, save_completed_day, update_habit_color,
    update_habit_start_date, update_habit_week_start,
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
}

#[component]
pub fn HabitTracker(props: HabitTrackerProps) -> Element {
    let state = use_context::<AppState>();
    let conn = state.conn.clone();

    let mut habit_data = use_signal(|| HabitData {
        title: String::from("Meditation"),
        start_date: Local::now().date_naive(),
        completed_days: Vec::new(),
        week_start: WeekStart::Sunday,
        color: String::from("#800080"),
    });

    let toggle_day = {
        let conn = conn.clone();
        move |date: NaiveDate| {
            if date <= Local::now().date_naive() {
                let mut data = habit_data.read().clone();
                if let Some(pos) = data.completed_days.iter().position(|&d| d == date) {
                    data.completed_days.remove(pos);
                    let _ = delete_completed_day(&conn, props.habit_id, date);
                } else {
                    data.completed_days.push(date);
                    let _ = save_completed_day(&conn, props.habit_id, date);
                }
                habit_data.set(data);
            }
        }
    };

    // Load initial data effect
    {
        let conn = conn.clone();
        let habit_id = props.habit_id;

        use_effect(move || {
            let conn = conn.clone();
            spawn(async move {
                println!("Loading habit data for ID: {}", habit_id);
                if let Ok(Some(habit)) = load_habit(&conn, habit_id) {
                    let mut data = habit_data.read().clone();
                    data.title = habit.title;
                    data.start_date = habit.start_date;
                    data.week_start = WeekStart::from_string(&habit.week_start);
                    if let Ok(days) = load_completed_days(&conn, habit_id) {
                        data.completed_days = days;
                    }
                    habit_data.set(data);
                }
            });
        });
    }

    rsx! {
        document::Link { rel: "stylesheet", href: HABIT_TRACKER_CSS }
        div { class: "habit-tracker",

            // Settings section
            div { class: "date-picker",
                label { "Start Date: " }
                input {
                    r#type: "date",
                    value: "{habit_data.read().start_date}",
                    oninput: {
                        let conn = conn.clone();  // Clone conn here
                        move |evt: Event<FormData>| {
                            if let Ok(date) = NaiveDate::parse_from_str(&evt.data.value(), "%Y-%m-%d") {
                                let mut data = habit_data.read().clone();
                                data.start_date = date;
                                let _ = update_habit_start_date(&conn, props.habit_id, date);
                                habit_data.set(data);
                            }
                        }
                    }
                }

                label { "Week Starts On: " }
                select {
                    onchange: {
                        let conn = conn.clone();
                        move |evt: Event<FormData>| {
                            let mut data = habit_data.read().clone();
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
                            data.week_start = week_start;
                            habit_data.set(data);
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
                    value: "{habit_data.read().color}",
                    class: "color-input",
                    oninput: {
                        let conn = conn.clone();
                        move |evt: Event<FormData>| {
                            let mut data = habit_data.read().clone();
                            let color = evt.data.value();  // Get the color value
                            data.color = color.clone();    // Clone it for data
                            let _ = update_habit_color(&conn, props.habit_id, &color);  // Use original color here
                            habit_data.set(data);
                        }
                    }
                }
            }
            br {}
            // Calendar header with dynamic week start
            div { class: "calendar-header",
                {["S", "M", "T", "W", "T", "F", "S"].iter().cycle()
                    .skip(habit_data.read().week_start.to_weekday().num_days_from_sunday() as usize)
                    .take(7)
                    .map(|day| rsx! { div { "{day}" } })}
            }

            // Calendar grid
            div { class: "calendar-grid",
                {render_calendar(
                    habit_data.read().start_date,
                    Local::now().date_naive(),
                    &habit_data.read().completed_days,
                    habit_data.read().color.clone(),
                    habit_data.read().week_start.clone(),
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
