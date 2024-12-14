use crate::components::habit_tracker::HabitTracker;
use crate::components::tab_bar::TabBar;  // Add this import
use crate::db::habits::{load_first_habit, save_habit};
use crate::models::habit::Habit;
use crate::state::AppState;
use chrono::Local;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn HabitsPage() -> Element {
    let state = use_context::<AppState>();
    let mut selected_habit_id = use_signal(|| None::<Uuid>);

    // Initialize default habit if none exists
    use_effect(move || {
        let conn = state.conn.clone();
        spawn(async move {
            if selected_habit_id.read().is_none() {
                if let Ok(Some(first_habit)) = load_first_habit(&conn) {
                    selected_habit_id.set(Some(first_habit.id));
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

    rsx! {
        div { class: "habits-container",
            // Add TabBar above the HabitTracker
            TabBar {
                selected_habit_id: selected_habit_id.read().unwrap_or_default(),
                on_habit_change: move |id| selected_habit_id.set(Some(id))
            }

            // Render HabitTracker for selected habit
            {match *selected_habit_id.read() {
                Some(id) => rsx! { HabitTracker { habit_id: id } },
                None => rsx! { div { "Loading..." } }
            }}
        }
    }
}