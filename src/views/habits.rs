use crate::components::habit_tracker::HabitTracker;
use crate::components::tab_bar::TabBar;
use crate::models::habit::{Habit, HabitData, WeekStart};
use crate::server::habits::{
    create_habit, get_completed_days, get_first_habit, get_habit, update_habit, update_habit_color,
};
use chrono::Local;
use dioxus::prelude::*;
use uuid::Uuid;
use tracing::{debug, error, info};

#[component]
pub fn HabitsPage() -> Element {
    let mut selected_habit_id = use_signal(|| None::<Uuid>);
    let mut current_habit_data = use_signal(|| None::<HabitData>);

    info!(
        "HabitsPage rendering with selected_habit_id: {:?}",
        selected_habit_id.read()
    );

    // Initialize default habit if none exists
    use_effect(move || {
        let mut current_habit_data = current_habit_data.clone();
        let mut selected_habit_id = selected_habit_id.clone();

        spawn(async move {
            if selected_habit_id.read().is_none() {
                info!("No habit selected, attempting to load first habit");
                match get_first_habit().await {
                    Ok(Some(first_habit)) => {
                        info!("Found first habit: {:?}", first_habit.id);
                        selected_habit_id.set(Some(first_habit.id));

                        // Load habit data
                        let habit_result = get_habit(first_habit.id).await;
                        let days_result = get_completed_days(first_habit.id).await;
                        
                        match (habit_result, days_result) {
                            (Ok(habit), Ok(days)) => {
                                info!("Successfully loaded habit data and completed days");
                                let data = HabitData {
                                    title: habit.title,
                                    start_date: habit.start_date,
                                    week_start: WeekStart::from_string(&habit.week_start),
                                    color: habit.color,
                                    completed_days: days,
                                };
                                current_habit_data.set(Some(data));
                            }
                            (Err(e), _) => error!("Failed to load habit: {:?}", e),
                            (_, Err(e)) => error!("Failed to load completed days: {:?}", e),
                        }
                    }
                    Ok(None) => {
                        info!("No habits found, creating default habit");
                        let new_habit = Habit {
                            id: Uuid::new_v4(),
                            title: "Meditation".to_string(),
                            start_date: Local::now().date_naive(),
                            color: "#800080".to_string(),
                            week_start: "sunday".to_string(),
                        };
                        
                        match create_habit(new_habit.clone()).await {
                            Ok(()) => {
                                info!("Successfully created default habit");
                                selected_habit_id.set(Some(new_habit.id));
                            }
                            Err(e) => error!("Failed to create default habit: {:?}", e),
                        }
                    }
                    Err(e) => error!("Failed to get first habit: {:?}", e),
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
                let habit_result = get_habit(id).await;
                let days_result = get_completed_days(id).await;
                
                match (habit_result, days_result) {
                    (Ok(habit), Ok(days)) => {
                        info!("Successfully loaded habit data and completed days");
                        let data = HabitData {
                            title: habit.title,
                            start_date: habit.start_date,
                            week_start: WeekStart::from_string(&habit.week_start),
                            color: habit.color,
                            completed_days: days,
                        };
                        current_habit_data.set(Some(data));
                    }
                    (Err(e), _) => error!("Failed to load habit: {:?}", e),
                    (_, Err(e)) => error!("Failed to load completed days: {:?}", e),
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
                                let habit_result = get_habit(id).await;
                                let days_result = get_completed_days(id).await;
                                
                                match (habit_result, days_result) {
                                    (Ok(habit), Ok(days)) => {
                                        info!("Successfully refreshed habit data");
                                        let data = HabitData {
                                            title: habit.title,
                                            start_date: habit.start_date,
                                            week_start: WeekStart::from_string(&habit.week_start),
                                            color: habit.color,
                                            completed_days: days,
                                        };
                                        current_habit_data.set(Some(data));
                                    }
                                    (Err(e), _) => error!("Failed to refresh habit: {:?}", e),
                                    (_, Err(e)) => error!("Failed to refresh completed days: {:?}", e),
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