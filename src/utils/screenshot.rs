use base64::{engine::general_purpose, Engine as _};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use resvg::render;
use resvg::usvg::{Options, Tree};
use tiny_skia::{Pixmap, Transform};
use dioxus_logger::tracing::error;
use image::{DynamicImage, ImageBuffer, Rgba};
use image::GenericImageView;
use image::codecs::webp::WebPEncoder;
use std::fs;
use rand::seq::SliceRandom;

#[cfg(target_arch = "wasm32")]
use js_sys::{Array, Object, Promise};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, Blob, BlobPropertyBag, File, FilePropertyBag, HtmlAnchorElement, Navigator};

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;

#[cfg(not(target_arch = "wasm32"))]
use crate::ui::get_svg_content;

#[cfg(target_arch = "wasm32")]
use include_dir::{include_dir, Dir};
#[cfg(target_arch = "wasm32")]
static LANDSCAPE_IMAGES: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/cards/landscape");
#[cfg(target_arch = "wasm32")]
static PORTRAIT_IMAGES: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/cards/portrait");


fn get_background_images(is_landscape: bool) -> Vec<String> {
    let folder_path = if is_landscape {
        "assets/cards/landscape"
    } else {
        "assets/cards/portrait"
    };

    match fs::read_dir(folder_path) {
        Ok(entries) => entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("webp") {
                        Some(path.to_string_lossy().into_owned())
                    } else {
                        None
                    }
                })
            })
            .collect(),
        Err(e) => {
            error!("Failed to read directory {}: {:?}", folder_path, e);
            Vec::new()
        }
    }
}

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


#[cfg(target_arch = "wasm32")]
fn load_background_image(is_landscape: bool) -> Result<DynamicImage, String> {
    let images_dir = if is_landscape { &LANDSCAPE_IMAGES } else { &PORTRAIT_IMAGES };
    
    let webp_files: Vec<_> = images_dir.files().filter(|f| f.path().extension().and_then(|s| s.to_str()) == Some("webp")).collect();
    
    if webp_files.is_empty() {
        return Err("No background images found".to_string());
    }

    let chosen_image = webp_files
        .choose(&mut rand::thread_rng())
        .ok_or("Failed to choose a random image")?;

    let image_data = chosen_image.contents();
    
    image::load_from_memory(image_data)
        .map_err(|e| format!("Failed to load background image: {:?}", e))
}

#[cfg(not(target_arch = "wasm32"))]
fn load_background_image(is_landscape: bool) -> Result<DynamicImage, String> {
    let background_images = get_background_images(is_landscape);
    
    if background_images.is_empty() {
        return Err("No background images found".to_string());
    }

    let chosen_image = background_images
        .choose(&mut rand::thread_rng())
        .ok_or("Failed to choose a random image")?;

    image::open(chosen_image)
        .map_err(|e| format!("Failed to open background image: {:?}", e))
}

fn render_svg_to_image(svg_content: &str, is_landscape: bool) -> Result<Vec<u8>, String> {
    let opt = Options::default();
    let tree = Tree::from_str(svg_content, &opt).map_err(|e| format!("Failed to parse SVG: {:?}", e))?;


    let background = load_background_image(is_landscape)?;
    let (bg_width, bg_height) = if is_landscape {
        (1344, 768)
    } else {
        (768, 1344)
    };
    info!("Background image size: {}x{}", bg_width, bg_height);
    
    // Resize the background image if necessary
    let background = background.resize_exact(bg_width, bg_height, image::imageops::FilterType::Lanczos3);

    // Calculate the scaling factor for the SVG
    let svg_size = tree.size();
    let svg_aspect_ratio = svg_size.width() / svg_size.height();

    let (svg_width, svg_height) = if is_landscape {
        (800, (800.0 / svg_aspect_ratio).round() as u32)
    } else {
        ((800.0 * svg_aspect_ratio).round() as u32, 800)
    };

    let scale_x = svg_width as f32 / svg_size.width();
    let scale_y = svg_height as f32 / svg_size.height();

    info!("Scaled SVG size: {}x{}", svg_width, svg_height);

    let mut pixmap = Pixmap::new(svg_width, svg_height).ok_or("Failed to create Pixmap")?;

    let transform = Transform::from_scale(scale_x, scale_y);
    render(&tree, transform, &mut pixmap.as_mut());

    // Convert Pixmap to image::RgbaImage
    let svg_image = ImageBuffer::<Rgba<u8>, _>::from_raw(
        svg_width,
        svg_height,
        pixmap.data().to_vec()
    ).ok_or("Failed to create RgbaImage")?;

    // Create a new image with the background dimensions
    let mut final_image = DynamicImage::new_rgba8(bg_width, bg_height);

    // Copy the background onto the final image
    image::imageops::replace(&mut final_image, &background, 0, 0);

    // Calculate the position to center the SVG image
    let x = (bg_width - svg_width) / 2;
    let y = (bg_height - svg_height) / 2;

    info!("Overlay position: ({}, {})", x, y);

    // Overlay the SVG image onto the final image
    image::imageops::overlay(&mut final_image, &svg_image, x.into(), y.into());

    // Encode to WebP
    let mut webp_data = Vec::new();
    WebPEncoder::new(&mut webp_data)
        .encode(
            &final_image.to_rgba8(),
            bg_width,
            bg_height,
            image::ColorType::Rgba8
        )
        .map_err(|e| format!("Failed to encode WebP: {:?}", e))?;

    Ok(webp_data)
}

