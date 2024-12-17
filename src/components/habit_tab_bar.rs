use crate::managers::habit_manager::get_habit_manager;
use crate::models::habit::{Habit, WeekStart};
use chrono::Local;
use dioxus::prelude::*;
use tracing::{debug, error, info};
use uuid::Uuid;

const TAB_BAR_CSS: Asset = asset!("/assets/styling/habits_tab_bar.css");

#[derive(Props, Clone, PartialEq)]
pub struct TabBarProps {
    selected_habit_id: Uuid,
    on_habit_change: EventHandler<Uuid>,
}

#[component]
pub fn TabBar(props: TabBarProps) -> Element {
    let mut habits = use_signal(Vec::<Habit>::new);
    let mut editing_tab_id = use_signal(|| None::<Uuid>);
    let mut show_delete_confirm = use_signal(|| None::<Uuid>);

    // Load habits effect
    {
        let mut habits = habits.clone();
        use_effect(move || {
            spawn(async move {
                info!("Loading habits");
                let manager = get_habit_manager();
                match manager.get_all_habits().await {
                    Ok(loaded_habits_data) => {
                        info!("Successfully loaded {} habits", loaded_habits_data.len());
                        let habits_vec = loaded_habits_data
                            .into_iter()
                            .map(|(id, data)| Habit {
                                id,
                                title: data.title,
                                start_date: data.start_date,
                                color: data.color,
                                week_start: data.week_start.to_string(),
                            })
                            .collect();
                        habits.set(habits_vec);
                    }
                    Err(e) => error!("Failed to load habits: {:?}", e),
                }
            });
        });
    }

    // Create new habit
    let create_new_habit = {
        let mut habits = habits.clone();
        let mut editing_tab_id = editing_tab_id.clone();

        move |_| {
            let new_habit = Habit {
                id: Uuid::new_v4(),
                title: "New Habit".to_string(),
                start_date: Local::now().date_naive(),
                color: "#800080".to_string(),
                week_start: "sunday".to_string(),
            };

            spawn({
                let new_habit = new_habit.clone();
                let mut habits = habits.clone();
                let mut editing_tab_id = editing_tab_id.clone();

                async move {
                    info!("Creating new habit with id: {}", new_habit.id);
                    let manager = get_habit_manager();
                    let habit_data = crate::models::habit::HabitData {
                        title: new_habit.title.clone(),
                        start_date: new_habit.start_date,
                        color: new_habit.color.clone(),
                        week_start: WeekStart::from_string(&new_habit.week_start),
                        completed_days: Vec::new(),
                    };

                    match manager.create_habit(new_habit.id, habit_data).await {
                        Ok(_) => {
                            let mut current_habits = habits.read().clone();
                            current_habits.push(new_habit.clone());
                            habits.set(current_habits);
                            editing_tab_id.set(Some(new_habit.id));
                            info!("Successfully created new habit");
                        }
                        Err(e) => error!("Failed to create habit: {:?}", e),
                    }
                }
            });
        }
    };

    let render_habit = move |habit: &Habit| {
        let habit = habit.clone();
        let is_selected = habit.id == props.selected_habit_id;
        let is_editing = editing_tab_id.read().map_or(false, |id| id == habit.id);
        let showing_confirm = show_delete_confirm
            .read()
            .map_or(false, |id| id == habit.id);

        rsx! {
            div {
                key: habit.id.to_string(),
                class: if is_selected { "habits-tab selected" } else { "habits-tab" },
                onclick: move |_| {
                    if !is_editing && !showing_confirm {
                        editing_tab_id.set(None);
                        props.on_habit_change.call(habit.id);
                    }
                },
                ondoubleclick: move |_| {
                    if !showing_confirm {
                        editing_tab_id.set(Some(habit.id));
                    }
                },

                {if is_editing {
                    let habit_id = habit.id;
                    rsx! {
                        div { class: "habits-tab-edit",
                            input {
                                r#type: "text",
                                value: habit.title.clone(),
                                onkeydown: {
                                    let habit_title = habit.title.clone();

                                    move |evt: Event<KeyboardData>| {
                                        if evt.key() == Key::Enter {
                                            let habit_title = habit.title.clone();
                                            spawn({
                                                let mut editing_tab_id = editing_tab_id.clone();
                                                async move {
                                                    let manager = get_habit_manager();
                                                    if let Ok(Some(mut data)) = manager.get_habit(habit_id).await {
                                                        data.title = habit_title;
                                                        if let Ok(_) = manager.update_habit(habit_id, data).await {
                                                            editing_tab_id.set(None);
                                                            info!("Successfully updated habit title");
                                                        }
                                                    }
                                                }
                                            });
                                        }
                                    }
                                },
                                onblur: move |_| editing_tab_id.set(None),
                                oninput: move |evt: Event<FormData>| {
                                    let mut current_habits = habits.read().clone();
                                    if let Some(habit) = current_habits.iter_mut().find(|h| h.id == habit_id) {
                                        habit.title = evt.data.value().clone();
                                        habits.set(current_habits);
                                    }
                                }
                            }
                            button {
                                class: "delete-btn",
                                onclick: move |_| show_delete_confirm.set(Some(habit_id)),
                                i { class: "fas fa-trash" }
                            }
                        }
                    }
                } else if showing_confirm {
                    rsx! {
                        div { class: "confirm-delete",
                            span { "Delete {habit.title}?" }
                            div { class: "confirm-actions",
                                button {
                                    class: "confirm-yes",
                                    onclick: move |_| {
                                        spawn({
                                            let habit_id = habit.id;
                                            let mut habits = habits.clone();
                                            let mut show_delete_confirm = show_delete_confirm.clone();

                                            async move {
                                                info!("Deleting habit: {}", habit_id);
                                                let manager = get_habit_manager();
                                                match manager.delete_habit(habit_id).await {
                                                    Ok(_) => {
                                                        let mut current_habits = habits.read().clone();
                                                        current_habits.retain(|h| h.id != habit_id);
                                                        habits.set(current_habits);
                                                        show_delete_confirm.set(None);
                                                        info!("Successfully deleted habit");
                                                    }
                                                    Err(e) => error!("Failed to delete habit: {:?}", e),
                                                }
                                            }
                                        });
                                    },
                                    "Yes"
                                }
                                button {
                                    class: "confirm-no",
                                    onclick: move |_| show_delete_confirm.set(None),
                                    "No"
                                }
                            }
                        }
                    }
                } else {
                    rsx! { span { "{habit.title}" } }
                }}
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: TAB_BAR_CSS }
        div { class: "habits-tab-bar",
            div { class: "habits-tabs",
                {habits.read().iter().map(render_habit)}
                button {
                    class: "new-habits-tab",
                    onclick: create_new_habit,
                    "+"
                }
            }
        }
    }
}
