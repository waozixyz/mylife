use crate::components::habit_tab_bar::TabBar;
use crate::components::habit_tracker::HabitTracker;
use crate::models::habit::{Habit, HabitData, WeekStart};
use crate::storage::habits::get_habit_manager;
use chrono::Local;
use dioxus::prelude::*;
use tracing::{debug, error, info};
use uuid::Uuid;

#[component]
pub fn HabitsPage() -> Element {
    let mut selected_habit_id = use_signal(|| None::<Uuid>);
    let mut current_habit_data = use_signal(|| None::<HabitData>);

    info!(
        "HabitsPage rendering with selected_habit_id: {:?}",
        selected_habit_id.read()
    );
    use_effect(move || {
        let mut current_habit_data = current_habit_data.clone();
        let mut selected_habit_id = selected_habit_id.clone();

        spawn(async move {
            if selected_habit_id.read().is_none() {
                info!("No habit selected, attempting to load first habit");
                let manager = get_habit_manager();

                match manager.get_all_habits().await {
                    Ok(habits) => {
                        if let Some((first_id, first_data)) = habits.into_iter().next() {
                            info!("Found first habit: {:?}", first_id);
                            selected_habit_id.set(Some(first_id));
                            current_habit_data.set(Some(first_data));
                        } else {
                            // Create default habit with better error handling
                            info!("No habits found, creating default habit");
                            let new_id = Uuid::new_v4();
                            let default_data = HabitData {
                                title: "Meditation".to_string(),
                                start_date: Local::now().date_naive(),
                                color: "#800080".to_string(),
                                week_start: WeekStart::Sunday,
                                completed_days: Vec::new(),
                            };

                            if let Err(e) = manager.create_habit(new_id, default_data.clone()).await
                            {
                                error!("Failed to create default habit: {:?}", e);
                                return;
                            }

                            selected_habit_id.set(Some(new_id));
                            current_habit_data.set(Some(default_data));
                        }
                    }
                    Err(e) => error!("Failed to get habits: {:?}", e),
                }
            }
        });
    });

    use_effect(move || {
        let habit_id = selected_habit_id.read();
        let mut current_habit_data = current_habit_data.clone();

        if let Some(id) = *habit_id {
            spawn(async move {
                info!("Loading data for habit: {}", id);
                let manager = get_habit_manager();

                match manager.get_habit(id).await {
                    Ok(Some(data)) => {
                        info!("Successfully loaded habit data");
                        current_habit_data.set(Some(data));
                    }
                    Ok(None) => error!("Habit not found: {}", id),
                    Err(e) => error!("Failed to load habit: {:?}", e),
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
                            let id = id;
                            let mut current_habit_data = current_habit_data.clone();
                            spawn(async move {
                                info!("Refreshing habit data for id: {}", id);
                                let manager = get_habit_manager();

                                match manager.get_habit(id).await {
                                    Ok(Some(data)) => {
                                        info!("Successfully refreshed habit data");
                                        current_habit_data.set(Some(data));
                                    }
                                    Ok(None) => error!("Habit not found during refresh: {}", id),
                                    Err(e) => error!("Failed to refresh habit: {:?}", e),
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
