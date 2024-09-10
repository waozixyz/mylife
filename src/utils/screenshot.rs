use dioxus::prelude::*;
use dioxus_logger::tracing::{info, error};
use base64::{engine::general_purpose, Engine as _};

#[cfg(target_arch = "wasm32")]
use resvg::usvg::{Tree, Options};
#[cfg(target_arch = "wasm32")]
use resvg::render;
#[cfg(target_arch = "wasm32")]
use tiny_skia::{Pixmap, Transform};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, HtmlAnchorElement};

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;
#[cfg(not(target_arch = "wasm32"))]
use screenshots::Screen;

#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::tao::dpi::PhysicalPosition;
#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::tao::dpi::PhysicalSize;


pub fn take_screenshot() -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    {
        take_screenshot_wasm()
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        take_screenshot_native()
    }
}

#[cfg(target_arch = "wasm32")]
fn take_screenshot_wasm() -> Result<String, String> {
    info!("Taking screenshot");
    
    let svg_content = get_svg_content_wasm()?;
    
    info!("SVG content length: {}", svg_content.len());
    
    info!("Creating usvg Options");
    let opt = Options::default();
    info!("Parsing SVG content");

    let svg_content_cleaned = svg_content.replace(r#" data-dioxus-id="\d+""#, "");
    info!("Cleaned SVG content length: {}", svg_content_cleaned.len());

    let svg_with_namespace = if !svg_content_cleaned.contains("xmlns=") {
        svg_content_cleaned.replace("<svg", r#"<svg xmlns="http://www.w3.org/2000/svg""#)
    } else {
        svg_content_cleaned
    };

    let tree = Tree::from_str(&svg_with_namespace, &opt)
        .map_err(|e| format!("Failed to parse SVG: {:?}", e))?;
    
    info!("Getting pixmap size");
    let size = tree.size();
    
    let scale_factor = 24.0; 
    
    let pixmap_width = (size.width() * scale_factor).round() as u32;
    let pixmap_height = (size.height() * scale_factor).round() as u32;
    info!("Scaled pixmap size: {}x{}", pixmap_width, pixmap_height);
    
    let mut pixmap = Pixmap::new(pixmap_width, pixmap_height)
        .ok_or("Failed to create Pixmap")?;
    
    info!("Rendering SVG to pixmap");
    let transform = Transform::from_scale(scale_factor, scale_factor);
    render(&tree, transform, &mut pixmap.as_mut());
    
    info!("Encoding PNG");
    let png_data = pixmap.encode_png()
        .map_err(|e| format!("Failed to encode PNG: {:?}", e))?;
    
    info!("Encoding PNG to base64");
    let base64_png = general_purpose::STANDARD.encode(&png_data);
    info!("Base64 encoded PNG length: {}", base64_png.len());
    
    Ok(format!("data:image/png;base64,{}", base64_png))
}

#[cfg(target_arch = "wasm32")]
fn get_svg_content_wasm() -> Result<String, String> {
    let window = window().ok_or("Failed to get window")?;
    let document = window.document().ok_or("Failed to get document")?;
    
    info!("Searching for SVG element");
    let svg = document.query_selector(".lifetime-view-svg")
        .map_err(|e| format!("Error querying SVG element: {:?}", e))?
        .ok_or("SVG element not found")?;
    
    Ok(svg.outer_html())
}


#[cfg(target_arch = "wasm32")]
pub fn save_screenshot(data: &Signal<String>) {
    let document = window().unwrap().document().unwrap();
    let a: HtmlAnchorElement = document.create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();
    a.set_href(&data().clone());
    a.set_attribute("download", "lifetime_screenshot.png").unwrap();
    
    document.body().unwrap().append_child(&a).unwrap();
    a.click();
    document.body().unwrap().remove_child(&a).unwrap();
}


#[cfg(not(target_arch = "wasm32"))]
fn take_screenshot_native() -> Result<String, String> {
    info!("Taking native screenshot of application window");
    
    let desktop_context = dioxus_desktop::use_window();
    let window = &desktop_context.window;

    let screens = Screen::all().map_err(|e| format!("Failed to get screens: {:?}", e))?;
    
    for screen in screens {
        let window_pos: PhysicalPosition<i32> = window.outer_position()
            .map_err(|e| format!("Failed to get window position: {:?}", e))?;
        let window_size: PhysicalSize<u32> = window.outer_size();

        let image = screen.capture_area(
            window_pos.x,
            window_pos.y,
            window_size.width,
            window_size.height
        ).map_err(|e| format!("Failed to capture window: {:?}", e))?;

        info!("Encoding PNG");
        let mut png_data = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_data);
        image.write_to(&mut cursor, image::ImageOutputFormat::Png)
            .map_err(|e| format!("Failed to encode PNG: {:?}", e))?;
        
        info!("Encoding PNG to base64");
        let base64_png = general_purpose::STANDARD.encode(&png_data);
        info!("Base64 encoded PNG length: {}", base64_png.len());
        
        return Ok(format!("data:image/png;base64,{}", base64_png));
    }

    Err("No screens found".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_screenshot(data: &Signal<String>) {
    let binding = data();
    let png_data = binding.strip_prefix("data:image/png;base64,").unwrap_or("");
    let decoded = general_purpose::STANDARD.decode(png_data).unwrap();
    
    let mut file = File::create("lifetime_screenshot.png").unwrap();
    file.write_all(&decoded).unwrap();
    
    info!("Screenshot saved as lifetime_screenshot.png");
}
