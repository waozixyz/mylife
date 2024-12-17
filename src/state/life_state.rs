use crate::models::timeline::{MyLifeApp, Yaml};
#[cfg(target_arch = "wasm32")]
use crate::utils::compression::decode_and_decompress;
use crate::managers::timeline_manager::get_timeline_manager;
use dioxus::prelude::*;
use tracing::{debug, error};

pub async fn initialize_state(y: &str) -> (Yaml, MyLifeApp) {
    // First try to get the initial YAML state
    let yaml_state = if !y.is_empty() {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(decompressed_str) = decode_and_decompress(y) {
                match serde_yaml::from_str::<Yaml>(&decompressed_str) {
                    Ok(new_yaml) => {
                        debug!("Successfully parsed shared YAML");
                        new_yaml
                    }
                    Err(e) => {
                        error!("Failed to parse YAML: {}", e);
                        let context = decompressed_str
                            .lines()
                            .take(5)
                            .collect::<Vec<_>>()
                            .join("\n");
                        error!("YAML parsing error context:\n{}", context);
                        get_default_timeline().await
                    }
                }
            } else {
                error!("Failed to decompress YAML");
                get_default_timeline().await
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        get_default_timeline().await

    } else {
        debug!("No shared YAML, loading from timeline manager");
        get_default_timeline().await
    };

    // Initialize the app state
    let mut app_state = initialize_app_state();
    
    // If we're using a shared timeline, update the selection
    if !y.is_empty() {
        app_state.selected_yaml = "Shared Timeline".to_string();
        
        // Save the shared timeline
        if let Err(e) = get_timeline_manager().update_timeline(&yaml_state).await {
            error!("Failed to save shared timeline: {}", e);
        }
    }

    // Update the loaded yamls with the current state
    app_state.loaded_yamls = vec![("default".to_string(), yaml_state.clone())];

    (yaml_state, app_state)
}

async fn get_default_timeline() -> Yaml {
    match get_timeline_manager().get_timeline().await {
        Ok(yaml) => {
            debug!("Successfully loaded timeline from manager");
            yaml
        }
        Err(e) => {
            error!("Failed to load timeline from manager: {}", e);
            // Create a default timeline
            let default_yaml = Yaml::default();
            
            // Try to save it
            if let Err(save_err) = get_timeline_manager().update_timeline(&default_yaml).await {
                error!("Failed to save default timeline: {}", save_err);
            } else {
                debug!("Saved default timeline successfully");
            }
            
            default_yaml
        }
    }
}

fn initialize_app_state() -> MyLifeApp {
    debug!("Initializing app state");
    MyLifeApp {
        view: "Lifetime".to_string(),
        selected_yaml: "default".to_string(),
        selected_legend_item: None,
        original_legend_item: None,
        selected_life_period: None,
        value: 0.0,
        loaded_yamls: vec![("default".to_string(), Yaml::default())],
        item_state: None,
        temp_start_date: String::new(),
        data_folder: "data".to_string(),
        screenshot_data: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize_state_empty_y() {
        let (yaml, app) = initialize_state("").await;
        assert_eq!(app.selected_yaml, "default");
        assert!(!yaml.life_periods.is_empty()); // Assuming default timeline has periods
    }

    #[tokio::test]
    async fn test_initialize_state_with_y() {
        let (yaml, app) = initialize_state("some_shared_timeline").await;
        assert_eq!(app.selected_yaml, "Shared Timeline");
        assert!(!yaml.life_periods.is_empty()); // Assuming default timeline has periods
    }

    #[test]
    fn test_initialize_app_state() {
        let app = initialize_app_state();
        assert_eq!(app.view, "Lifetime");
        assert_eq!(app.selected_yaml, "default");
        assert!(app.selected_legend_item.is_none());
        assert!(app.original_legend_item.is_none());
        assert!(app.selected_life_period.is_none());
        assert_eq!(app.value, 0.0);
        assert_eq!(app.temp_start_date, "");
        assert_eq!(app.data_folder, "data");
        assert!(app.screenshot_data.is_none());
    }
}