// src/utils/wasm_config.rs

#[cfg(target_arch = "wasm32")]
use crate::models::RuntimeConfig;
#[cfg(target_arch = "wasm32")]
use once_cell::sync::Lazy;
#[cfg(target_arch = "wasm32")]
use std::sync::Mutex;

// Define the static variable for WASM only
#[cfg(target_arch = "wasm32")]
pub static NEW_CONFIG: Lazy<Mutex<Option<(String, RuntimeConfig)>>> = Lazy::new(|| Mutex::new(None));