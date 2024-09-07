use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
#[serde(default)]
pub struct MyLifeApp {
    pub selected_life_period: Option<Uuid>,
    pub config: Config,
    pub view: String,
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
    pub loaded_configs: Vec<(String, Config)>,
    #[cfg(target_arch = "wasm32")]
    pub selected_config_index: usize,
    pub show_settings: bool,
    pub item_state: Option<LegendItem>,
    pub temp_start_date: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Config {
    pub name: String,
    pub date_of_birth: String,
    pub life_expectancy: u32,
    pub life_periods: Vec<LifePeriod>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct LifePeriod {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub name: String,
    pub start: String,
    pub color: String,
    #[serde(default)]
    pub events: Vec<LifePeriodEvent>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct LifePeriodEvent {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub start: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegendItem {
    pub id: Uuid,
    pub name: String,
    pub start: String,
    pub color: String,
    pub is_event: bool,
}
