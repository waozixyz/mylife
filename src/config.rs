#[cfg(target_arch = "wasm32")]
pub const DEFAULT_CONFIG_YAML: &str = "\
name: John Doe
date_of_birth: 2000-01
life_expectancy: 80
life_periods:
  - name: Childhood
    start: 2000-01
    color: \"#FFB3BA\"
  - name: Teenage Years
    start: 2013-01
    color: \"#BAFFC9\"
  - name: Early Adulthood
    start: 2018-01
    color: \"#BAE1FF\"
  - name: Career Growth
    start: 2023-01
    color: \"#FFFFBA\"";
