// storage/config.rs
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub create_dirs: bool,
    pub backup_on_save: bool,
    pub max_backups: usize,
    pub extension: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            create_dirs: true,
            backup_on_save: true,
            max_backups: 5,
            extension: String::from("json"),
        }
    }
}
