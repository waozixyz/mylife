// utils/imag_utils.rs

use crate::models::LegendItem;
#[cfg(not(target_arch = "wasm32"))]
use dioxus_logger::tracing::error;
use dioxus_logger::tracing::info;

use hex_color::HexColor;
#[cfg(target_arch = "wasm32")]
use image::codecs::png::PngEncoder;
#[cfg(not(target_arch = "wasm32"))]
use image::codecs::webp::WebPEncoder;
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};

use rand::seq::SliceRandom;

use rusttype::{Font, Scale};
#[cfg(not(target_arch = "wasm32"))]
use std::fs;

use resvg::render;
use resvg::usvg::{Options, Tree};
use tiny_skia::{Pixmap, Transform};

#[cfg(target_arch = "wasm32")]
const LANDSCAPE_IMAGES: [&[u8]; 9] = [
    include_bytes!("../../assets/cards/landscape/1.webp"),
    include_bytes!("../../assets/cards/landscape/2.webp"),
    include_bytes!("../../assets/cards/landscape/3.webp"),
    include_bytes!("../../assets/cards/landscape/4.webp"),
    include_bytes!("../../assets/cards/landscape/5.webp"),
    include_bytes!("../../assets/cards/landscape/6.webp"),
    include_bytes!("../../assets/cards/landscape/7.webp"),
    include_bytes!("../../assets/cards/landscape/8.webp"),
    include_bytes!("../../assets/cards/landscape/9.webp"),
];

#[cfg(target_arch = "wasm32")]
const PORTRAIT_IMAGES: [&[u8]; 9] = [
    include_bytes!("../../assets/cards/portrait/1.webp"),
    include_bytes!("../../assets/cards/portrait/2.webp"),
    include_bytes!("../../assets/cards/portrait/3.webp"),
    include_bytes!("../../assets/cards/portrait/4.webp"),
    include_bytes!("../../assets/cards/portrait/5.webp"),
    include_bytes!("../../assets/cards/portrait/6.webp"),
    include_bytes!("../../assets/cards/portrait/7.webp"),
    include_bytes!("../../assets/cards/portrait/8.webp"),
    include_bytes!("../../assets/cards/portrait/9.webp"),
];
pub fn draw_title(image: &mut RgbaImage, text: &str, font: &Font<'_>, is_landscape: bool) {
    let scale = if is_landscape {
        Scale::uniform(100.0) // Smaller font size for landscape
    } else {
        Scale::uniform(200.0) // Keep the large size for portrait
    };
    let color = Rgba([255, 255, 255, 255]); // White color
    let _v_metrics = font.v_metrics(scale);

    // Center the text horizontally
    let text_width = font
        .layout(text, scale, rusttype::point(0.0, 0.0))
        .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .last()
        .unwrap_or(0.0);

    let x = ((image.width() as f32 - text_width) / 2.0).round() as i32;
    let y = 20; // 20 pixels from the top

    draw_text_mut(image, color, x, y, scale, font, text);
}

pub fn render_legend(legend_items: &[LegendItem], width: u32) -> RgbaImage {
    let item_height = 40; // Increased height for larger text
    let padding = 10;
    let items_per_row = 2;
    let rows = (legend_items.len() + items_per_row - 1) / items_per_row;
    let height = rows as u32 * item_height + 2 * padding;

    let mut img = RgbaImage::new(width, height);
    let font = Font::try_from_bytes(include_bytes!("../../assets/Handjet-Regular.ttf")).unwrap();
    let scale = Scale::uniform(24.0); // Increased font size

    // Fill the entire background with a semi-transparent black
    for pixel in img.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 128]); // Semi-transparent black background
    }

    for (i, item) in legend_items.iter().enumerate() {
        let row = i / items_per_row;
        let col = i % items_per_row;
        let x = col as u32 * (width / items_per_row as u32);
        let y = row as u32 * item_height + padding;

        // Parse hex color
        let color = HexColor::parse(&item.color).unwrap_or_default();

        // Draw colored rectangle
        let rect_width = 30;
        let rect_height = 30;
        let rect_x = x + padding;
        let rect_y = y + (item_height - rect_height) / 2;
        for py in rect_y..rect_y + rect_height {
            for px in rect_x..rect_x + rect_width {
                img.put_pixel(px, py, Rgba([color.r, color.g, color.b, 255]));
            }
        }

        // Draw text
        let text = format!("{} ({})", item.name, item.start);
        let text_x = rect_x + rect_width + padding;
        let text_y = y + (item_height - scale.y as u32) / 2;
        draw_text_mut(
            &mut img,
            Rgba([255, 255, 255, 255]),
            text_x as i32,
            text_y as i32,
            scale,
            &font,
            &text,
        );
    }

    img
}

pub fn draw_text_mut(
    image: &mut RgbaImage,
    color: Rgba<u8>,
    x: i32,
    y: i32,
    scale: Scale,
    font: &Font<'_>,
    text: &str,
) {
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font
        .layout(
            text,
            scale,
            rusttype::point(x as f32, y as f32 + v_metrics.ascent),
        )
        .collect();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, gv| {
                let gx = gx as i32 + bounding_box.min.x;
                let gy = gy as i32 + bounding_box.min.y;
                if gx >= 0 && gx < image.width() as i32 && gy >= 0 && gy < image.height() as i32 {
                    let pixel = image.get_pixel_mut(gx as u32, gy as u32);
                    let font_color = Rgba([
                        ((1.0 - gv) * pixel[0] as f32 + gv * color.0[0] as f32) as u8,
                        ((1.0 - gv) * pixel[1] as f32 + gv * color.0[1] as f32) as u8,
                        ((1.0 - gv) * pixel[2] as f32 + gv * color.0[2] as f32) as u8,
                        ((1.0 - gv) * pixel[3] as f32 + gv * color.0[3] as f32) as u8,
                    ]);
                    *pixel = font_color;
                }
            });
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn get_background_images(is_landscape: bool) -> Vec<&'static [u8]> {
    if is_landscape {
        LANDSCAPE_IMAGES.to_vec()
    } else {
        PORTRAIT_IMAGES.to_vec()
    }
}