pub fn take_screenshot(is_landscape: bool) -> Result<String, String> {
    #[cfg(target_arch = "wasm32")]
    let svg_content = get_svg_content_wasm()?;

    #[cfg(not(target_arch = "wasm32"))]
    let svg_content = get_svg_content().ok_or_else(|| "Yaml context not found".to_string())?;

    let processed_svg = process_svg_content(svg_content)?;
    
    info!("Processed SVG content length: {}", processed_svg.len());
    
    let image_data = render_svg_to_image(&processed_svg, is_landscape)
        .map_err(|e| format!("Failed to render SVG to image: {}", e))?;

    let base64_image = general_purpose::STANDARD.encode(&image_data);
    Ok(format!("data:image/webp;base64,{}", base64_image))
}
#[cfg(target_arch = "wasm32")]
fn get_svg_content_wasm() -> Result<String, String> {
    let window = window().ok_or("Failed to get window")?;
    let document = window.document().ok_or("Failed to get document")?;

    info!("Searching for SVG element");
    let svg = document
        .query_selector(".lifetime-view-svg")
        .map_err(|e| format!("Error querying SVG element: {:?}", e))?
        .ok_or("SVG element not found")?;

    Ok(svg.outer_html())
}


#[cfg(target_arch = "wasm32")]
pub fn save_screenshot(data: &Signal<String>) {
    let document = window().unwrap().document().unwrap();
    let a: HtmlAnchorElement = document
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();
    a.set_href(&data().clone());
    a.set_attribute("download", "lifetime_screenshot.png")
        .unwrap();

    document.body().unwrap().append_child(&a).unwrap();
    a.click();
    document.body().unwrap().remove_child(&a).unwrap();
}
#[cfg(not(target_arch = "wasm32"))]
pub fn save_screenshot(data: &Signal<String>) {
    use std::path::PathBuf;
    use chrono::Local;
    use rfd::FileDialog;
    use image::io::Reader as ImageReader;
    use std::io::Cursor;

    let binding = data();
    let image_data = binding.strip_prefix("data:image/webp;base64,").unwrap_or("");
    let decoded = general_purpose::STANDARD.decode(image_data).unwrap();

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let default_filename = format!("mylifetimeline_{}.webp", current_date);

    if let Some(path) = FileDialog::new()
        .set_file_name(&default_filename)
        .add_filter("WebP Image", &["webp"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .save_file() {
        let mut file = std::fs::File::create(&path).unwrap();
        
        if path.extension().and_then(|ext| ext.to_str()) == Some("webp") {
            file.write_all(&decoded).unwrap();
        } else {
            // Convert WebP to JPEG if user chose JPEG
            let image = ImageReader::new(Cursor::new(decoded))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            image.write_to(&mut file, image::ImageFormat::Jpeg).unwrap();
        }
        
        info!("Screenshot saved as {}", path.file_name().unwrap().to_string_lossy());
    } else {
        error!("Screenshot save cancelled or failed");
    }
}
#[cfg(target_arch = "wasm32")]
pub fn share_screenshot(data: &Signal<String>) {
    let data = data.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let window = window().expect("no global `window` exists");
        let navigator: Navigator = window.navigator();

        if let Ok(share_fn) = js_sys::Reflect::get(&navigator, &JsValue::from_str("share")) {
            if share_fn.is_function() {
                let blob_parts = Array::new();
                blob_parts.push(&JsValue::from_str(&data()));

                let blob_property_bag = BlobPropertyBag::new();
                blob_property_bag.set_type("image/png");
                let blob =
                    Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_property_bag)
                        .expect("Failed to create Blob");

                let file_property_bag = FilePropertyBag::new();
                file_property_bag.set_type("image/png");
                let file = File::new_with_blob_sequence_and_options(
                    &js_sys::Array::of1(&blob.into()),
                    "lifetime_screenshot.png",
                    &file_property_bag,
                )
                .expect("Failed to create File");

                let files = Array::new();
                files.push(&file);

                let share_data = Object::new();
                js_sys::Reflect::set(&share_data, &JsValue::from_str("files"), &files).unwrap();

                let share_promise = share_fn
                    .dyn_ref::<js_sys::Function>()
                    .expect("share is not a function")
                    .call1(&navigator, &share_data)
                    .expect("failed to call share");

                let share_promise: Promise = share_promise.dyn_into().unwrap();
                match JsFuture::from(share_promise).await {
                    Ok(_) => info!("Successfully shared the screenshot"),
                    Err(e) => error!("Failed to share: {:?}", e),
                }
            } else {
                error!("Web Share API is not supported");
            }
        } else {
            error!("Web Share API is not supported");
        }
    });
}
