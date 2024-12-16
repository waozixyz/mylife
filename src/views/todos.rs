use crate::components::todo_day_tabs::DayTabs;
use crate::components::todo_list::TodoList;
use dioxus::prelude::*;

#[component]
pub fn TodosPage() -> Element {
    let mut active_day = use_signal(|| "Monday".to_string());

    rsx! {
        div { class: "todos-container",
            DayTabs {
                active_day: active_day.read().clone(),
                on_day_change: move |day| active_day.set(day)
            }
            TodoList {
                day: active_day.read().clone()
            }
        }
    }
}
