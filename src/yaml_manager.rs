use crate::models::Yaml;
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

use dioxus::prelude::*;
use crate::models::MyLifeApp;

#[cfg(target_arch = "wasm32")]
const DEFAULT_YAML: &str = include_str!("../data/default.yaml");

pub trait YamlManager {
    fn load_yaml(&self, yaml_file: &str) -> io::Result<Yaml>;
    fn save_yaml(&self, yaml: &Yaml, yaml_file: &str) -> io::Result<()>;
    fn get_available_yamls(&self) -> io::Result<Vec<String>>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeYamlManager {
    data_folder: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeYamlManager {
    pub fn new(data_folder: String) -> Self {
        Self { data_folder }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl YamlManager for NativeYamlManager {
    fn load_yaml(&self, yaml_file: &str) -> io::Result<Yaml> {
        let yaml_path = std::path::Path::new(&self.data_folder).join(yaml_file);
        let yaml_content = std::fs::read_to_string(yaml_path)?;
        let yaml: Yaml = serde_yaml::from_str(&yaml_content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(yaml)
    }

    fn save_yaml(&self, yaml: &Yaml, yaml_file: &str) -> io::Result<()> {
        let yaml_content = serde_yaml::to_string(&yaml)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let yaml_path = std::path::Path::new(&self.data_folder).join(yaml_file);
        std::fs::write(yaml_path, yaml_content)
    }

    fn get_available_yamls(&self) -> io::Result<Vec<String>> {
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
pub struct WasmYamlManager;

#[cfg(target_arch = "wasm32")]
impl YamlManager for WasmYamlManager {
    fn load_yaml(&self, _yaml_file: &str) -> io::Result<Yaml> {
        Ok(get_default_yaml())
    }

    fn save_yaml(&self, yaml: &Yaml, _yaml_file: &str) -> io::Result<()> {
        let yaml_content = serde_yaml::to_string(&yaml)
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

    fn get_available_yamls(&self) -> io::Result<Vec<String>> {
        Ok(vec!["Default".to_string()])
    }
}

pub fn get_yaml_manager() -> Box<dyn YamlManager> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::new(NativeYamlManager::new("data".to_string()))
    }
    #[cfg(target_arch = "wasm32")]
    {
        Box::new(WasmYamlManager)
    }
}

#[cfg(target_arch = "wasm32")]
pub fn get_default_yaml() -> Yaml {
    load_yaml_from_yaml_content(DEFAULT_YAML).expect("Failed to load default yaml")
}

#[cfg(target_arch = "wasm32")]
pub fn load_yaml_from_yaml_content(yaml_content: &str) -> Result<Yaml, String> {
    serde_yaml::from_str(yaml_content)
        .map_err(|e| format!("Failed to parse YAML: {:?}", e))
}

#[cfg(target_arch = "wasm32")]
pub async fn load_yaml_async() -> Option<(String, Yaml)> {
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

    match load_yaml_from_yaml_content(&content) {
        Ok(yaml) => Some((file_name, yaml)),
        Err(e) => {
            log::error!("Failed to load yaml from YAML: {:?}", e);
            None
        }
    }
}

pub fn get_yaml() -> Yaml {
    get_yaml_manager()
        .load_yaml("default.yaml")
        .expect("Failed to load yaml")
}

pub fn save_yaml(yaml: &Yaml, yaml_file: &str) -> Result<(), String> {
    get_yaml_manager()
        .save_yaml(yaml, yaml_file)
        .map_err(|e| format!("Failed to save yaml: {:?}", e))
}

pub fn get_available_yamls() -> Vec<String> {
    get_yaml_manager()
        .get_available_yamls()
        .expect("Failed to get available yamls")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn import_yaml() -> Option<(String, Yaml)> {
    let app_state = use_context::<Signal<MyLifeApp>>();

    if let Some(file_path) = rfd::FileDialog::new()
        .add_filter("YAML", &["yaml", "yml"])
        .pick_file()
    {
        let file_name = file_path.file_name()?.to_str()?.to_string();
        let content = fs::read_to_string(&file_path).ok()?;
        let yaml: Yaml = serde_yaml::from_str(&content).ok()?;
        
        let data_folder_string = app_state.read().data_folder.clone();
        let data_folder = Path::new(&data_folder_string);
        
        let mut new_file_name = file_name.clone();
        let mut counter = 1;
        
        while data_folder.join(&new_file_name).exists() {
            new_file_name = format!("{}-{}.yaml", file_name.trim_end_matches(".yaml"), counter);
            counter += 1;
        }
        
        fs::copy(file_path, data_folder.join(&new_file_name)).ok()?;
        
        Some((new_file_name, yaml))
    } else {
        None
    }
}