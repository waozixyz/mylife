use dioxus::prelude::*;
use views::{HabitsPage, HomePage, TodosPage, TimelinePage, TimelinePageNoParam};
mod components;
mod models;
mod server;
mod views;
mod state;
mod utils;

use crate::models::timeline::SizeInfo;
use crate::components::window_manager::WindowSizeManager;

use components::navbar::Navbar;
use server::state::initialize_db;

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
    // Initialize database regardless of platform
    initialize_db();
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

    use_effect(move || {
        // Initialize the database when the app starts
        initialize_db();
    });

    rsx! {
        div {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            WindowSizeManager {}

            Router::<Route> {}
        }
    }
}