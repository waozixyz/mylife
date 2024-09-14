use crate::models::SizeInfo;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{closure::Closure, JsCast};

#[allow(dead_code)]
fn calculate_size_info(width: u32, height: u32) -> SizeInfo {
    let cell_size = width.min(height) as f64 / 20.0;

    SizeInfo {
        cell_size,
        window_width: width as f64,
        window_height: height as f64,
    }
}

#[component]
pub fn WindowSizeManager() -> Element {
    let size_info = use_context::<Signal<SizeInfo>>();

    #[cfg(not(target_arch = "wasm32"))]
    {
        use_effect(move || {
            to_owned![size_info];

            let window = dioxus_desktop::use_window();

            // Make sure the window is resizable
            window.set_resizable(true);

            // Get the initial size
            let initial_size = window.inner_size();
            size_info.set(calculate_size_info(initial_size.width, initial_size.height));

            // Set up a timer to check for window size changes
            use_future(move || {
                let mut size_info = size_info;
                let initial_size = initial_size;
                let window_clone = window.clone();
                async move {
                    loop {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        let new_size = window_clone.inner_size();
                        if new_size != initial_size {
                            size_info.set(calculate_size_info(new_size.width, new_size.height));
                        }
                    }
                }
            });
        });
    }

    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            to_owned![size_info];

            // Get the initial size
            let window = web_sys::window().unwrap();
            let initial_width = window.inner_width().unwrap().as_f64().unwrap() as u32;
            let initial_height = window.inner_height().unwrap().as_f64().unwrap() as u32;
            size_info.set(calculate_size_info(initial_width, initial_height));

            // Set up a resize event listener
            let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                let window = web_sys::window().unwrap();
                let new_width = window.inner_width().unwrap().as_f64().unwrap() as u32;
                let new_height = window.inner_height().unwrap().as_f64().unwrap() as u32;
                size_info.set(calculate_size_info(new_width, new_height));
            }) as Box<dyn FnMut(_)>);

            window
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget(); // Prevents the closure from being dropped
        });
    }

    provide_context(size_info);

    None
}
