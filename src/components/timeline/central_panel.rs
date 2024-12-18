use crate::components::timeline::events_view::EventView;
use crate::components::timeline::lifetime_view::LifetimeView;
use crate::models::timeline::MyLifeApp;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn CentralPanel() -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();

    let on_period_click = move |period_id: Uuid| {
        app_state.with_mut(|state| {
            state.view = "EventView".to_string();
            state.selected_life_period = Some(period_id);
        });
    };

    rsx! {
        div {
            class: "central-panel",
            div {
                class: "central-content",
                {
                    match app_state().view.as_str() {
                        "Lifetime" => rsx! {
                            LifetimeView {
                                on_period_click: on_period_click
                            }
                        },

                        "EventView" => {
                            if let Some(period_id) = app_state().selected_life_period {
                                rsx! {
                                    EventView {
                                        selected_life_period_id: period_id
                                    }
                                }
                            } else {
                                rsx! {
                                    div { "No life period selected" }
                                }
                            }
                        },
                        _ => rsx! {
                            div { "Unknown view" }
                        }
                    }
                }
            }
        }
    }
}
