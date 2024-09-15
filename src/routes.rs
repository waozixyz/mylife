use crate::pages::{HomePage, HomePageNoParam, TimelinePage, TimelinePageNoParam};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/?:y")]
    HomePage { y: String },
    #[route("/")]
    HomePageNoParam {},
    #[route("/timeline?:y")]
    TimelinePage { y: String },
    #[route("/timeline")]
    TimelinePageNoParam,
}
