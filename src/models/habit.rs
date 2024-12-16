use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Habit {
    pub id: Uuid,
    pub title: String,
    pub start_date: NaiveDate,
    pub color: String,
    pub week_start: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WeekStart {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl WeekStart {
    // Convert WeekStart to chrono::Weekday
    pub fn to_weekday(&self) -> Weekday {
        match self {
            WeekStart::Sunday => Weekday::Sun,
            WeekStart::Monday => Weekday::Mon,
            WeekStart::Tuesday => Weekday::Tue,
            WeekStart::Wednesday => Weekday::Wed,
            WeekStart::Thursday => Weekday::Thu,
            WeekStart::Friday => Weekday::Fri,
            WeekStart::Saturday => Weekday::Sat,
        }
    }

    // Calculate days from start given a date
    pub fn get_days_from_start(&self, date: NaiveDate) -> i64 {
        let week_start = self.to_weekday();
        let current_weekday = date.weekday();

        let days = current_weekday.num_days_from_sunday() as i64
            - week_start.num_days_from_sunday() as i64;

        if days < 0 {
            days + 7
        } else {
            days
        }
    }

    // String conversion methods
    pub fn to_string(&self) -> String {
        match self {
            WeekStart::Sunday => "sunday",
            WeekStart::Monday => "monday",
            WeekStart::Tuesday => "tuesday",
            WeekStart::Wednesday => "wednesday",
            WeekStart::Thursday => "thursday",
            WeekStart::Friday => "friday",
            WeekStart::Saturday => "saturday",
        }
        .to_string()
    }

    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "monday" => WeekStart::Monday,
            "tuesday" => WeekStart::Tuesday,
            "wednesday" => WeekStart::Wednesday,
            "thursday" => WeekStart::Thursday,
            "friday" => WeekStart::Friday,
            "saturday" => WeekStart::Saturday,
            _ => WeekStart::Sunday,
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, Deserialize)]
pub struct HabitData {
    pub title: String,
    pub start_date: NaiveDate,
    pub completed_days: Vec<NaiveDate>,
    pub week_start: WeekStart,
    pub color: String,
}
