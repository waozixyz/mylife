// src/pages/home_page.rs

use crate::models::{MyLifeApp, Yaml};
use crate::state_manager::initialize_state;
use dioxus::prelude::*;
use crate::routes::Route;
use rand::seq::SliceRandom;
use crate::utils::image_utils::get_background_images;

#[component]
pub fn HomePage(y: String) -> Element {
    let (yaml_state, app_state) = initialize_state(&y);
    let yaml_state = use_signal(|| yaml_state);
    let app_state = use_signal(|| app_state);

    use_context_provider(|| yaml_state);
    use_context_provider(|| app_state);

    let background_image = use_signal(|| get_random_background_image());

    rsx! {
        style { {include_str!("../../assets/styles/home.css")} }
        div {
            class: "home-container",
            style: "background-image: url('{background_image}');",

            div {
                class: "title",
                "MyLife"
            }

            div {
                class: "button-container",
                Link {
                    to: Route::TimelinePage { y: String::new() },
                    class: "view-timeline-button",
                    "View Timeline"
                }
            }
        }
    }
}

fn get_random_background_image() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::window;
        let window = window().unwrap();
        let is_landscape = window.inner_width().unwrap().as_f64().unwrap() > window.inner_height().unwrap().as_f64().unwrap();
        let images = get_background_images(is_landscape);
        images.choose(&mut rand::thread_rng())
            .cloned()
            .unwrap_or_else(|| "".to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For non-wasm, we'll assume landscape for simplicity
        // In a real-world scenario, you might want to pass this information from the platform-specific code
        let is_landscape = true;
        let images = get_background_images(is_landscape);
        images.choose(&mut rand::thread_rng())
            .cloned()
            .unwrap_or_else(|| "".to_string())
    }
}