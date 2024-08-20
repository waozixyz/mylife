#[cfg(target_arch = "wasm32")]
use crate::config_manager::config_to_runtime_config;
#[cfg(not(target_arch = "wasm32"))]
use crate::config_manager::get_config_manager;
#[cfg(target_arch = "wasm32")]
use crate::models::Config;
use crate::models::RuntimeConfig;

use chrono::NaiveDate;
use eframe::egui;

pub fn hex_to_color32(hex: &str) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    egui::Color32::from_rgb(r, g, b)
}

pub fn calculate_centered_rect(available: egui::Rect, desired_size: egui::Vec2) -> egui::Rect {
    let size = egui::Vec2::new(
        desired_size.x.min(available.width()),
        desired_size.y.min(available.height()),
    );
    let pos = available.center() - (size / 2.0);
    egui::Rect::from_min_size(pos, size)
}

pub fn is_valid_date(date_str: &str) -> bool {
    NaiveDate::parse_from_str(&format!("{}-01", date_str), "%Y-%m-%d").is_ok()
}

pub fn color32_to_hex(color: egui::Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_config() -> RuntimeConfig {
    get_config_manager()
        .load_config("default.yaml")
        .expect("Failed to load config")
}

#[cfg(target_arch = "wasm32")]
pub fn get_default_config() -> RuntimeConfig {
    let config = Config::default();
    let runtime_config = config_to_runtime_config(config);
    runtime_config
}

#[cfg(target_arch = "wasm32")]
pub async fn load_config_async() -> Option<RuntimeConfig> {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("YAML", &["yaml", "yml"])
        .pick_file()
        .await?;

    let content = file.read().await;
    let yaml_content = String::from_utf8(content).ok()?;

    let config: Config = serde_yaml::from_str(&yaml_content).ok()?;
    Some(config_to_runtime_config(config))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_config(config: &RuntimeConfig, yaml_file: &str) {
    get_config_manager()
        .save_config(config, yaml_file)
        .expect("Failed to save config");
}

#[cfg(target_arch = "wasm32")]
pub fn save_config(config: &RuntimeConfig, _yaml_file: &str) {
    // Implement WASM-specific save logic here
    // For example, you might want to trigger a download of the YAML file
    let _yaml_content = serde_yaml::to_string(config).expect("Failed to serialize config");
    // Implement logic to trigger download of yaml_content
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_available_configs() -> Vec<String> {
    get_config_manager()
        .get_available_configs()
        .expect("Failed to get available configs")
}
