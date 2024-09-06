
use dioxus::prelude::*;
use crate::config_manager::get_config_manager;
use crate::models::MyLifeApp;

#[component]
pub fn SettingsWindow() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    if app_state().show_settings {
        rsx! {
            div {
                class: "settings-window",
                h2 { "Settings" }
                button {
                    onclick: move |_| {
                        app_state.write().show_settings = false;
                    },
                    "Close"
                }
            }
        }
    } else {
        rsx! { Fragment {} }
    }
}