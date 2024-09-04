use crate::models::RuntimeConfig;

#[cfg(not(target_arch = "wasm32"))]
use crate::config_manager::get_config_manager;

#[cfg(target_arch = "wasm32")]
use crate::config_manager::config_to_runtime_config;

#[cfg(target_arch = "wasm32")]
use manganis::*;
#[cfg(target_arch = "wasm32")]
use crate::models::Config;

#[cfg(target_arch = "wasm32")]
const DEFAULT_CONFIG: &str = include_str!("../../data/default.yaml");

#[cfg(target_arch = "wasm32")]
pub fn get_default_config() -> RuntimeConfig {
    use web_sys::console;
    console::log_1(&"Loading default config".into());
    console::log_1(&format!("DEFAULT_CONFIG: {}", DEFAULT_CONFIG).into());
    
    load_config_from_yaml(DEFAULT_CONFIG)
}

#[cfg(target_arch = "wasm32")]
pub fn load_config_from_yaml(yaml_content: &str) -> RuntimeConfig {
    use web_sys::console;
    console::log_1(&format!("YAML content: {}", yaml_content).into());
    
    let config: Config = match serde_yaml::from_str(yaml_content) {
        Ok(c) => c,
        Err(e) => {
            console::error_1(&format!("Failed to parse YAML: {:?}", e).into());
            panic!("Failed to parse YAML: {:?}", e);
        }
    };
    config_to_runtime_config(config)
}


#[cfg(target_arch = "wasm32")]
pub fn get_available_configs() -> Vec<(String, RuntimeConfig)> {
    use web_sys::console;
    console::log_1(&"Getting available configs".into());
    
    vec![
        ("Default".to_string(), load_config_from_yaml(DEFAULT_CONFIG)),
    ]
}

#[cfg(target_arch = "wasm32")]
pub async fn load_config_async() -> Option<RuntimeConfig> {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("YAML", &["yaml", "yml"])
        .pick_file()
        .await?;

    let content = file.read().await;
    let yaml_content = String::from_utf8(content).ok()?;

    Some(load_config_from_yaml(&yaml_content))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_config(config: &RuntimeConfig, yaml_file: &str) {
    get_config_manager()
        .save_config(config, yaml_file)
        .expect("Failed to save config");
}

#[cfg(target_arch = "wasm32")]
pub fn save_config(config: &RuntimeConfig, _yaml_file: &str) {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, Url, HtmlAnchorElement};

    let yaml_content = serde_yaml::to_string(config).expect("Failed to serialize config");
    
    let blob = Blob::new_with_str_sequence(&js_sys::Array::of1(&yaml_content.into()))
        .expect("Failed to create Blob");
    let url = Url::create_object_url_with_blob(&blob)
        .expect("Failed to create object URL");

    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();
    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .unwrap()
        .dyn_into()
        .unwrap();

    anchor.set_href(&url);
    anchor.set_download("config.yaml");
    anchor.click();

    Url::revoke_object_url(&url).expect("Failed to revoke object URL");
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_available_configs() -> Vec<String> {
    get_config_manager()
        .get_available_configs()
        .expect("Failed to get available configs")
}

pub fn get_config() -> RuntimeConfig {
    #[cfg(not(target_arch = "wasm32"))]
    {
        get_config_manager()
            .load_config("default.yaml")
            .expect("Failed to load config")
    }
    #[cfg(target_arch = "wasm32")]
    {
        get_default_config()
    }
}