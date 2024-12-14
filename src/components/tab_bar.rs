use crate::db::habits::{delete_habit, load_all_habits, save_habit, update_habit_title};
use crate::models::habit::Habit;
use crate::state::AppState;
use chrono::Local;
use dioxus::prelude::*;
use uuid::Uuid;

const TAB_BAR_CSS: Asset = asset!("/assets/styling/tab_bar.css");

#[derive(Props, Clone, PartialEq)]
pub struct TabBarProps {
    selected_habit_id: Uuid,
    on_habit_change: EventHandler<Uuid>,
}

#[component]
pub fn TabBar(props: TabBarProps) -> Element {
    let state = use_context::<AppState>();
    let conn = state.conn.clone();

    let mut habits = use_signal(Vec::<Habit>::new);
    let mut editing_tab_id = use_signal(|| None::<Uuid>);
    let mut show_delete_confirm = use_signal(|| None::<Uuid>);

    // Load habits effect remains the same
    {
        let conn = conn.clone();
        let mut habits = habits.clone();
        use_effect(move || {
            let conn = conn.clone();
            spawn(async move {
                if let Ok(loaded_habits) = load_all_habits(&conn) {
                    habits.set(loaded_habits);
                }
            });
        });
    }

    // Create new habit remains the same
    let create_new_habit = {
        let conn = conn.clone();
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

            let _ = save_habit(&conn, &new_habit);

            let mut current_habits = habits.read().clone();
            current_habits.push(new_habit.clone());
            habits.set(current_habits);

            editing_tab_id.set(Some(new_habit.id));
        }
    };

    let render_habit = move |habit: &Habit| {
        let habit = habit.clone();
        let is_selected = habit.id == props.selected_habit_id;
        let is_editing = editing_tab_id.read().map_or(false, |id| id == habit.id);
        let showing_confirm = show_delete_confirm
            .read()
            .map_or(false, |id| id == habit.id);
        let conn = conn.clone();
        let conn_for_keydown = conn.clone(); // Clone for keydown handler
        let conn_for_blur = conn.clone(); // Clone for blur handler

        rsx! {
            div {
                key: habit.id.to_string(),
                class: if is_selected { "tab selected" } else { "tab" },
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
                        div { class: "tab-edit",
                            input {
                                r#type: "text",
                                value: habit.title,
                                onkeydown: move |evt| {
                                    if evt.key() == Key::Enter {
                                        let habits_read = habits.read();
                                        if let Some(habit) = habits_read.iter().find(|h| h.id == habit_id) {
                                            let _ = update_habit_title(&conn_for_keydown, habit_id, &habit.title);
                                        }
                                        editing_tab_id.set(None);
                                    }
                                },
                                onblur: move |_| {
                                    let habits_read = habits.read();
                                    if let Some(habit) = habits_read.iter().find(|h| h.id == habit_id) {
                                        let _ = update_habit_title(&conn_for_blur, habit_id, &habit.title);
                                    }
                                    editing_tab_id.set(None);
                                },
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
                                        let conn = conn.clone();
                                        let _ = delete_habit(&conn, habit.id);
                                        let mut current_habits = habits.read().clone();
                                        current_habits.retain(|h| h.id != habit.id);
                                        habits.set(current_habits);
                                        show_delete_confirm.set(None);
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
        div { class: "tab-bar",
            div { class: "tabs",
                {habits.read().iter().map(render_habit)}
                button {
                    class: "new-tab",
                    onclick: create_new_habit,
                    "+"
                }
            }
        }
    }
}
