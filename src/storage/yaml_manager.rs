#[cfg(not(target_arch = "wasm32"))]
use crate::models::timeline::MyLifeApp;
use crate::models::timeline::{LifePeriod, Yaml};
#[cfg(not(target_arch = "wasm32"))]
use dioxus::prelude::*;
use uuid::Uuid;

#[cfg(not(target_arch = "wasm32"))]
use rfd::FileDialog;
#[cfg(not(target_arch = "wasm32"))]
use std::{fs, path::Path};

#[cfg(target_arch = "wasm32")]
use {
    js_sys,
    wasm_bindgen::{closure::Closure, JsCast, JsValue},
    wasm_bindgen_futures::JsFuture,
    web_sys::{Blob, File, FileReader, HtmlAnchorElement, HtmlInputElement, Url},
};

#[cfg(target_arch = "wasm32")]
const DEFAULT_YAML: &str = include_str!("../data/default.yaml");

// Common trait for storage operations
pub trait Storage {
    fn save(&self, path: &str, content: &str) -> Result<(), String>;
    fn load(&self, path: &str) -> Result<String, String>;
    fn list_files(&self) -> Result<Vec<String>, String>;
}

// Native storage implementation
#[cfg(not(target_arch = "wasm32"))]
pub struct FileStorage {
    data_folder: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl FileStorage {
    pub fn new(data_folder: String) -> Self {
        Self { data_folder }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Storage for FileStorage {
    fn save(&self, path: &str, content: &str) -> Result<(), String> {
        let file_path = Path::new(&self.data_folder).join(path);
        fs::create_dir_all(file_path.parent().unwrap())
            .map_err(|e| format!("Failed to create directory: {:?}", e))?;
        fs::write(file_path, content).map_err(|e| format!("Failed to write file: {:?}", e))
    }

    fn load(&self, path: &str) -> Result<String, String> {
        let file_path = Path::new(&self.data_folder).join(path);
        fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {:?}", e))
    }

    fn list_files(&self) -> Result<Vec<String>, String> {
        let data_folder = Path::new(&self.data_folder);
        let yamls = fs::read_dir(data_folder)
            .map_err(|e| format!("Failed to read directory: {:?}", e))?
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

// Web storage implementation
#[cfg(target_arch = "wasm32")]
pub struct WebStorage;

#[cfg(target_arch = "wasm32")]
impl WebStorage {
    fn get_storage() -> Result<web_sys::Storage, String> {
        web_sys::window()
            .ok_or_else(|| "Failed to get window".to_string())?
            .local_storage()
            .map_err(|_| "Failed to get localStorage".to_string())?
            .ok_or_else(|| "localStorage not available".to_string())
    }
}

#[cfg(target_arch = "wasm32")]
impl Storage for WebStorage {
    fn save(&self, path: &str, content: &str) -> Result<(), String> {
        Self::get_storage()?
            .set_item(path, content)
            .map_err(|_| "Failed to save to localStorage".to_string())
    }

    fn load(&self, path: &str) -> Result<String, String> {
        Self::get_storage()?
            .get_item(path)
            .map_err(|_| "Failed to load from localStorage".to_string())?
            .ok_or_else(|| "Item not found in localStorage".to_string())
    }

    fn list_files(&self) -> Result<Vec<String>, String> {
        let storage = Self::get_storage()?;
        let keys = js_sys::Object::keys(&storage);
        Ok(keys
            .iter()
            .filter_map(|key| key.as_string())
            .filter(|key| key.ends_with(".yaml") || key.ends_with(".yml"))
            .collect())
    }
}

// YamlManager implementations
pub trait YamlManager {
    fn load_yaml(&self, yaml_file: &str) -> Result<Yaml, String>;
    fn export_yaml(&self, yaml: &Yaml, yaml_file: &str) -> Result<(), String>;
    fn update_yaml(&self, yaml: &Yaml, yaml_file: &str) -> Result<(), String>;
    fn get_available_yamls(&self) -> Result<Vec<String>, String>;
}

fn process_yaml_for_storage(yaml: &Yaml) -> Result<String, String> {
    let mut yaml_to_save = yaml.clone();

    // Remove IDs before saving
    for period in &mut yaml_to_save.life_periods {
        period.id = None;
        for event in &mut period.events {
            event.id = None;
        }
    }

    serde_yaml::to_string(&yaml_to_save).map_err(|e| format!("Failed to serialize YAML: {:?}", e))
}

fn create_default_yaml() -> Yaml {
    Yaml {
        name: "John Doe".to_string(),
        date_of_birth: "2000-01".to_string(),
        life_expectancy: 92,
        life_periods: vec![LifePeriod {
            name: "Childhood".to_string(),
            start: "2000-01".to_string(),
            color: "#5100FF".to_string(),
            events: vec![],
            id: Some(Uuid::nil()),
        }],
        routines: Some(vec![]),
    }
}

pub struct YamlManagerImpl<S: Storage> {
    storage: S,
}

impl<S: Storage> YamlManagerImpl<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

impl<S: Storage> YamlManager for YamlManagerImpl<S> {
    fn load_yaml(&self, yaml_file: &str) -> Result<Yaml, String> {
        let yaml_content = match self.storage.load(yaml_file) {
            Ok(content) => content,
            Err(_) => {
                let default_yaml = create_default_yaml();
                let default_content = serde_yaml::to_string(&default_yaml)
                    .map_err(|e| format!("Failed to serialize default YAML: {:?}", e))?;
                self.storage.save(yaml_file, &default_content)?;
                default_content
            }
        };

        let yaml_result: Result<Yaml, serde_yaml::Error> = serde_yaml::from_str(&yaml_content);

        match yaml_result {
            Ok(mut yaml) => {
                // Generate IDs for life periods and events if they don't exist
                for period in &mut yaml.life_periods {
                    if period.id.is_none() {
                        period.id = Some(Uuid::new_v4());
                    }
                    for event in &mut period.events {
                        if event.id.is_none() {
                            event.id = Some(Uuid::new_v4());
                        }
                    }
                }
                Ok(yaml)
            }
            Err(e) => {
                eprintln!("Error parsing YAML: {:?}", e);
                Ok(create_default_yaml())
            }
        }
    }

    fn export_yaml(&self, yaml: &Yaml, yaml_file: &str) -> Result<(), String> {
        let yaml_content = process_yaml_for_storage(yaml)?;

        #[cfg(target_arch = "wasm32")]
        {
            // Web-specific export implementation
            let blob = Blob::new_with_str_sequence(&js_sys::Array::of1(&yaml_content.into()))
                .map_err(|_| "Failed to create Blob".to_string())?;
            let url = Url::create_object_url_with_blob(&blob)
                .map_err(|_| "Failed to create object URL".to_string())?;

            let window = web_sys::window().ok_or_else(|| "Failed to get window".to_string())?;
            let document = window
                .document()
                .ok_or_else(|| "Failed to get document".to_string())?;
            let anchor: HtmlAnchorElement = document
                .create_element("a")
                .map_err(|_| "Failed to create anchor element".to_string())?
                .dyn_into()
                .map_err(|_| "Failed to cast to HtmlAnchorElement".to_string())?;

            anchor.set_href(&url);
            anchor.set_download(yaml_file);
            anchor.click();

            Url::revoke_object_url(&url).map_err(|_| "Failed to revoke object URL".to_string())?;
            Ok(())
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // Native export implementation
            if let Some(path) = FileDialog::new()
                .set_file_name(yaml_file)
                .add_filter("YAML File", &["yaml", "yml"])
                .save_file()
            {
                fs::write(path, yaml_content)
                    .map_err(|e| format!("Failed to save YAML file: {:?}", e))
            } else {
                Err("File save cancelled".to_string())
            }
        }
    }

    fn update_yaml(&self, yaml: &Yaml, yaml_file: &str) -> Result<(), String> {
        let yaml_content = process_yaml_for_storage(yaml)?;
        self.storage.save(yaml_file, &yaml_content)
    }

    fn get_available_yamls(&self) -> Result<Vec<String>, String> {
        self.storage.list_files()
    }
}

// Factory function to create the appropriate YamlManager
pub fn get_yaml_manager() -> Box<dyn YamlManager> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::new(YamlManagerImpl::new(FileStorage::new("data".to_string())))
    }
    #[cfg(target_arch = "wasm32")]
    {
        Box::new(YamlManagerImpl::new(WebStorage))
    }
}

// Helper functions
pub fn get_yaml() -> Yaml {
    get_yaml_manager()
        .load_yaml("default.yaml")
        .unwrap_or_else(|_| {
            #[cfg(target_arch = "wasm32")]
            {
                get_default_yaml()
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                panic!("Failed to load yaml")
            }
        })
}

pub fn export_yaml(yaml: &Yaml, yaml_file: &str) -> Result<(), String> {
    get_yaml_manager().export_yaml(yaml, yaml_file)
}

pub fn get_available_yamls() -> Vec<String> {
    get_yaml_manager()
        .get_available_yamls()
        .unwrap_or_else(|_| vec![])
}

pub fn update_yaml(yaml: &Yaml, yaml_file: &str) -> Result<(), String> {
    get_yaml_manager().update_yaml(yaml, yaml_file)
}

#[cfg(target_arch = "wasm32")]
pub fn get_default_yaml() -> Yaml {
    serde_yaml::from_str(DEFAULT_YAML).expect("Failed to load default yaml")
}

// Import functionality
#[cfg(target_arch = "wasm32")]
pub async fn import_yaml() -> Option<(String, Yaml)> {
    load_yaml_async().await
}

#[cfg(target_arch = "wasm32")]
pub async fn load_yaml_async() -> Option<(String, Yaml)> {
    let window = web_sys::window()?;
    let document = window.document()?;

    let input: HtmlInputElement = document.create_element("input").ok()?.dyn_into().ok()?;

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
            // Store the loaded YAML in local storage
            if let Some(storage) = window.local_storage().ok().flatten() {
                let _ = storage.set_item(&file_name, &content_string);
            }

            Some((file_name, yaml))
        }
        Err(_e) => None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn import_yaml() -> Option<(String, Yaml)> {
    let app_state = use_context::<Signal<MyLifeApp>>();

    if let Some(file_path) = FileDialog::new()
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
