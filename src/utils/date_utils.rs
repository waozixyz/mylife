use chrono::NaiveDate;

pub fn is_valid_date(date_str: &str) -> bool {
    NaiveDate::parse_from_str(&format!("{}-01", date_str), "%Y-%m-%d").is_ok()
}
