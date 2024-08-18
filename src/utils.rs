use std::fs;
use std::path::Path;
use crate::config::Config;
use eframe::egui;

pub fn hex_to_color32(hex: &str) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    egui::Color32::from_rgb(r, g, b)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_yaml_files_in_data_folder() -> Vec<String> {
    let data_folder = Path::new("data");
    fs::read_dir(data_folder)
        .expect("Failed to read data folder")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "yaml" {
                Some(path.file_name()?.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_config(yaml_file: &str) -> Config {
    let yaml_path = Path::new("data").join(yaml_file);
    let yaml_content = fs::read_to_string(yaml_path)
        .unwrap_or_else(|_| panic!("Failed to read YAML file: {}", yaml_file));
    serde_yaml::from_str(&yaml_content)
        .unwrap_or_else(|_| panic!("Failed to parse YAML file: {}", yaml_file))
}

#[cfg(target_arch = "wasm32")]
pub fn load_config(yaml_content: &str) -> Config {
    serde_yaml::from_str(yaml_content).unwrap_or_else(|e| {
        log::error!("Failed to parse YAML content: {}. Using default config.", e);
        Config::default()
    })
}
