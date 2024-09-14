use crate::pages::{HomePage, TimelinePage};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/?:y")]
    HomePage { y: String },
    #[route("/timeline?:y")]
    TimelinePage { y: String },
}
