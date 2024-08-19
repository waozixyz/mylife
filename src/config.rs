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
    color: \"#FFFFBA\"
yearly_events:
  2022:
    - color: \"#4CAF50\"
      start: 2022-01-03
    - color: \"#2196F3\"
      start: 2022-03-21
    - color: \"#4CAF50\"
      start: 2022-06-06
    - color: \"#2196F3\"
      start: 2022-09-05
  2023:
    - color: \"#FFA500\"
      start: 2023-01-01
    - color: \"#4CAF50\"
      start: 2023-02-01
    - color: \"#FFFFBA\"
      start: 2023-05-01
    - color: \"#4CAF50\"
      start: 2023-09-01
    - color: \"#FFA500\"
      start: 2023-12-23
  2024:
    - color: \"#4CAF50\"
      start: 2024-01-08
    - color: \"#2196F3\"
      start: 2024-04-01
    - color: \"#FFA500\"
      start: 2024-06-15
    - color: \"#FFFFBA\"
      start: 2024-08-01
    - color: \"#FFA500\"
      start: 2024-11-25
";