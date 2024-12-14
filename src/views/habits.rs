use crate::components::habit_tracker::HabitTracker;
use crate::components::tab_bar::TabBar; // Add this import
use crate::db::habits::{load_completed_days, load_first_habit, load_habit, save_habit};
use crate::models::habit::{Habit, HabitData, WeekStart};
use crate::state::AppState;
use chrono::Local;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn HabitsPage() -> Element {
    let state = use_context::<AppState>();
    let state_for_first_effect = state.clone();
    let state_for_second_effect = state.clone();

    let mut selected_habit_id = use_signal(|| None::<Uuid>);
    let mut current_habit_data = use_signal(|| None::<HabitData>);

    println!(
        "HabitsPage rendering with selected_habit_id: {:?}",
        selected_habit_id.read()
    );

    // Initialize default habit if none exists
    use_effect(move || {
        let conn = state_for_first_effect.conn.clone();
        let mut current_habit_data = current_habit_data.clone();

        spawn(async move {
            if selected_habit_id.read().is_none() {
                if let Ok(Some(first_habit)) = load_first_habit(&conn) {
                    selected_habit_id.set(Some(first_habit.id));

                    // Load habit data
                    if let Ok(Some(habit)) = load_habit(&conn, first_habit.id) {
                        let data = HabitData {
                            title: habit.title,
                            start_date: habit.start_date,
                            week_start: WeekStart::from_string(&habit.week_start),
                            color: habit.color,
                            completed_days: load_completed_days(&conn, first_habit.id)
                                .unwrap_or_default(),
                        };
                        current_habit_data.set(Some(data));
                    }
                } else {
                    // Create new habit if none exists
                    let new_habit = Habit {
                        id: Uuid::new_v4(),
                        title: "Meditation".to_string(),
                        start_date: Local::now().date_naive(),
                        color: "#800080".to_string(),
                        week_start: "sunday".to_string(),
                    };
                    if let Ok(()) = save_habit(&conn, &new_habit) {
                        selected_habit_id.set(Some(new_habit.id));
                    }
                }
            }
        });
    });

    use_effect(move || {
        let conn = state_for_second_effect.conn.clone();
        let habit_id = selected_habit_id.read();
        let mut current_habit_data = current_habit_data.clone();

        if let Some(id) = *habit_id {
            spawn(async move {
                if let Ok(Some(habit)) = load_habit(&conn, id) {
                    let data = HabitData {
                        title: habit.title,
                        start_date: habit.start_date,
                        week_start: WeekStart::from_string(&habit.week_start),
                        color: habit.color,
                        completed_days: load_completed_days(&conn, id).unwrap_or_default(),
                    };
                    current_habit_data.set(Some(data));
                }
            });
        }
    });
    rsx! {
        div { class: "habits-container",
            TabBar {
                selected_habit_id: selected_habit_id.read().unwrap_or_default(),
                on_habit_change: move |id| {
                    selected_habit_id.set(Some(id));
                }
            }
            {match (*selected_habit_id.read(), current_habit_data.read().as_ref()) {
                (Some(id), Some(data)) => rsx! {
                    HabitTracker {
                        habit_id: id,
                        habit_data: data.clone(),
                        on_data_change: move |_| {
                            let conn = state.conn.clone();
                            let id = id;
                            let mut current_habit_data = current_habit_data.clone();
                            spawn(async move {
                                if let Ok(Some(habit)) = load_habit(&conn, id) {
                                    let data = HabitData {
                                        title: habit.title,
                                        start_date: habit.start_date,
                                        week_start: WeekStart::from_string(&habit.week_start),
                                        color: habit.color,
                                        completed_days: load_completed_days(&conn, id).unwrap_or_default(),
                                    };
                                    current_habit_data.set(Some(data));
                                }
                            });
                        }
                    }
                },
                _ => rsx! { div { "Loading..." } }
            }}
        }
    }
}
