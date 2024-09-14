// src/app.rs

use crate::models::SizeInfo;
use crate::routes::Route;
use crate::ui::WindowSizeManager;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let size_info = use_signal(|| SizeInfo {
        cell_size: 40.0,
        window_width: 800.0,
        window_height: 600.0,
    });

    use_context_provider(|| size_info);

    rsx! {
        style { {include_str!("../assets/styles/input.css")} }
        style { {include_str!("../assets/styles/modal.css")} }
        style { {include_str!("../assets/styles/views.css")} }
        style { {include_str!("../assets/styles/items.css")} }
        style { {include_str!("../assets/styles/main.css")} }

        Router::<Route> {}
        WindowSizeManager {}
    }
}
