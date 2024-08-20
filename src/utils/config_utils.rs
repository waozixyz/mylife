#[cfg(target_arch = "wasm32")]
use crate::config_manager::config_to_runtime_config;
#[cfg(not(target_arch = "wasm32"))]
use crate::config_manager::get_config_manager;
#[cfg(target_arch = "wasm32")]
use crate::models::Config;
use crate::models::RuntimeConfig;

#[cfg(not(target_arch = "wasm32"))]
pub fn get_config() -> RuntimeConfig {
    get_config_manager()
        .load_config("default.yaml")
        .expect("Failed to load config")
}

#[cfg(target_arch = "wasm32")]
pub fn get_default_config() -> RuntimeConfig {
    let config = Config::default();
    config_to_runtime_config(config)
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
    let _yaml_content = serde_yaml::to_string(config).expect("Failed to serialize config");
    // Implement logic to trigger download of yaml_content
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_available_configs() -> Vec<String> {
    get_config_manager()
        .get_available_configs()
        .expect("Failed to get available configs")
}
