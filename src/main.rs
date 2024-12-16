use dioxus::prelude::*;
use views::{HabitsPage, HomePage, TodosPage, TestPage};
mod components;
mod models;
mod server;
mod views;
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
    
    #[route("/test")]
    TestPage {},
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
    use_effect(move || {
        // Initialize the database when the app starts
        initialize_db();
    });

    rsx! {
        div {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            Router::<Route> {}
        }
    }
}