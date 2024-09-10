use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Deserialize, Serialize, Clone, Debug)]
#[serde(default)]
pub struct MyLifeApp {
    pub selected_life_period: Option<Uuid>,
    pub view: String,
    pub selected_yaml: String,
    #[serde(skip)]
    pub value: f32,
    pub selected_legend_item: Option<LegendItem>,
    #[serde(skip)]
    pub original_legend_item: Option<LegendItem>,
    pub loaded_yamls: Vec<(String, Yaml)>,
    pub item_state: Option<LegendItem>,
    pub temp_start_date: String,
    pub data_folder: String,
    pub screenshot_data: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Yaml {
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
