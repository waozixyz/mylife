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
    - name: Winter Internship
      start: 2022-01-03
      color: \"#4CAF50\"
    - name: Spring Semester
      start: 2022-03-21
      color: \"#2196F3\"
    - name: Summer Job
      start: 2022-06-06
      color: \"#FFA500\"
    - name: Fall Semester
      start: 2022-09-05
      color: \"#9C27B0\"
  2023:
    - name: New Year Trip
      start: 2023-01-01
      color: \"#E91E63\"
    - name: Job Search
      start: 2023-02-01
      color: \"#607D8B\"
    - name: First Full-time Job
      start: 2023-05-01
      color: \"#795548\"
    - name: Work From Home
      start: 2023-09-01
      color: \"#FF5722\"
    - name: Year-end Vacation
      start: 2023-12-23
      color: \"#009688\"
  2024:
    - name: New Project at Work
      start: 2024-01-08
      color: \"#3F51B5\"
    - name: Learning New Skills
      start: 2024-04-01
      color: \"#CDDC39\"
    - name: Summer Workation
      start: 2024-06-15
      color: \"#FF9800\"
    - name: Promotion Preparation
      start: 2024-08-01
      color: \"#9C27B0\"
    - name: Holiday Season
      start: 2024-11-25
      color: \"#F44336\"
";