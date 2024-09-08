#![allow(non_snake_case)]

use dioxus_logger::tracing::{Level, info};

use dioxus::prelude::*;

mod app;
mod utils;
mod ui;
mod yaml_manager;
mod models;

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");

    info!("starting app");
    launch(app::App);
}
