#[cfg(target_arch = "wasm32")]
use crate::config::DEFAULT_CONFIG_YAML;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CatppuccinTheme {
    Frappe,
    Latte,
    Macchiato,
    Mocha,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(default)]
pub struct MyLifeApp {
    pub theme: CatppuccinTheme,
    pub config: RuntimeConfig,
    pub view: String,
    pub selected_year: i32,
    #[cfg(not(target_arch = "wasm32"))]
    pub yaml_files: Vec<String>,
    #[cfg(target_arch = "wasm32")]
    pub yaml_content: String,
    pub selected_yaml: String,
    #[serde(skip)]
    pub value: f32,
    pub selected_legend_item: Option<LegendItem>,
    #[serde(skip)]
    pub original_legend_item: Option<LegendItem>,
    #[cfg(target_arch = "wasm32")]
    pub loaded_configs: Vec<(String, RuntimeConfig)>,
    #[cfg(target_arch = "wasm32")]
    pub selected_config_index: usize,
    #[cfg(target_arch = "wasm32")]
    pub loaded_app: Option<Box<MyLifeApp>>,
    #[cfg(target_arch = "wasm32")]
    pub loaded_config: Option<RuntimeConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub name: String,
    pub date_of_birth: String,
    pub life_expectancy: u32,
    pub life_periods: Vec<LifePeriod>,
    pub yearly_events: HashMap<i32, Vec<YearlyEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct RuntimeConfig {
    pub name: String,
    pub date_of_birth: String,
    pub life_expectancy: u32,
    pub life_periods: Vec<RuntimeLifePeriod>,
    pub yearly_events: HashMap<i32, Vec<RuntimeYearlyEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LifePeriod {
    pub name: String,
    pub start: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct YearlyEvent {
    pub color: String,
    pub start: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RuntimeLifePeriod {
    pub id: Uuid,
    pub name: String,
    pub start: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RuntimeYearlyEvent {
    pub id: Uuid,
    pub color: String,
    pub start: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegendItem {
    pub id: Uuid,
    pub name: String,
    pub start: String,
    pub color: String,
    pub is_yearly: bool,
}
#[cfg(target_arch = "wasm32")]
impl Default for Config {
    fn default() -> Self {
        use log::{error, info};

        info!("Attempting to create default Config");

        match serde_yaml::from_str(DEFAULT_CONFIG_YAML) {
            Ok(config) => {
                info!("Successfully parsed DEFAULT_CONFIG_YAML");
                info!("Parsed config: {:?}", config);
                config
            }
            Err(e) => {
                error!("Failed to parse DEFAULT_CONFIG_YAML: {:?}", e);
                info!("Using fallback default config");
                Config {
                    name: "John Doe".to_string(),
                    date_of_birth: "2000-01".to_string(),
                    life_expectancy: 80,
                    life_periods: vec![],
                    yearly_events: HashMap::new(),
                }
            }
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl From<&RuntimeConfig> for Config {
    fn from(runtime_config: &RuntimeConfig) -> Self {
        Config {
            name: runtime_config.name.clone(),
            date_of_birth: runtime_config.date_of_birth.clone(),
            life_expectancy: runtime_config.life_expectancy,
            life_periods: runtime_config
                .life_periods
                .iter()
                .map(|p| LifePeriod {
                    name: p.name.clone(),
                    start: p.start.clone(),
                    color: p.color.clone(),
                })
                .collect(),
            yearly_events: runtime_config
                .yearly_events
                .iter()
                .map(|(year, events)| {
                    (
                        *year,
                        events
                            .iter()
                            .map(|e| YearlyEvent {
                                color: e.color.clone(),
                                start: e.start.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}
