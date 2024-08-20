use chrono::NaiveDate;

pub fn is_valid_date(date: &str, year_month_only: bool) -> bool {
    if year_month_only {
        // Check for YYYY-MM format
        if let Some((year, month)) = date.split_once('-') {
            return year.len() == 4
                && month.len() == 2
                && year.parse::<i32>().is_ok()
                && month
                    .parse::<u32>()
                    .map(|m| m >= 1 && m <= 12)
                    .unwrap_or(false);
        }
        false
    } else {
        // Check for YYYY-MM-DD format
        NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
    }
}
