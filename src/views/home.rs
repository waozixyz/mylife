use crate::Route;
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn HomePage() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        div { class: "home-container",
            div { class: "card-grid",
                Link {
                    to: Route::HabitsPage {},
                    class: "nav-card",
                    div { class: "card-title", "Habits" }
                    div { class: "card-description",
                        "Track and maintain your daily habits for better living"
                    }
                }
                Link {
                    to: Route::TodosPage {},
                    class: "nav-card",
                    div { class: "card-title", "Todos" }
                    div { class: "card-description",
                        "Manage your tasks and stay organized"
                    }
                }
                Link {
                    to: Route::TimelinePageNoParam {},
                    class: "nav-card",
                    div { class: "card-title", "Timeline" }
                    div { class: "card-description",
                        "View your life's journey and important moments"
                    }
                }
                // Placeholder for future Routine page
                div {
                    class: "nav-card",
                    div { class: "card-title", "Routine" }
                    div { class: "card-description",
                        "Coming soon: Plan and track your daily routines"
                    }
                }
            }
        }
    }
}
