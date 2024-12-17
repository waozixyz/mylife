// storage/mod.rs
mod config;
mod formats;
mod paths;
mod storage_manager;

pub use config::StorageConfig;
pub use formats::{DataFormat, JsonFormat, YamlFormat};
pub use paths::{get_path_manager, PathManager};
pub use storage_manager::{StorageError, StorageManager, StorageResult};

// Re-export commonly used types
pub type JsonStorage<T> = StorageManager<T, JsonFormat>;
pub type YamlStorage<T> = StorageManager<T, YamlFormat>;
