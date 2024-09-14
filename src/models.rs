use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct SizeInfo {
    pub cell_size: f64,
    pub window_width: f64,
    pub window_height: f64,
}

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

#[derive(PartialEq, Clone)]
pub struct CellData {
    pub color: String,
    pub period: Option<LifePeriod>,
    pub date: NaiveDate,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Yaml {
    pub name: String,
    pub date_of_birth: String,
    pub life_expectancy: u32,
    pub life_periods: Vec<LifePeriod>,
    pub routines: Vec<Routine>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Routine {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct LifePeriod {
    pub name: String,
    pub start: String,
    pub color: String,
    #[serde(default)]
    pub events: Vec<LifePeriodEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct LifePeriodEvent {
    pub name: String,
    pub color: String,
    pub start: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegendItem {
    pub id: Uuid,
    pub name: String,
    pub start: String,
    pub color: String,
    pub is_event: bool,
}
