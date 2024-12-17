// storage/formats.rs
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

pub trait DataFormat {
    fn serialize<T: Serialize>(data: &T) -> Result<String, String>;
    fn deserialize<T: DeserializeOwned>(content: &str) -> Result<T, String>;
    fn default_extension() -> &'static str;
}

pub struct JsonFormat;
impl DataFormat for JsonFormat {
    fn serialize<T: Serialize>(data: &T) -> Result<String, String> {
        serde_json::to_string_pretty(data).map_err(|e| e.to_string())
    }

    fn deserialize<T: DeserializeOwned>(content: &str) -> Result<T, String> {
        serde_json::from_str(content).map_err(|e| e.to_string())
    }

    fn default_extension() -> &'static str {
        "json"
    }
}

pub struct YamlFormat;
impl DataFormat for YamlFormat {
    fn serialize<T: Serialize>(data: &T) -> Result<String, String> {
        serde_yaml::to_string(data).map_err(|e| e.to_string())
    }

    fn deserialize<T: DeserializeOwned>(content: &str) -> Result<T, String> {
        serde_yaml::from_str(content).map_err(|e| e.to_string())
    }

    fn default_extension() -> &'static str {
        "yaml"
    }
}
