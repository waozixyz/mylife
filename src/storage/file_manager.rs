use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs as async_fs;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Error type for FileManager operations
#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Lock acquisition failed")]
    LockError,

    #[error("Failed to initialize directory: {0}")]
    InitError(String),
}

/// Result type for FileManager operations
pub type FileResult<T> = Result<T, FileError>;

/// Configuration for FileManager
#[derive(Debug, Clone)]
pub struct FileManagerConfig {
    pub create_dirs: bool,
    pub backup_on_save: bool,
    pub max_backups: usize,
}

impl Default for FileManagerConfig {
    fn default() -> Self {
        Self {
            create_dirs: true,
            backup_on_save: true,
            max_backups: 5,
        }
    }
}

/// A thread-safe file manager for handling persistent data storage
pub struct FileManager<T> {
    /// Path to the data file
    file_path: PathBuf,
    /// In-memory data storage
    data: Arc<RwLock<T>>,
    /// Configuration options
    config: FileManagerConfig,
}

impl<T> FileManager<T>
where
    T: Default + Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    /// Creates a new FileManager instance synchronously
    pub fn new(file_path: PathBuf) -> FileResult<Self> {
        Self::with_config(file_path, FileManagerConfig::default())
    }

    /// Creates a new FileManager instance with custom configuration synchronously
    pub fn with_config(file_path: PathBuf, config: FileManagerConfig) -> FileResult<Self> {
        debug!("Initializing FileManager for path: {:?}", file_path);

        // Ensure parent directories exist if configured
        if config.create_dirs {
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    FileError::InitError(format!("Failed to create directory structure: {}", e))
                })?;
            }
        }

        // Initialize data from file or default
        let data = if file_path.exists() {
            debug!("Loading existing data file");
            let content = std::fs::read_to_string(&file_path)?;
            match serde_json::from_str(&content) {
                Ok(data) => {
                    info!("Successfully loaded data from file");
                    data
                }
                Err(e) => {
                    warn!("Failed to parse file, using default data: {}", e);
                    T::default()
                }
            }
        } else {
            debug!("No existing file found, using default data");
            T::default()
        };

        Ok(Self {
            file_path,
            data: Arc::new(RwLock::new(data)),
            config,
        })
    }

    /// Reads data using the provided closure
    pub async fn read<F, R>(&self, f: F) -> FileResult<R>
    where
        F: FnOnce(&T) -> R,
    {
        let guard = self.data.read().await;
        Ok(f(&guard))
    }

    /// Writes data using the provided closure and persists changes
    pub async fn write<F, R>(&self, f: F) -> FileResult<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.data.write().await;
        let result = f(&mut guard);

        // Clone data for async saving
        let data_clone = (*guard).clone();
        let file_path = self.file_path.clone();
        let config = self.config.clone();

        // Spawn async task to save data
        tokio::spawn(async move {
            if let Err(e) = Self::save_to_disk(&file_path, &data_clone, &config).await {
                error!("Failed to save data: {}", e);
            }
        });

        Ok(result)
    }

    /// Gets a clone of the current data
    pub async fn get_data(&self) -> FileResult<T> {
        let guard = self.data.read().await;
        Ok((*guard).clone())
    }

    /// Saves the current state to disk
    async fn save_to_disk(
        file_path: &PathBuf,
        data: &T,
        config: &FileManagerConfig,
    ) -> FileResult<()> {
        // Create backup if configured
        if config.backup_on_save && file_path.exists() {
            Self::create_backup(file_path, config.max_backups).await?;
        }

        // Serialize and save data
        let content = serde_json::to_string_pretty(data)?;
        async_fs::write(file_path, content).await?;
        debug!("Successfully saved data to disk");

        Ok(())
    }

    /// Creates a backup of the current file
    async fn create_backup(file_path: &PathBuf, max_backups: usize) -> FileResult<()> {
        let backup_path = file_path.with_extension("backup");

        // Rotate existing backups
        for i in (1..max_backups).rev() {
            let current = backup_path.with_extension(format!("backup{}", i));
            let next = backup_path.with_extension(format!("backup{}", i + 1));
            if current.exists() {
                async_fs::rename(&current, &next).await?;
            }
        }

        // Create new backup
        if file_path.exists() {
            async_fs::copy(file_path, backup_path.with_extension("backup1")).await?;
            debug!("Created backup of data file");
        }

        Ok(())
    }

    /// Forces an immediate save to disk
    pub async fn force_save(&self) -> FileResult<()> {
        let data = self.data.read().await;
        Self::save_to_disk(&self.file_path, &data, &self.config).await
    }

    /// Reloads data from disk, discarding any unsaved changes
    pub async fn reload(&self) -> FileResult<()> {
        let content = async_fs::read_to_string(&self.file_path).await?;
        let new_data: T = serde_json::from_str(&content)?;
        let mut guard = self.data.write().await;
        *guard = new_data;
        Ok(())
    }
}
