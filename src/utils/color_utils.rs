use crate::models::{RuntimeLifePeriod, RuntimeYearlyEvent};
use chrono::{NaiveDate, Utc};
use eframe::egui;

pub fn hex_to_color32(hex: &str) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    egui::Color32::from_rgb(r, g, b)
}

pub fn color32_to_hex(color: egui::Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}

fn get_color_for_item<T, F, G>(
    date: &NaiveDate,
    items: &[T],
    get_start: F,
    get_color: G,
) -> egui::Color32
where
    F: Fn(&T) -> String,
    G: Fn(&T) -> &String,
{
    let current_date = Utc::now().naive_utc().date();

    if date > &current_date {
        return egui::Color32::WHITE;
    }

    for item in items.iter().rev() {
        let start = NaiveDate::parse_from_str(&get_start(item), "%Y-%m-%d").unwrap_or_else(|e| {
            panic!("Failed to parse start date '{}': {:?}", get_start(item), e)
        });
        if &start <= date {
            return hex_to_color32(get_color(item));
        }
    }
    egui::Color32::WHITE
}

pub fn get_color_for_date(date: &NaiveDate, life_periods: &[RuntimeLifePeriod]) -> egui::Color32 {
    get_color_for_item(
        date,
        life_periods,
        |period| format!("{}-01", period.start),
        |period| &period.color,
    )
}

pub fn get_color_for_yearly_event(
    date: &NaiveDate,
    events: &[RuntimeYearlyEvent],
) -> egui::Color32 {
    get_color_for_item(
        date,
        events,
        |event| event.start.clone(),
        |event| &event.color,
    )
}
