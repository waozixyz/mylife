use dioxus::prelude::*;
use chrono::{NaiveDate, Datelike, Local, Weekday, Duration};

const HABIT_TRACKER_CSS: Asset = asset!("/assets/styling/habit_tracker.css");

#[derive(PartialEq, Clone)]
struct CellState {
    date: NaiveDate,
    completed: bool,
}

// Add an enum for week start
#[derive(PartialEq, Clone)]
enum WeekStart {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl WeekStart {
    fn to_weekday(&self) -> Weekday {
        match self {
            WeekStart::Sunday => Weekday::Sun,
            WeekStart::Monday => Weekday::Mon,
            WeekStart::Tuesday => Weekday::Tue,
            WeekStart::Wednesday => Weekday::Wed,
            WeekStart::Thursday => Weekday::Thu,
            WeekStart::Friday => Weekday::Fri,
            WeekStart::Saturday => Weekday::Sat,
        }
    }

    fn get_days_from_start(&self, date: NaiveDate) -> i64 {
        let mut week_start = self.to_weekday();
        let mut current_weekday = date.weekday();
        
        let days = current_weekday.num_days_from_sunday() as i64 
            - week_start.num_days_from_sunday() as i64;
        
        if days < 0 {
            days + 7
        } else {
            days
        }
    }
}

fn generate_calendar_days(
    start: NaiveDate,
    current_date: NaiveDate,
    completed_days: Signal<Vec<CellState>>,
    week_start: WeekStart,
) -> Element {
    let days_to_start = week_start.get_days_from_start(start);
    let week_start_date = start - Duration::days(days_to_start);
    
    let min_end_date = start + Duration::days(66);
    // Then extend to complete the week
    let days_to_complete_week = 6 - week_start.get_days_from_start(min_end_date);
    let total_days = (min_end_date - week_start_date).num_days() + days_to_complete_week + 1;

    rsx! {
        Fragment {
            {(0..total_days).map(|index| {
                let current = week_start_date + Duration::days(index);
                let is_current_day = current == current_date;
                let is_before_start = current < start;
                let is_completed = completed_days.read().iter().any(|state| state.date == current);
                let is_future = current > current_date;
                
                let day_class = match (is_before_start, is_current_day, is_completed, is_future) {
                    (_, true, true, _) => "calendar-cell current-day completed",
                    (_, true, false, _) => "calendar-cell current-day",
                    (_, _, true, _) => "calendar-cell completed",
                    (_, _, _, true) => "calendar-cell future",
                    (true, _, _, _) => "calendar-cell past",
                    _ => "calendar-cell",
                };

                let current_owned = current;
                let mut completed_days = completed_days.clone();
                
                rsx! {
                    div {
                        key: "{current}",
                        class: "{day_class}",
                        onclick: move |_| {
                            if !is_future {
                                let mut days = completed_days.read().clone();
                                if let Some(pos) = days.iter().position(|state| state.date == current_owned) {
                                    days.remove(pos);
                                } else {
                                    days.push(CellState { date: current_owned, completed: true });
                                }
                                completed_days.set(days);
                            }
                        },
                        "{current_owned.day()}"
                    }
                }
            })}
        }
    }
}

#[component]
pub fn HabitTracker() -> Element {
    let mut start_date = use_signal(|| Local::now().date_naive());
    let current_date = Local::now().date_naive();
    let mut completed_days = use_signal(Vec::new);
    let mut week_start = use_signal(|| WeekStart::Sunday);

    rsx! {
        document::Link { rel: "stylesheet", href: HABIT_TRACKER_CSS }
        div { class: "habit-tracker",
            div { class: "date-picker",
                label { "Start Date: " }
                input {
                    r#type: "date",
                    value: "{start_date.read()}",
                    oninput: move |evt| {
                        let value = evt.data.value();
                        if let Ok(date) = NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                            start_date.set(date);
                        }
                    }
                }
                label { "Week Starts On: " }
                select {
                    onchange: move |evt| {
                        let week_start_value = match evt.data.value().as_str() {
                            "monday" => WeekStart::Monday,
                            "tuesday" => WeekStart::Tuesday,
                            "wednesday" => WeekStart::Wednesday,
                            "thursday" => WeekStart::Thursday,
                            "friday" => WeekStart::Friday,
                            "saturday" => WeekStart::Saturday,
                            _ => WeekStart::Sunday,
                        };
                        week_start.set(week_start_value);
                    },
                    option { value: "sunday", "Sunday" }
                    option { value: "monday", "Monday" }
                    option { value: "tuesday", "Tuesday" }
                    option { value: "wednesday", "Wednesday" }
                    option { value: "thursday", "Thursday" }
                    option { value: "friday", "Friday" }
                    option { value: "saturday", "Saturday" }
                }
            }
            div { class: "calendar-header",
                {["S", "M", "T", "W", "T", "F", "S"].iter().cycle()
                    .skip(week_start.read().to_weekday().num_days_from_sunday() as usize)
                    .take(7)
                    .map(|day| rsx! { div { "{day}" } })}
            }
            div { class: "calendar-grid",
                {generate_calendar_days(*start_date.read(), current_date, completed_days, (*week_start.read()).clone())}
            }
        }
    }
}