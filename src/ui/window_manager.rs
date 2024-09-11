use dioxus::prelude::*;
use crate::models::SizeInfo;
use dioxus_desktop::tao::dpi::PhysicalSize;

fn calculate_size_info(size: PhysicalSize<u32>) -> SizeInfo {
    let cell_size = size.width.min(size.height) as f64 / 20.0; 

    SizeInfo {
        cell_size,
        window_width: size.width as f64,
        window_height: size.height as f64,
    }
}

#[component]
pub fn WindowSizeManager() -> Element {
    let mut size_info = use_context::<Signal<SizeInfo>>();

    use_effect(move || {
        to_owned![size_info];
        
        let window = dioxus_desktop::use_window();
        
        // Make sure the window is resizable
        window.set_resizable(true);
        
        // Get the initial size
        let initial_size = window.inner_size();
        size_info.set(calculate_size_info(initial_size));
        
        // Set up a timer to check for window size changes
        use_future(move || {
            let mut size_info = size_info.clone();
            let initial_size = initial_size;
            let window_clone = window.clone();
            async move {
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    let new_size = window_clone.inner_size();
                    if new_size != initial_size {
                        size_info.set(calculate_size_info(new_size));
                    }
                }
            }
        });
    });

    provide_context(size_info);

    None
}