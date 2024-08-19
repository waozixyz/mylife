use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct LegendItem {
    pub name: String,
    pub start: String,
    pub color: String,
    pub is_yearly: bool,
}