#[cfg(not(target_arch = "wasm32"))]
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
    let images = get_background_images(is_landscape);

    let chosen_image = images
        .choose(&mut rand::thread_rng())
        .ok_or("Failed to choose a random image")?;

    // Load the image from bytes
    image::load_from_memory(chosen_image)
        .map_err(|e| format!("Failed to load image from memory: {:?}", e))
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

    image::open(chosen_image).map_err(|e| format!("Failed to open background image: {:?}", e))
}

#[cfg(target_arch = "wasm32")]
fn encode_image(image: &DynamicImage) -> Result<Vec<u8>, String> {
    let rgba_image = image.to_rgba8();
    let (width, height) = rgba_image.dimensions();

    let mut png_data = Vec::new();

    PngEncoder::write_image(&mut png_data)
        .encode(rgba_image.as_raw(), width, height, image::ColorType::Rgba8)
        .map_err(|e| format!("Failed to encode PNG: {:?}", e))?;

    Ok(png_data)
}

#[cfg(not(target_arch = "wasm32"))]
fn encode_image(image: &DynamicImage) -> Result<Vec<u8>, String> {
    let mut webp_data = Vec::new();
    let rgba_image = image.to_rgba8();
    let (width, height) = rgba_image.dimensions();

    WebPEncoder::new_lossless(&mut webp_data)
        .encode(&rgba_image, width, height, image::ColorType::Rgba8)
        .map_err(|e| format!("Failed to encode WebP: {:?}", e))?;

    Ok(webp_data)
}

pub fn render_svg_to_image(
    svg_content: &str,
    is_landscape: bool,
    legend_items: &[LegendItem],
) -> Result<Vec<u8>, String> {
    let opt = Options::default();
    let tree =
        Tree::from_str(svg_content, &opt).map_err(|e| format!("Failed to parse SVG: {:?}", e))?;

    let background = load_background_image(is_landscape)?;
    let (bg_width, bg_height) = if is_landscape {
        (1344, 768)
    } else {
        (768, 1344)
    };
    info!("Background image size: {}x{}", bg_width, bg_height);

    // Resize the background image if necessary
    let background =
        background.resize_exact(bg_width, bg_height, image::imageops::FilterType::Lanczos3);

    // Calculate the scaling factor for the SVG
    let svg_size = tree.size();
    let svg_aspect_ratio = svg_size.width() / svg_size.height();

    // Calculate maximum dimensions that will fit within the background
    let max_svg_width = bg_width.min(800);
    let max_svg_height = bg_height.min(800);

    let (svg_width, svg_height) =
        if svg_aspect_ratio > (max_svg_width as f32 / max_svg_height as f32) {
            (
                max_svg_width,
                (max_svg_width as f32 / svg_aspect_ratio).round() as u32,
            )
        } else {
            (
                (max_svg_height as f32 * svg_aspect_ratio).round() as u32,
                max_svg_height,
            )
        };

    let scale_x = svg_width as f32 / svg_size.width();
    let scale_y = svg_height as f32 / svg_size.height();

    info!("Scaled SVG size: {}x{}", svg_width, svg_height);

    let mut pixmap = Pixmap::new(svg_width, svg_height).ok_or("Failed to create Pixmap")?;

    let transform = Transform::from_scale(scale_x, scale_y);
    render(&tree, transform, &mut pixmap.as_mut());

    // Convert Pixmap to image::RgbaImage
    let svg_image =
        ImageBuffer::<Rgba<u8>, _>::from_raw(svg_width, svg_height, pixmap.data().to_vec())
            .ok_or("Failed to create RgbaImage")?;

    // Calculate the legend height based on the number of items
    let items_per_row = 2;
    let rows = (legend_items.len() + items_per_row - 1) / items_per_row;
    let legend_height = rows as u32 * 40 + 20; // 40px per row + 20px padding

    // Create a new image with space for both the SVG and the legend
    let final_width = bg_width;
    let final_height = bg_height;
    let mut final_image = DynamicImage::new_rgba8(final_width, final_height);

    // Copy the background onto the final image
    image::imageops::replace(&mut final_image, &background, 0, 0);

    // Draw the title
    let font = Font::try_from_bytes(include_bytes!("../../assets/Handjet-Regular.ttf")).unwrap();
    draw_title(
        final_image.as_mut_rgba8().unwrap(),
        "MyLife",
        &font,
        is_landscape,
    );

    // Calculate the position to center the SVG image
    let title_height = if is_landscape { 60 } else { 130 }; // Less space for title in landscape mode
    let x = (bg_width - svg_width) / 2;
    let y = title_height + (bg_height - svg_height - legend_height - title_height) / 2;

    info!("Overlay position: ({}, {})", x, y);

    // Overlay the SVG image onto the final image
    image::imageops::overlay(&mut final_image, &svg_image, x.into(), y.into());

    // Render the legend
    let legend_image = render_legend(legend_items, bg_width);

    // Add the legend to the bottom of the image
    let legend_y = bg_height - legend_height;
    image::imageops::overlay(&mut final_image, &legend_image, 0, legend_y.into());

    // Encode to WebP using our new function
    encode_image(&final_image)
}
