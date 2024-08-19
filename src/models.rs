use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
