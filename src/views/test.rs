use crate::server::habits::get_habits;
use dioxus::prelude::*;
use tracing::{debug, error, info};

#[component]
pub fn TestPage() -> Element {
    let mut habits = use_server_future(get_habits)?;

    use_hook(move || {
        if let Some(result) = (habits.value())() {
            match result {
                Ok(habit_list) => {
                    // Print all habit IDs using tracing
                    info!("Total Habits: {}", habit_list.len());
                    for (index, habit) in habit_list.iter().enumerate() {
                        info!("Habit {}: ID = {}", index + 1, habit.id);

                        // Debug log with more details if needed
                        debug!(
                            "Habit Details - Index: {}, ID: {}, Title: {}",
                            index, habit.id, habit.title
                        );
                    }
                }
                Err(err) => {
                    error!("Failed to fetch habits: {:?}", err);
                }
            }
        }
    });

    rsx! {
        div {
           "hello"
        }
    }
}
