use crate::models::Config;
use std::io;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{Blob, HtmlAnchorElement, Url, FileReader};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use js_sys;
#[cfg(target_arch = "wasm32")]
use rfd;

#[cfg(target_arch = "wasm32")]
const DEFAULT_CONFIG: &str = include_str!("../data/default.yaml");

pub trait ConfigManager {
    fn load_config(&self, yaml_file: &str) -> io::Result<Config>;
    fn save_config(&self, config: &Config, yaml_file: &str) -> io::Result<()>;
    fn get_available_configs(&self) -> io::Result<Vec<String>>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeConfigManager {
    data_folder: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeConfigManager {
    pub fn new(data_folder: String) -> Self {
        Self { data_folder }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ConfigManager for NativeConfigManager {
    fn load_config(&self, yaml_file: &str) -> io::Result<Config> {
        let yaml_path = std::path::Path::new(&self.data_folder).join(yaml_file);
        let yaml_content = std::fs::read_to_string(yaml_path)?;
        let config: Config = serde_yaml::from_str(&yaml_content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }

    fn save_config(&self, config: &Config, yaml_file: &str) -> io::Result<()> {
        let yaml_content = serde_yaml::to_string(&config)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let yaml_path = std::path::Path::new(&self.data_folder).join(yaml_file);
        std::fs::write(yaml_path, yaml_content)
    }

    fn get_available_configs(&self) -> io::Result<Vec<String>> {
        let data_folder = Path::new(&self.data_folder);
        Ok(fs::read_dir(data_folder)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()? == "yaml" {
                    Some(path.file_name()?.to_string_lossy().into_owned())
                } else {
                    None
                }
            })
            .collect())
    }
}

#[cfg(target_arch = "wasm32")]
pub struct WasmConfigManager;

#[cfg(target_arch = "wasm32")]
impl ConfigManager for WasmConfigManager {
    fn load_config(&self, _yaml_file: &str) -> io::Result<Config> {
        // For WASM, we'll always return the default config
        Ok(get_default_config())
    }

    fn save_config(&self, config: &Config, _yaml_file: &str) -> io::Result<()> {
        let yaml_content = serde_yaml::to_string(&config)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let blob = Blob::new_with_str_sequence(&js_sys::Array::of1(&yaml_content.into()))
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create Blob"))?;
        let url = Url::create_object_url_with_blob(&blob)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create object URL"))?;

        let window = web_sys::window()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get window"))?;
        let document = window.document()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get document"))?;
        let anchor: HtmlAnchorElement = document
            .create_element("a")
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create anchor element"))?
            .dyn_into()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to cast to HtmlAnchorElement"))?;

        anchor.set_href(&url);
        anchor.set_download("config.yaml");
        anchor.click();

        Url::revoke_object_url(&url)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to revoke object URL"))?;

        Ok(())
    }

    fn get_available_configs(&self) -> io::Result<Vec<String>> {
        // For WASM, we'll always return a single default config
        Ok(vec!["Default".to_string()])
    }
}

pub fn get_config_manager() -> Box<dyn ConfigManager> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::new(NativeConfigManager::new("data".to_string()))
    }
    #[cfg(target_arch = "wasm32")]
    {
        Box::new(WasmConfigManager)
    }
}

#[cfg(target_arch = "wasm32")]
pub fn get_default_config() -> Config {
    load_config_from_yaml(DEFAULT_CONFIG).expect("Failed to load default config")
}


#[cfg(target_arch = "wasm32")]
pub fn load_config_from_yaml(yaml_content: &str) -> Result<Config, String> {
    serde_yaml::from_str(yaml_content)
        .map_err(|e| format!("Failed to parse YAML: {:?}", e))
}

#[cfg(target_arch = "wasm32")]
pub async fn load_config_async() -> Option<(String, Config)> {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("YAML", &["yaml", "yml"])
        .pick_file()
        .await?;

    let file_name = file.file_name();
    let file_content = file.read().await;

    let blob = Blob::new_with_u8_array_sequence(&js_sys::Array::of1(&js_sys::Uint8Array::from(&file_content[..])))
        .map_err(|_| "Failed to create Blob".to_string()).ok()?;

    let reader = FileReader::new().unwrap();
    reader.read_as_text(&blob).unwrap();

    let promise = js_sys::Promise::resolve(&reader.result().unwrap());
    let content = JsFuture::from(promise).await.ok()?.as_string().unwrap();

    match load_config_from_yaml(&content) {
        Ok(config) => Some((file_name, config)),
        Err(e) => {
            log::error!("Failed to load config from YAML: {:?}", e);
            None
        }
    }
}

pub fn get_config() -> Config {
    get_config_manager()
        .load_config("default.yaml")
        .expect("Failed to load config")
}

pub fn save_config(config: &Config, yaml_file: &str) -> Result<(), String> {
    get_config_manager()
        .save_config(config, yaml_file)
        .map_err(|e| format!("Failed to save config: {:?}", e))
}

pub fn get_available_configs() -> Vec<String> {
    get_config_manager()
        .get_available_configs()
        .expect("Failed to get available configs")
}