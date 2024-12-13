use crate::models::todo::Todo;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn TodoItem(
    todo: Todo,
    index: usize,
    show_placeholder: bool,
    ondragstart: EventHandler<(DragEvent, Todo)>,
    ondragover: EventHandler<(DragEvent, usize)>,
    ondrop: EventHandler<DragEvent>,
    ondelete: EventHandler<Uuid>,
) -> Element {
    let todo_id = todo.id;

    rsx! {
        Fragment {
            key: "{todo.id}",
            if show_placeholder {
                li {
                    class: "drop-placeholder"
                }
            }
            li {
                key: "{todo.id}",
                class: "todo-item",
                draggable: true,
                ondragstart: move |ev| ondragstart.call((ev, todo.clone())),
                ondragover: move |ev| ondragover.call((ev, index)),
                ondrop: move |ev| ondrop.call(ev),
                span {
                    class: "drag-handle",
                    "⋮"
                }
                span {
                    class: "todo-content",
                    "{todo.content}"
                }
                button {
                    class: "delete-btn",
                    onclick: {
                        move |_| ondelete.call(todo_id)
                    },
                    "Delete"
                }
            }
        }
    }
}
