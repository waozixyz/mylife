#![allow(non_snake_case)]

use dioxus_logger::tracing::{info, Level};

use dioxus::prelude::*;

mod app;
mod models;
mod pages;
mod routes;
mod state_manager;
mod ui;
mod utils;
mod yaml_manager;

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");

    info!("starting app");
    launch(app::App);
}
