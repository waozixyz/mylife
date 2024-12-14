use crate::components::todo_item::TodoItem;
use crate::db::todos::{delete_todo, load_todos_by_day, save_todo, update_todo_positions};
use crate::models::todo::Todo;
use crate::state::AppState;
use dioxus::prelude::*;
use uuid::Uuid;

const TODO_LIST_CSS: Asset = asset!("/assets/styling/todo_list.css");

#[component]
pub fn TodoList(day: String) -> Element {
    let state = use_context::<AppState>();
    let conn = state.conn.clone();

    let todos = use_signal(Vec::<Todo>::new);
    let mut new_todo = use_signal(String::new);
    let mut dragged_todo = use_signal(|| None::<Todo>);
    let mut drop_index = use_signal(|| None::<usize>);

    // Initial load effect
    {
        let conn = conn.clone();
        let day = day.clone();

        use_effect(move || {
            let conn = conn.clone();
            let mut todos = todos.clone();
            let day = day.clone();

            spawn(async move {
                if let Ok(loaded_todos) = load_todos_by_day(&conn, &day) {
                    todos.set(loaded_todos);
                }
            });

            ()
        });
    }

    let add_todo = {
        let conn = conn.clone();
        move |ev: FormEvent| {
            let conn = conn.clone();
            let mut todos = todos.clone();
            let day = day.clone();

            ev.prevent_default();

            let content = {
                let content = new_todo.read().trim().to_string();
                if !content.is_empty() {
                    new_todo.set(String::new());
                }
                content
            };

            if !content.is_empty() {
                let todo = Todo::new(content, day);

                spawn(async move {
                    if let Ok(()) = save_todo(&conn, &todo) {
                        let mut current_todos = todos.read().clone();
                        current_todos.push(todo);
                        todos.set(current_todos);
                    }
                });
            }
        }
    };

    let handle_delete = {
        let conn = conn.clone();
        move |id: Uuid| {
            let conn = conn.clone();
            let mut todos = todos.clone();

            spawn(async move {
                if let Ok(()) = delete_todo(&conn, id) {
                    let mut current_todos = todos.read().clone();
                    current_todos.retain(|todo| todo.id != id);
                    todos.set(current_todos);
                }
            });
        }
    };

    let handle_drop = {
        let conn = conn.clone();
        move |ev: DragEvent| {
            ev.prevent_default();
            let conn = conn.clone();
            let mut todos = todos.clone();

            let dragged = dragged_todo.read().clone();
            let drop_idx = drop_index.read().clone();

            if let (Some(todo), Some(new_index)) = (dragged, drop_idx) {
                let mut current_todos = todos.read().clone();
                if let Some(old_index) = current_todos.iter().position(|t| t.id == todo.id) {
                    if old_index != new_index {
                        let item = current_todos.remove(old_index);
                        current_todos.insert(new_index, item);

                        let updates: Vec<(Uuid, i32)> = current_todos
                            .iter()
                            .enumerate()
                            .map(|(i, t)| (t.id, i as i32))
                            .collect();

                        todos.set(current_todos);

                        let conn = conn.clone();
                        spawn(async move {
                            let _ = update_todo_positions(&conn, &updates);
                        });
                    }
                }
            }

            dragged_todo.set(None);
            drop_index.set(None);
        }
    };
    let handle_drag_start = move |(_ev, todo): (DragEvent, Todo)| {
        dragged_todo.set(Some(todo));
    };

    let handle_drag_over = move |(ev, index): (DragEvent, usize)| {
        ev.prevent_default();
        if dragged_todo.read().is_some() {
            drop_index.set(Some(index));
        }
    };

    let todos_data = todos.read().clone();
    let current_drop_index = drop_index.read().clone();
    let current_dragged = dragged_todo.read().clone();

    // Clone handlers once before the map
    let handle_delete = handle_delete.clone();
    let handle_drag_start = handle_drag_start.clone();
    let handle_drag_over = handle_drag_over.clone();
    let handle_drop = handle_drop.clone();

    rsx! {
        document::Link { rel: "stylesheet", href: TODO_LIST_CSS }
        div {
            class: "todo-list",
            form {
                onsubmit: add_todo,
                class: "todo-form",
                input {
                    r#type: "text",
                    placeholder: "Add a new todo...",
                    value: "{new_todo.read()}",
                    oninput: move |ev| new_todo.set(ev.value().clone())
                }
                button {
                    r#type: "submit",
                    "Add"
                }
            }
            ul {
                class: "todos",
                {todos_data.iter().enumerate().map(|(index, todo)| {
                    let show_placeholder = current_drop_index == Some(index)
                        && current_dragged.as_ref().map(|t| t.id) != Some(todo.id);

                    rsx! {
                        TodoItem {
                            key: "{todo.id}",
                            todo: todo.clone(),
                            index: index,
                            show_placeholder: show_placeholder,
                            ondelete: handle_delete.clone(),
                            ondragstart: handle_drag_start.clone(),
                            ondragover: handle_drag_over.clone(),
                            ondrop: handle_drop.clone(),
                        }
                    }
                })}
            }
        }
    }
}
