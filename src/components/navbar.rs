use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            Link {
                to: Route::HomePage {},
                "Home"
            }
            Link {
                to: Route::HabitsPage {},
                "Habits"
            }
            Link {
                to: Route::TodosPage {},
                "Todos"
            }
            Link {
                to: Route::TimelinePageNoParam {},
                "Life"
            }
        }

        Outlet::<Route> {}
    }
}
