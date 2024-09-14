use crate::models::{LifePeriod, MyLifeApp, Yaml};
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

pub trait YamlManager {
    fn load_yaml(&self, yaml_file: &str) -> Result<Yaml, String>;
    fn export_yaml(&self, yaml: &Yaml, yaml_file: &str) -> Result<(), String>;
    fn update_yaml(&self, yaml: &Yaml, yaml_file: &str) -> Result<(), String>;
    fn get_available_yamls(&self) -> Result<Vec<String>, String>;
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
    fn load_yaml(&self, yaml_file: &str) -> Result<Yaml, String> {
        let yaml_path = Path::new(&self.data_folder).join(yaml_file);

        let yaml_content = if yaml_path.exists() {
            fs::read_to_string(&yaml_path)
                .map_err(|e| format!("Failed to read YAML file: {:?}", e))?
        } else {
            // Create default YAML content if file doesn't exist
            let default_yaml = Yaml {
                name: "John Doe".to_string(),
                date_of_birth: "2000-01".to_string(),
                life_expectancy: 92,
                life_periods: vec![
                    LifePeriod {
                        name: "Childhood".to_string(),
                        start: "2000-01".to_string(),
                        color: "#5100FF".to_string(),
                        events: vec![],
                        id: Some(Uuid::nil()),
                    },
                    // Add more default life periods as needed
                ],
                routines: vec![], // Empty routines for now
            };

            let default_content = serde_yaml::to_string(&default_yaml)
                .map_err(|e| format!("Failed to create default YAML: {:?}", e))?;

            // Create directory if it doesn't exist
            if let Some(parent) = yaml_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {:?}", e))?;
            }

            // Write default content to file
            fs::write(&yaml_path, &default_content)
                .map_err(|e| format!("Failed to write default YAML: {:?}", e))?;

            default_content
        };

        let yaml_result: Result<Yaml, serde_yaml::Error> = serde_yaml::from_str(&yaml_content);

