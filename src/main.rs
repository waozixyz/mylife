use dioxus::prelude::*;
use views::{HabitsPage, HomePage, TimelinePage, TimelinePageNoParam, TodosPage};
mod components;
mod models;
mod state;
mod storage;
mod utils;
mod views;

use crate::components::window_manager::WindowSizeManager;
use crate::models::timeline::SizeInfo;

use components::navbar::Navbar;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    HomePage {},

    #[route("/habits")]
    HabitsPage {},

    #[route("/todos")]
    TodosPage {},

    #[route("/timeline?:y")]
    TimelinePage { y: String },

    #[route("/timeline")]
    TimelinePageNoParam,

}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let size_info = use_signal(|| SizeInfo {
        cell_size: 40.0,
        window_width: 800.0,
        window_height: 600.0,
    });

    use_context_provider(|| size_info);

    rsx! {
        div {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            WindowSizeManager {}

            Router::<Route> {}
        }
    }
}
