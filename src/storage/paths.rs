// storage/paths.rs
use directories::UserDirs;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use tracing::{debug, error, info};

const APP_NAME: &str = "com.example.Myquest";

pub struct PathManager {
    root_dir: PathBuf,
}

impl PathManager {
    pub fn new() -> Self {
        let root_dir = Self::determine_root_dir();
        Self { root_dir }
    }

    fn determine_root_dir() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            let potential_paths = vec![
                format!("/data/data/{}/files", APP_NAME),
                format!("/data/user/0/{}/files", APP_NAME),
            ];
            for path in potential_paths {
                let dir_path = PathBuf::from(&path);
                if let Ok(_) = std::fs::create_dir_all(&dir_path) {
                    return dir_path;
                }
            }
            PathBuf::from(".")
        }
        #[cfg(not(target_os = "android"))]
        {
            if let Some(user_dirs) = UserDirs::new() {
                let documents_dir = user_dirs
                    .document_dir()
                    .unwrap_or_else(|| user_dirs.home_dir())
                    .to_path_buf();
                let myquest_dir = documents_dir.join("myquest");
                if let Err(e) = std::fs::create_dir_all(&myquest_dir) {
                    error!("Failed to create myquest directory: {}", e);
                    return PathBuf::from(".");
                }
                myquest_dir
            } else {
                error!("Could not determine user directories");
                PathBuf::from(".")
            }
        }
    }

    pub fn habits_dir(&self) -> PathBuf {
        let habits_dir = self.root_dir.join("habits");
        if let Err(e) = std::fs::create_dir_all(&habits_dir) {
            error!("Failed to create habits directory: {}", e);
        }
        habits_dir
    }

    pub fn timelines_dir(&self) -> PathBuf {
        let timelines_dir = self.root_dir.join("timelines");
        if let Err(e) = std::fs::create_dir_all(&timelines_dir) {
            error!("Failed to create timelines directory: {}", e);
        }
        timelines_dir
    }

    pub fn habits_file(&self) -> PathBuf {
        self.habits_dir().join("habits.json")
    }

    pub fn todos_file(&self) -> PathBuf {
        self.root_dir.join("todos.json")
    }

    pub fn timeline_file(&self, name: &str) -> PathBuf {
        self.timelines_dir().join(format!("{}.yaml", name))
    }
}

static PATH_MANAGER: Lazy<PathManager> = Lazy::new(PathManager::new);

pub fn get_path_manager() -> &'static PathManager {
    &PATH_MANAGER
}
