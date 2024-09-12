// utils/imag_utils.rs

use resvg::render;
use resvg::usvg::{Options, Tree};
use tiny_skia::{Pixmap, Transform};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use image::codecs::webp::WebPEncoder;
use std::fs;
use rand::seq::SliceRandom;
use crate::models::LegendItem;
use rusttype::{Font, Scale};
use hex_color::HexColor;
use dioxus_logger::tracing::{info, error};

#[cfg(target_arch = "wasm32")]
use include_dir::{include_dir, Dir};
#[cfg(target_arch = "wasm32")]
static LANDSCAPE_IMAGES: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/cards/landscape");
#[cfg(target_arch = "wasm32")]
static PORTRAIT_IMAGES: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/cards/portrait");

pub fn render_legend(legend_items: &[LegendItem], width: u32) -> RgbaImage {
    let item_height = 20;
    let padding = 5;
    let height = legend_items.len() as u32 * item_height + 2 * padding;
    
    let mut img = RgbaImage::new(width, height);
    let font = Font::try_from_bytes(include_bytes!("../../assets/JacquesFrancoisShadow-Regular.ttf")).unwrap();
    let scale = Scale::uniform(12.0);

    // Fill the entire background with a light color
    for pixel in img.pixels_mut() {
        *pixel = Rgba([240, 240, 240, 255]); // Light gray background
    }

    for (i, item) in legend_items.iter().enumerate() {
        let y = (i as u32 * item_height) + padding;
        
        // Parse hex color
        let color = HexColor::parse(&item.color).unwrap_or(HexColor::default());
        
        // Draw colored rectangle
        for py in y..y+item_height {
            for px in padding..width-padding {
                img.put_pixel(px, py, Rgba([color.r, color.g, color.b, 255]));
            }
        }

        // Draw text
        let text = format!("{} ({})", item.name, item.start);
        draw_text_mut(&mut img, Rgba([0, 0, 0, 255]), (padding + 5) as i32, (y + 2) as i32, scale, &font, &text);
    }

    img
}

pub fn draw_text_mut(image: &mut RgbaImage, color: Rgba<u8>, x: i32, y: i32, scale: Scale, font: &Font, text: &str) {
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font.layout(text, scale, rusttype::point(x as f32, y as f32 + v_metrics.ascent)).collect();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, gv| {
                let gx = gx as i32 + bounding_box.min.x;
                let gy = gy as i32 + bounding_box.min.y;
                if gx >= 0 && gx < image.width() as i32 && gy >= 0 && gy < image.height() as i32 {
                    let pixel = image.get_pixel_mut(gx as u32, gy as u32);
                    let font_color = (color.0[0] as f32 * gv, color.0[1] as f32 * gv, color.0[2] as f32 * gv, color.0[3] as f32 * gv);
                    *pixel = Rgba([
                        (font_color.0) as u8,
                        (font_color.1) as u8,
                        (font_color.2) as u8,
                        (font_color.3) as u8,
                    ]);
                }
            });
        }
    }
}

pub fn get_background_images(is_landscape: bool) -> Vec<String> {
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

pub fn process_svg_content(svg_content: String) -> Result<String, String> {
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
pub fn load_background_image(is_landscape: bool) -> Result<DynamicImage, String> {
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
pub fn load_background_image(is_landscape: bool) -> Result<DynamicImage, String> {
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

pub fn render_svg_to_image(svg_content: &str, is_landscape: bool, legend_items: &[LegendItem]) -> Result<Vec<u8>, String> {
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

    // Calculate maximum dimensions that will fit within the background
    let max_svg_width = bg_width.min(800);
    let max_svg_height = bg_height.min(800);

    let (svg_width, svg_height) = if svg_aspect_ratio > (max_svg_width as f32 / max_svg_height as f32) {
        (max_svg_width, (max_svg_width as f32 / svg_aspect_ratio).round() as u32)
    } else {
        ((max_svg_height as f32 * svg_aspect_ratio).round() as u32, max_svg_height)
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

    // Render the legend
    let legend_height = legend_items.len() as u32 * 20 + 10; // 20px per item + 10px padding
    let legend_image = render_legend(legend_items, bg_width);

    // Create a new image with space for both the SVG and the legend
    let final_width = bg_width;
    let final_height = bg_height + legend_height;
    let mut final_image = DynamicImage::new_rgba8(final_width, final_height);

    // Copy the background onto the final image
    image::imageops::replace(&mut final_image, &background, 0, 0);

    // Calculate the position to center the SVG image
    let x = (bg_width - svg_width) / 2;
    let y = (bg_height - svg_height) / 2;

    info!("Overlay position: ({}, {})", x, y);

    // Overlay the SVG image onto the final image
    image::imageops::overlay(&mut final_image, &svg_image, x.into(), y.into());

    // Add the legend to the bottom of the image
    image::imageops::overlay(&mut final_image, &legend_image, 0, bg_height.into());

    // Encode to WebP
    let mut webp_data = Vec::new();
    WebPEncoder::new(&mut webp_data)
        .encode(
            &final_image.to_rgba8(),
            final_width,
            final_height,
            image::ColorType::Rgba8
        )
        .map_err(|e| format!("Failed to encode WebP: {:?}", e))?;

    Ok(webp_data)
}