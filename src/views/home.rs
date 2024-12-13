use crate::components::{HabitTracker};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        HabitTracker {}
    }
}
