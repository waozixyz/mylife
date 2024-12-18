use super::{DataFormat, StorageConfig};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs as async_fs;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Lock acquisition failed")]
    LockError,

    #[error("Failed to initialize directory: {0}")]
    InitError(String),
}

pub type StorageResult<T> = Result<T, StorageError>;

pub struct StorageManager<T, F: DataFormat> {
    file_path: PathBuf,
    data: Arc<RwLock<T>>,
    _format: PhantomData<F>,
    config: StorageConfig,
}

impl<T, F> StorageManager<T, F>
where
    T: Default + Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
    F: DataFormat,
{
    pub fn new(file_path: PathBuf) -> StorageResult<Self> {
        Self::with_config_and_default(file_path, StorageConfig::default(), None)
    }

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn with_config(file_path: PathBuf, config: StorageConfig) -> StorageResult<Self> {
        Self::with_config_and_default(file_path, config, None)
    }

    pub fn with_config_and_default(
        file_path: PathBuf,
        config: StorageConfig,
        default_data: Option<T>,
    ) -> StorageResult<Self> {
        debug!("Initializing StorageManager for path: {:?}", file_path);

        if config.create_dirs {
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    StorageError::InitError(format!("Failed to create directory structure: {}", e))
                })?;
            }
        }

        let data = if file_path.exists() {
            debug!("Loading existing data file");
            let content = std::fs::read_to_string(&file_path)?;
            match F::deserialize(&content) {
                Ok(data) => {
                    info!("Successfully loaded data from file");
                    data
                }
                Err(e) => {
                    error!("Failed to parse file, using default data: {}", e);
                    default_data.unwrap_or_default()
                }
            }
        } else {
            debug!("No existing file found, using default data");
            let data = default_data.unwrap_or_default();
            if config.create_dirs {
                let content = F::serialize(&data).map_err(|e| {
                    error!("Failed to serialize default data: {}", e);
                    StorageError::Serialization(e.to_string())
                })?;

                std::fs::write(&file_path, &content).map_err(|e| {
                    error!("Failed to write default data to file: {}", e);
                    StorageError::Io(e)
                })?;

                debug!("Successfully wrote default data to file: {:?}", file_path);
            }
            data
        };

        Ok(Self {
            file_path,
            data: Arc::new(RwLock::new(data)),
            _format: PhantomData,
            config,
        })
    }

    pub async fn read<R, Func>(&self, f: Func) -> StorageResult<R>
    where
        Func: FnOnce(&T) -> R,
    {
        let guard = self.data.read().await;
        Ok(f(&guard))
    }

    pub async fn write<R, Func>(&self, f: Func) -> StorageResult<R>
    where
        Func: FnOnce(&mut T) -> R,
    {
        let mut guard = self.data.write().await;
        let result = f(&mut guard);

        let data_clone = (*guard).clone();
        let file_path = self.file_path.clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::save_to_disk(&file_path, &data_clone, &config).await {
                error!("Failed to save data: {}", e);
            }
        });

        Ok(result)
    }

    pub async fn get_data(&self) -> StorageResult<T> {
        let guard = self.data.read().await;
        Ok((*guard).clone())
    }

    async fn save_to_disk(
        file_path: &PathBuf,
        data: &T,
        config: &StorageConfig,
    ) -> StorageResult<()> {
        if config.backup_on_save && file_path.exists() {
            Self::create_backup(file_path, config.max_backups).await?;
        }

        let content = F::serialize(data).map_err(StorageError::Serialization)?;
        async_fs::write(file_path, content).await?;
        debug!("Successfully saved data to disk");

        Ok(())
    }

    async fn create_backup(file_path: &PathBuf, max_backups: usize) -> StorageResult<()> {
        let backup_path = file_path.with_extension("backup");

        for i in (1..max_backups).rev() {
            let current = backup_path.with_extension(format!("backup{}", i));
            let next = backup_path.with_extension(format!("backup{}", i + 1));
            if current.exists() {
                async_fs::rename(&current, &next).await?;
            }
        }

        if file_path.exists() {
            async_fs::copy(file_path, backup_path.with_extension("backup1")).await?;
            debug!("Created backup of data file");
        }

        Ok(())
    }

    pub async fn force_save(&self) -> StorageResult<()> {
        let data = self.data.read().await;
        Self::save_to_disk(&self.file_path, &data, &self.config).await
    }

    pub async fn reload(&self) -> StorageResult<()> {
        let content = async_fs::read_to_string(&self.file_path).await?;
        let new_data = F::deserialize(&content).map_err(StorageError::Serialization)?;
        let mut guard = self.data.write().await;
        *guard = new_data;
        Ok(())
    }
}
