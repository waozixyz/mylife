use crate::components::habit_tracker::HabitTracker;
use crate::db::habits::{load_first_habit, save_habit};
use crate::models::habit::Habit;
use crate::state::AppState;
use chrono::Local;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn HabitsPage() -> Element {
    let state = use_context::<AppState>();
    let mut habit_id = use_signal(|| None::<Uuid>);

    // Initialize default habit if none exists
    use_effect(move || {
        let conn = state.conn.clone();
        spawn(async move {
            if habit_id.read().is_none() {
                if let Ok(Some(first_habit)) = load_first_habit(&conn) {
                    habit_id.set(Some(first_habit.id));
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
                        habit_id.set(Some(new_habit.id));
                    }
                }
            }
        });
    });

    rsx! {
        div { class: "habits-container",
            {match *habit_id.read() {
                Some(id) => rsx! { HabitTracker { habit_id: id } },
                None => rsx! { div { "Loading..." } }
            }}
        }
    }
}
