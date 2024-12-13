use dioxus::prelude::*;
use std::sync::Arc;
use views::{HabitsPage, HomePage, TodosPage};

mod components;
mod db;
mod models;
mod state;
mod views;

use components::navbar::Navbar;

use db::core::init_db;
use state::AppState;

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
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    // Initialize database connection
    Arc::new(init_db().expect("Failed to initialize database"));
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Set up the context provider with our database connection
    let _state = use_context_provider(|| AppState {
        conn: Arc::new(init_db().expect("Failed to initialize database")),
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
