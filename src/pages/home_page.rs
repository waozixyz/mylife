// src/pages/home_page.rs

use crate::models::SizeInfo;
use crate::routes::Route;
use crate::state_manager::initialize_state;
use crate::utils::image_utils::get_background_images;
#[cfg(target_arch = "wasm32")]
use base64::{engine::general_purpose, Engine as _};
use dioxus::prelude::*;
use rand::seq::SliceRandom;

#[component]
pub fn HomePage(y: String) -> Element {
    let (yaml_state, app_state) = initialize_state(&y);
    let yaml_state = use_signal(|| yaml_state);
    let app_state = use_signal(|| app_state);

    use_context_provider(|| yaml_state);
    use_context_provider(|| app_state);

    let background_image = use_signal(get_random_background_image);

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
                    class: "button home-page-button",
                    "View Timeline"
                }
            }
            {
                #[cfg(not(target_arch = "wasm32"))]
                rsx! {
                    div {
                        class: "button-container",
                        button {
                            class: "home-page-button",
                            onclick: move |_| {
                                std::process::exit(0);
                            },
                            "Quit"
                        }
                    }
                }
            }

        }
    }
}

fn get_random_background_image() -> String {
    let size_info = use_context::<Signal<SizeInfo>>();
    let is_landscape = size_info().window_width > size_info().window_height;
    let images = get_background_images(is_landscape);

    #[cfg(target_arch = "wasm32")]
    {
        images
            .choose(&mut rand::thread_rng())
            .map(|&img| {
                format!(
                    "data:image/webp;base64,{}",
                    general_purpose::STANDARD.encode(img)
                )
            })
            .unwrap_or_else(|| "".to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        images
            .choose(&mut rand::thread_rng())
            .cloned()
            .unwrap_or_else(|| "".to_string())
    }
}
