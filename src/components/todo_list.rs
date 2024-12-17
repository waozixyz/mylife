use crate::components::todo_item::TodoItem;
use crate::managers::todo_manager::get_todo_manager;
use crate::models::todo::Todo;
use dioxus::prelude::*;
use tracing::debug;
use uuid::Uuid;

const TODO_LIST_CSS: Asset = asset!("/assets/styling/todo_list.css");

#[derive(Props, Clone, PartialEq)]
pub struct TodoListProps {
    day: String,
    todos: Vec<Todo>,
    on_todos_change: EventHandler<()>,
}

#[component]
pub fn TodoList(props: TodoListProps) -> Element {
    let mut new_todo = use_signal(String::new);
    let mut dragged_todo = use_signal(|| None::<Todo>);
    let mut drop_index = use_signal(|| None::<usize>);
    let todo_manager = get_todo_manager();

    let add_todo = {
        let on_todos_change = props.on_todos_change.clone();
        let day = props.day.clone();
        move |ev: FormEvent| {
            ev.prevent_default();

            let content = {
                let content = new_todo.read().trim().to_string();
                if !content.is_empty() {
                    new_todo.set(String::new());
                }
                content
            };

            if !content.is_empty() {
                let day = day.clone();
                let on_todos_change = on_todos_change.clone();
                spawn(async move {
                    if let Ok(()) = todo_manager.create_todo(content, day).await {
                        on_todos_change.call(());
                    }
                });
            }
        }
    };

    let handle_delete = {
        let on_todos_change = props.on_todos_change.clone();
        move |id: Uuid| {
            let on_todos_change = on_todos_change.clone();
            spawn(async move {
                if let Ok(()) = todo_manager.delete_todo(id).await {
                    on_todos_change.call(());
                }
            });
        }
    };

    let handle_drop = {
        let day = props.day.clone();
        let on_todos_change = props.on_todos_change.clone();
        let todos = props.todos.clone();
        move |ev: DragEvent| {
            ev.prevent_default();

            let dragged = dragged_todo.read().clone();
            let drop_idx = drop_index.read().clone();

            if let (Some(todo), Some(new_index)) = (dragged, drop_idx) {
                let mut current_todos = todos.clone();
                if let Some(old_index) = current_todos.iter().position(|t| t.id == todo.id) {
                    if old_index != new_index {
                        let item = current_todos.remove(old_index);
                        current_todos.insert(new_index, item);

                        let updates: Vec<(Uuid, i32)> = current_todos
                            .iter()
                            .enumerate()
                            .map(|(i, t)| (t.id, i as i32))
                            .collect();

                        let day = day.clone();
                        let on_todos_change = on_todos_change.clone();
                        spawn(async move {
                            if let Ok(()) = todo_manager.update_positions(&day, updates).await {
                                on_todos_change.call(());
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

    let current_drop_index = drop_index.read().clone();
    let current_dragged = dragged_todo.read().clone();

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
                {props.todos.iter().enumerate().map(|(index, todo)| {
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
