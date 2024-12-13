#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::info;

mod app;
mod models;
mod pages;
mod routes;
mod state_manager;
mod ui;
mod utils;
mod yaml_manager;

fn main() {

    info!("starting app");
    launch(app::App);
}
