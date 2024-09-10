use dioxus::prelude::*;
use dioxus_logger::tracing::{info, error};
use base64::{engine::general_purpose, Engine as _};
use resvg::usvg::{Tree, Options};
use resvg::render;
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
use crate::ui::get_svg_content;

const SCALE_FACTOR: f32 = 24.0;

fn process_svg_content(svg_content: String) -> Result<String, String> {
    info!("SVG content length: {}", svg_content.len());
    
    let svg_content_cleaned = svg_content.replace(r#" data-dioxus-id="\d+""#, "");
    info!("Cleaned SVG content length: {}", svg_content_cleaned.len());

    let svg_with_namespace = if !svg_content_cleaned.contains("xmlns=") {
        svg_content_cleaned.replace("<svg", r#"<svg xmlns="http://www.w3.org/2000/svg""#)
    } else {
        svg_content_cleaned
    };

    Ok(svg_with_namespace)
}

fn render_svg_to_png(svg_content: &str) -> Result<String, String> {
    let opt = Options::default();
    let tree = Tree::from_str(svg_content, &opt)
        .map_err(|e| format!("Failed to parse SVG: {:?}", e))?;
    
    let size = tree.size();
    let pixmap_width = (size.width() * SCALE_FACTOR).round() as u32;
    let pixmap_height = (size.height() * SCALE_FACTOR).round() as u32;
    info!("Scaled pixmap size: {}x{}", pixmap_width, pixmap_height);
    
    let mut pixmap = Pixmap::new(pixmap_width, pixmap_height)
        .ok_or("Failed to create Pixmap")?;
    
    let transform = Transform::from_scale(SCALE_FACTOR, SCALE_FACTOR);
    render(&tree, transform, &mut pixmap.as_mut());
    
    let png_data = pixmap.encode_png()
        .map_err(|e| format!("Failed to encode PNG: {:?}", e))?;
    
    let base64_png = general_purpose::STANDARD.encode(&png_data);
    info!("Base64 encoded PNG length: {}", base64_png.len());
    
    Ok(format!("data:image/png;base64,{}", base64_png))
}

pub fn take_screenshot() -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    let svg_content = get_svg_content_wasm()?;
    
    #[cfg(not(target_arch = "wasm32"))]
    let svg_content = get_svg_content()
        .ok_or_else(|| "Yaml context not found".to_string())?;

    let processed_svg = process_svg_content(svg_content)?;
    render_svg_to_png(&processed_svg)
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
pub fn save_screenshot(data: &Signal<String>) {
    let binding = data();
    let png_data = binding.strip_prefix("data:image/png;base64,").unwrap_or("");
    let decoded = general_purpose::STANDARD.decode(png_data).unwrap();
    
    let mut file = File::create("lifetime_screenshot.png").unwrap();
    file.write_all(&decoded).unwrap();
    
    info!("Screenshot saved as lifetime_screenshot.png");
}