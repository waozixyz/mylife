use crate::models::Yaml;
#[cfg(not(target_arch = "wasm32"))]
use crate::models::MyLifeApp;
#[cfg(not(target_arch = "wasm32"))]
use dioxus::prelude::*;
use std::io;

#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;


#[cfg(target_arch = "wasm32")]
use js_sys;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys::{Blob, FileReader, HtmlAnchorElement, Url, File, HtmlInputElement};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsValue, JsCast};

#[cfg(target_arch = "wasm32")]
const DEFAULT_YAML: &str = include_str!("../data/default.yaml");

use dioxus_logger::tracing::{info, warn, error};


pub trait YamlManager {
    fn load_yaml(&self, yaml_file: &str) -> io::Result<Yaml>;
    #[cfg(target_arch = "wasm32")]
    fn save_yaml(&self, yaml: &Yaml, yaml_file: &str) -> io::Result<()>;
    fn get_available_yamls(&self) -> io::Result<Vec<String>>;

    #[cfg(target_arch = "wasm32")]
    fn update_yaml_in_storage(&self, yaml: &Yaml, yaml_file: &str) -> io::Result<()>;
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
        let yaml_path = Path::new(&self.data_folder).join(yaml_file);
        let yaml_content = fs::read_to_string(yaml_path)?;
        serde_yaml::from_str(&yaml_content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }


    fn get_available_yamls(&self) -> io::Result<Vec<String>> {
        let data_folder = Path::new(&self.data_folder);
        let yamls = fs::read_dir(data_folder)?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                        path.file_name().and_then(|s| s.to_str()).map(String::from)
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<String>>();
        Ok(yamls)
    }

}

#[cfg(target_arch = "wasm32")]
pub struct WasmYamlManager;

#[cfg(target_arch = "wasm32")]
impl YamlManager for WasmYamlManager {
    fn load_yaml(&self, _yaml_file: &str) -> io::Result<Yaml> {
        let window = web_sys::window()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get window"))?;
        let storage = window
            .local_storage()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get localStorage"))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "localStorage not available"))?;

        if let Ok(Some(yaml_content)) = storage.get_item("current_yaml") {
            serde_yaml::from_str(&yaml_content)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        } else {
            Ok(get_default_yaml())
        }
    }

    fn save_yaml(&self, yaml: &Yaml, _yaml_file: &str) -> io::Result<()> {
        let yaml_content = serde_yaml::to_string(yaml)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let blob = Blob::new_with_str_sequence(&js_sys::Array::of1(&yaml_content.into()))
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create Blob"))?;
        let url = Url::create_object_url_with_blob(&blob)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create object URL"))?;

        let window = web_sys::window()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get window"))?;
        let document = window
            .document()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get document"))?;
        let anchor: HtmlAnchorElement = document
            .create_element("a")
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to create anchor element"))?
            .dyn_into()
            .map_err(|_| {
                io::Error::new(io::ErrorKind::Other, "Failed to cast to HtmlAnchorElement")
            })?;

        anchor.set_href(&url);
        anchor.set_download("config.yaml");
        anchor.click();

        Url::revoke_object_url(&url)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to revoke object URL"))?;

        Ok(())
    }
    fn get_available_yamls(&self) -> io::Result<Vec<String>> {
        let window = web_sys::window()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get window"))?;
        let storage = window
            .local_storage()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get localStorage"))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "localStorage not available"))?;

        let keys = js_sys::Object::keys(&storage);
        let yamls: Vec<String> = keys
            .iter()
            .filter_map(|key| key.as_string())
            .filter(|key| key.ends_with(".yaml") || key.ends_with(".yml"))
            .collect();

        Ok(yamls)
    }

    fn update_yaml_in_storage(&self, yaml: &Yaml, yaml_file: &str) -> io::Result<()> {
        let yaml_content = serde_yaml::to_string(yaml)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let window = web_sys::window()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get window"))?;
        let storage = window
            .local_storage()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get localStorage"))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "localStorage not available"))?;

        storage
            .set_item(yaml_file, &yaml_content)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to set item in localStorage"))?;

        Ok(())
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
    serde_yaml::from_str(DEFAULT_YAML).expect("Failed to load default yaml")
}

#[cfg(target_arch = "wasm32")]
pub async fn load_yaml_async() -> Option<(String, Yaml)> {
    info!("Starting load_yaml_async");
    let window = web_sys::window()?;
    let document = window.document()?;

    let input: HtmlInputElement = document
        .create_element("input")
        .ok()?
        .dyn_into()
        .ok()?;

    input.set_type("file");
    input.set_accept(".yaml,.yml");

    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let on_change = Closure::once(Box::new(move |_event: web_sys::Event| {
            resolve.call0(&JsValue::NULL).unwrap();
        }));
        input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
        on_change.forget();
    });

    input.click();
    JsFuture::from(promise).await.ok()?;

    let file: File = input.files()?.get(0)?;
    let file_name = file.name();

    let reader = FileReader::new().ok()?;
    
    let reader_promise = js_sys::Promise::new(&mut |resolve, reject| {
        let onload = Closure::once(Box::new(move |_event: web_sys::Event| {
            resolve.call0(&JsValue::NULL).unwrap();
        }));
        let onerror = Closure::once(Box::new(move |_event: web_sys::Event| {
            reject.call0(&JsValue::NULL).unwrap();
        }));
        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onload.forget();
        onerror.forget();
    });

    reader.read_as_text(&file).ok()?;
    JsFuture::from(reader_promise).await.ok()?;

    let content = reader.result().ok()?;
    let content_string = content.as_string()?;

    match serde_yaml::from_str(&content_string) {
        Ok(yaml) => {
            info!("YAML parsed successfully");
            
            // Store the loaded YAML in local storage
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Err(e) = storage.set_item(&file_name, &content_string) {
                    warn!("Failed to store YAML in local storage: {:?}", e);
                } else {
                    info!("YAML stored in local storage");
                }
            }

            Some((file_name, yaml))
        }
        Err(e) => {
            error!("Failed to parse YAML: {:?}", e);
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
    #[cfg(not(target_arch = "wasm32"))]
    {
        let yaml_content = serde_yaml::to_string(yaml)
            .map_err(|e| format!("Failed to serialize YAML: {:?}", e))?;

        let file_path = FileDialog::new()
            .set_file_name(yaml_file)
            .add_filter("YAML File", &["yaml", "yml"])
            .save_file();

        if let Some(path) = file_path {
            std::fs::write(path, yaml_content)
                .map_err(|e| format!("Failed to save YAML file: {:?}", e))?;
            Ok(())
        } else {
            Err("File save cancelled".to_string())
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        get_yaml_manager()
            .save_yaml(yaml, yaml_file)
            .map_err(|e| format!("Failed to save yaml: {:?}", e))
    }
}
pub fn get_available_yamls() -> Vec<String> {
    get_yaml_manager()
        .get_available_yamls()
        .expect("Failed to get available yamls")
}

pub fn update_yaml(yaml: &Yaml, yaml_file: &str) -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        save_yaml(yaml, yaml_file)
    }

    #[cfg(target_arch = "wasm32")]
    {
        get_yaml_manager()
            .update_yaml_in_storage(yaml, yaml_file)
            .map_err(|e| format!("Failed to update yaml in storage: {:?}", e))
    }
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

#[cfg(target_arch = "wasm32")]
pub async fn import_yaml() -> Option<(String, Yaml)> {
    load_yaml_async().await
}