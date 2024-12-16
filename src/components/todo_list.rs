use crate::components::todo_item::TodoItem;
use crate::models::todo::Todo;
use crate::storage::todos::get_todo_manager;
use chrono::Local;
use dioxus::prelude::*;
use uuid::Uuid;

const TODO_LIST_CSS: Asset = asset!("/assets/styling/todo_list.css");

#[component]
pub fn TodoList(day: String) -> Element {
    let todos = use_signal(Vec::<Todo>::new);
    let mut new_todo = use_signal(String::new);
    let mut dragged_todo = use_signal(|| None::<Todo>);
    let mut drop_index = use_signal(|| None::<usize>);

    // Initial load effect
    {
        let day = day.clone();
        use_effect(move || {
            let mut todos = todos.clone();
            let day = day.clone();

            spawn(async move {
                let manager = get_todo_manager();
                if let Ok(loaded_todos) = manager.get_todos_by_day(&day).await {
                    let todos_vec = loaded_todos
                        .into_iter()
                        .map(|data| Todo {
                            id: data.id,
                            content: data.content,
                            day: day.clone(),
                            created_at: data.created_at,
                            position: data.position,
                        })
                        .collect();
                    todos.set(todos_vec);
                }
            });

            ()
        });
    }

    let add_todo = {
        let day = day.clone();
        move |ev: FormEvent| {
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
                spawn(async move {
                    let manager = get_todo_manager();
                    if let Ok(()) = manager.create_todo(content.clone(), day.clone()).await {
                        if let Ok(updated_todos) = manager.get_todos_by_day(&day).await {
                            let todos_vec = updated_todos
                                .into_iter()
                                .map(|data| Todo {
                                    id: data.id,
                                    content: data.content,
                                    day: day.clone(),
                                    created_at: data.created_at,
                                    position: data.position,
                                })
                                .collect();
                            todos.set(todos_vec);
                        }
                    }
                });
            }
        }
    };

    let handle_delete = {
        let day = day.clone();
        move |id: Uuid| {
            let mut todos = todos.clone();
            let day = day.clone();

            spawn(async move {
                let manager = get_todo_manager();
                if let Ok(()) = manager.delete_todo(id).await {
                    if let Ok(updated_todos) = manager.get_todos_by_day(&day).await {
                        let todos_vec = updated_todos
                            .into_iter()
                            .map(|data| Todo {
                                id: data.id,
                                content: data.content,
                                day: day.clone(),
                                created_at: data.created_at,
                                position: data.position,
                            })
                            .collect();
                        todos.set(todos_vec);
                    }
                }
            });
        }
    };

    let handle_drop = {
        let day = day.clone();
        move |ev: DragEvent| {
            ev.prevent_default();
            let mut todos = todos.clone();
            let day = day.clone();

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

                        spawn(async move {
                            let manager = get_todo_manager();
                            if let Ok(()) = manager.update_positions(&day, updates).await {
                                if let Ok(updated_todos) = manager.get_todos_by_day(&day).await {
                                    let todos_vec = updated_todos
                                        .into_iter()
                                        .map(|data| Todo {
                                            id: data.id,
                                            content: data.content,
                                            day: day.clone(),
                                            created_at: data.created_at,
                                            position: data.position,
                                        })
                                        .collect();
                                    todos.set(todos_vec);
                                }
                            }
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
