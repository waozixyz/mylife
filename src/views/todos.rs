// TodosPage
use crate::components::todo_day_tabs::DayTabs;
use crate::components::todo_list::TodoList;
use crate::managers::todo_manager::get_todo_manager;
use crate::models::todo::Todo;
use dioxus::prelude::*;

#[component]
pub fn TodosPage() -> Element {
    let mut active_day = use_signal(|| "Monday".to_string());
    let todos = use_signal(Vec::<Todo>::new);

    // Load todos whenever active day changes
    use_effect(move || {
        let mut todos = todos.clone();
        let day = active_day.read().clone();

        spawn(async move {
            let manager = get_todo_manager();
            if let Ok(loaded_todos) = manager.get_todos_by_day(&day).await {
                todos.set(loaded_todos);
            }
        });
    });

    let handle_todos_change = move || {
        let day = active_day.read().clone();
        let mut todos = todos.clone();

        spawn(async move {
            let manager = get_todo_manager();
            if let Ok(loaded_todos) = manager.get_todos_by_day(&day).await {
                todos.set(loaded_todos);
            }
        });
    };

    rsx! {
        div { class: "todos-container",
            DayTabs {
                active_day: active_day.read().clone(),
                on_day_change: move |day| active_day.set(day)
            }
            TodoList {
                day: active_day.read().clone(),
                todos: todos.read().clone(),
                on_todos_change: handle_todos_change
            }
        }
    }
}
