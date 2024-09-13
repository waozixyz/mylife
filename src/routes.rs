use dioxus::prelude::*;
use crate::pages::{HomePage, TimelinePage};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/?:y")]
    HomePage { y: String },
    #[route("/timeline?:y")]
    TimelinePage { y: String },
}