        match yaml_result {
            Ok(mut yaml) => {
                // Generate IDs for life periods and events
                for period in &mut yaml.life_periods {
                    period.id = Some(Uuid::new_v4());
                    for event in &mut period.events {
                        event.id = Some(Uuid::new_v4());
                    }
                }
                Ok(yaml)
            }
            Err(e) => {
                eprintln!("Error parsing YAML: {:?}", e);
                // Return a minimal valid YAML if parsing fails
                Ok(Yaml {
                    name: "Default User".to_string(),
                    date_of_birth: "2000-01".to_string(),
                    life_expectancy: 80,
                    life_periods: vec![],
                    routines: vec![],
                })
            }
        }
    }

    fn export_yaml(&self, yaml: &Yaml, yaml_file: &str) -> Result<(), String> {
        let yaml_content = serde_yaml::to_string(yaml)
            .map_err(|e| format!("Failed to serialize YAML: {:?}", e))?;

        if let Some(path) = FileDialog::new()
            .set_file_name(yaml_file)
            .add_filter("YAML File", &["yaml", "yml"])
            .save_file()
        {
            fs::write(path, yaml_content)
                .map_err(|e| format!("Failed to save YAML file: {:?}", e))?;
            Ok(())
        } else {
            Err("File save cancelled".to_string())
        }
    }

    fn update_yaml(&self, yaml: &Yaml, yaml_file: &str) -> Result<(), String> {
        let mut yaml_to_save = yaml.clone();

        // Remove IDs before saving
        for period in &mut yaml_to_save.life_periods {
            period.id = Some(Uuid::nil());
            for event in &mut period.events {
                event.id = Some(Uuid::nil());
            }
        }

        let yaml_content = serde_yaml::to_string(&yaml_to_save)
            .map_err(|e| format!("Failed to serialize YAML: {:?}", e))?;

        let file_path = Path::new(&self.data_folder).join(yaml_file);
        fs::create_dir_all(file_path.parent().unwrap())
            .map_err(|e| format!("Failed to create directory: {:?}", e))?;

        fs::write(file_path, yaml_content)
            .map_err(|e| format!("Failed to update YAML file: {:?}", e))
    }

    fn get_available_yamls(&self) -> Result<Vec<String>, String> {
        let data_folder = Path::new(&self.data_folder);
        let yamls = fs::read_dir(data_folder)
            .map_err(|e| format!("Failed to read data folder: {:?}", e))?
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
    pub fn load_yaml(&self, yaml_file: &str) -> Result<Yaml, String> {
        let yaml_path = Path::new(&self.data_folder).join(yaml_file);

        let yaml_content = if yaml_path.exists() {
            fs::read_to_string(&yaml_path)
                .map_err(|e| format!("Failed to read YAML file: {:?}", e))?
        } else {
            // Create default YAML content if file doesn't exist
            let default_yaml = Yaml {
                name: "John Doe".to_string(),
                date_of_birth: "2000-01".to_string(),
                life_expectancy: 92,
                life_periods: vec![
                    LifePeriod {
                        name: "Childhood".to_string(),
                        start: "2000-01".to_string(),
                        color: "#5100FF".to_string(),
                        events: vec![],
                        id: Uuid::nil(), // This will be replaced later
                    },
                    // Add more default life periods as needed
                ],
                routines: vec![], // Empty routines for now
            };

            let default_content = serde_yaml::to_string(&default_yaml)
                .map_err(|e| format!("Failed to create default YAML: {:?}", e))?;

            // Create directory if it doesn't exist
            if let Some(parent) = yaml_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {:?}", e))?;
            }

            // Write default content to file
            fs::write(&yaml_path, &default_content)
                .map_err(|e| format!("Failed to write default YAML: {:?}", e))?;

            default_content
        };

        let yaml_result: Result<Yaml, serde_yaml::Error> = serde_yaml::from_str(&yaml_content);

        match yaml_result {
            Ok(mut yaml) => {
                // Generate IDs for life periods and events
                for period in &mut yaml.life_periods {
                    period.id = Uuid::new_v4();
                    for event in &mut period.events {
                        event.id = Uuid::new_v4();
                    }
                }
                Ok(yaml)
            }
            Err(e) => {
                eprintln!("Error parsing YAML: {:?}", e);
                // Return a minimal valid YAML if parsing fails
                Ok(Yaml {
                    name: "Default User".to_string(),
                    date_of_birth: "2000-01".to_string(),
                    life_expectancy: 80,
                    life_periods: vec![],
                    routines: vec![],
                })
            }
        }
    }
    fn export_yaml(&self, yaml: &Yaml, _yaml_file: &str) -> Result<(), String> {
        let yaml_content = serde_yaml::to_string(yaml)
            .map_err(|e| format!("Failed to serialize YAML: {:?}", e))?;
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
        anchor.set_download("config.yaml");
        anchor.click();

        Url::revoke_object_url(&url).map_err(|_| "Failed to revoke object URL".to_string())?;

        Ok(())
    }

    fn update_yaml(&self, yaml: &Yaml, yaml_file: &str) -> Result<(), String> {
        let yaml_content = serde_yaml::to_string(yaml)
            .map_err(|e| format!("Failed to serialize YAML: {:?}", e))?;
        let window = web_sys::window().ok_or_else(|| "Failed to get window".to_string())?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage".to_string())?
            .ok_or_else(|| "localStorage not available".to_string())?;

        storage
            .set_item(yaml_file, &yaml_content)
            .map_err(|_| "Failed to set item in localStorage".to_string())
    }

    fn get_available_yamls(&self) -> Result<Vec<String>, String> {
        let window = web_sys::window().ok_or_else(|| "Failed to get window".to_string())?;
        let storage = window
            .local_storage()
            .map_err(|_| "Failed to get localStorage".to_string())?
            .ok_or_else(|| "localStorage not available".to_string())?;

        let keys = js_sys::Object::keys(&storage);
        Ok(keys
            .iter()
            .filter_map(|key| key.as_string())
            .filter(|key| key.ends_with(".yaml") || key.ends_with(".yml"))
            .collect())
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

pub fn get_yaml() -> Yaml {
    get_yaml_manager()
        .load_yaml("default.yaml")
        .expect("Failed to load yaml")
}

pub fn export_yaml(yaml: &Yaml, yaml_file: &str) -> Result<(), String> {
    get_yaml_manager().export_yaml(yaml, yaml_file)
}

pub fn get_available_yamls() -> Vec<String> {
    get_yaml_manager()
        .get_available_yamls()
        .expect("Failed to get available yamls")
}

pub fn update_yaml(yaml: &Yaml, yaml_file: &str) -> Result<(), String> {
    get_yaml_manager().update_yaml(yaml, yaml_file)
}

#[cfg(target_arch = "wasm32")]
pub fn get_default_yaml() -> Yaml {
    serde_yaml::from_str(DEFAULT_YAML).expect("Failed to load default yaml")
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

#[cfg(target_arch = "wasm32")]
pub async fn import_yaml() -> Option<(String, Yaml)> {
    load_yaml_async().await
}
