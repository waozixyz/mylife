use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub name: String,
    pub date_of_birth: String,
    pub life_expectancy: u32,
    pub categories: HashMap<String, String>,
    pub life_periods: Vec<LifePeriod>,
    pub yearly_events: HashMap<i32, Vec<YearlyEvent>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LifePeriod {
    pub name: String,
    pub start: String,
    pub category: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct YearlyEvent {
    pub category: String,
    pub start: String,
}

#[cfg(target_arch = "wasm32")]
pub const DEFAULT_CONFIG_YAML: &str = "\
name: John Doe
date_of_birth: 2000-01
life_expectancy: 80
categories:
  Childhood: \"#FFB3BA\"
  Teenage: \"#BAFFC9\"
  EarlyAdult: \"#BAE1FF\"
  Career: \"#FFFFBA\"
  Education: \"#2196F3\"
  Work: \"#4CAF50\"
  Vacation: \"#FFA500\"
life_periods:
  - name: Childhood
    start: 2000-01
    category: Childhood
  - name: Teenage Years
    start: 2013-01
    category: Teenage
  - name: Early Adulthood
    start: 2018-01
    category: EarlyAdult
  - name: Career Growth
    start: 2023-01
    category: Career
yearly_events:
  2022:
    - category: Work
      start: 2022-01-03
    - category: Education
      start: 2022-03-21
    - category: Work
      start: 2022-06-06
    - category: Education
      start: 2022-09-05
  2023:
    - category: Vacation
      start: 2023-01-01
    - category: Work
      start: 2023-02-01
    - category: Career
      start: 2023-05-01
    - category: Work
      start: 2023-09-01
    - category: Vacation
      start: 2023-12-23
  2024:
    - category: Work
      start: 2024-01-08
    - category: Education
      start: 2024-04-01
    - category: Vacation
      start: 2024-06-15
    - category: Career
      start: 2024-08-01
    - category: Vacation
      start: 2024-11-25
";