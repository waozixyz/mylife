use crate::managers::timeline_manager::get_timeline_manager;
use crate::models::timeline::{MyLifeApp, Yaml};
#[cfg(target_arch = "wasm32")]
use crate::utils::compression::decode_and_decompress;
use tracing::{debug, error};
use uuid::Uuid;
fn ensure_ids(yaml: &mut Yaml) {
    // Assign IDs to periods that don't have them
    for period in &mut yaml.life_periods {
        if period.id.is_none() {
            period.id = Some(Uuid::new_v4());
        }

        // Assign IDs to events that don't have them
        for event in &mut period.events {
            if event.id.is_none() {
                event.id = Some(Uuid::new_v4());
            }
        }
    }
}
pub async fn initialize_state(y: &str) -> (Yaml, MyLifeApp) {
    // Get the initial YAML state with IDs
    let yaml_state = if !y.is_empty() {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(decompressed_str) = decode_and_decompress(y) {
                match serde_yaml::from_str::<Yaml>(&decompressed_str) {
                    Ok(mut new_yaml) => {
                        debug!("Successfully parsed shared YAML");
                        ensure_ids(&mut new_yaml);
                        // Save to timeline manager to ensure IDs are preserved
                        if let Err(e) = get_timeline_manager().update_timeline(&new_yaml).await {
                            error!("Failed to save timeline with IDs: {}", e);
                        }
                        new_yaml
                    }
                    Err(e) => {
                        error!("Failed to parse YAML: {}", e);
                        let mut default = get_default_timeline().await;
                        ensure_ids(&mut default);
                        if let Err(e) = get_timeline_manager().update_timeline(&default).await {
                            error!("Failed to save default timeline with IDs: {}", e);
                        }
                        default
                    }
                }
            } else {
                error!("Failed to decompress YAML");
                let mut default = get_default_timeline().await;
                ensure_ids(&mut default);
                if let Err(e) = get_timeline_manager().update_timeline(&default).await {
                    error!("Failed to save default timeline with IDs: {}", e);
                }
                default
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut default = get_default_timeline().await;
            ensure_ids(&mut default);
            if let Err(e) = get_timeline_manager().update_timeline(&default).await {
                error!("Failed to save default timeline with IDs: {}", e);
            }
            default
        }
    } else {
        debug!("No shared YAML, loading from timeline manager");
        let mut default = get_default_timeline().await;
        ensure_ids(&mut default);
        // Always save back to ensure IDs are preserved
        if let Err(e) = get_timeline_manager().update_timeline(&default).await {
            error!("Failed to save default timeline with IDs: {}", e);
        }
        default
    };

    // Initialize the app state
    let mut app_state = initialize_app_state();
    app_state.loaded_yamls = vec![("default".to_string(), yaml_state.clone())];

    if !y.is_empty() {
        app_state.selected_yaml = "Shared Timeline".to_string();
    }

    // Store the initial state in the timeline manager to ensure IDs are preserved
    if let Err(e) = get_timeline_manager().update_timeline(&yaml_state).await {
        error!("Failed to save initial timeline: {}", e);
    }

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
