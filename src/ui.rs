use eframe::egui;
use eframe::epaint::Vec2;
use chrono::{NaiveDate, Utc};
use crate::utils::hex_to_color32;
use crate::models::{RuntimeConfig, RuntimeLifePeriod, RuntimeYearlyEvent, LegendItem};

pub fn draw_lifetime_view(ui: &mut egui::Ui, grid_size: Vec2, config: &RuntimeConfig) {
    let dob = NaiveDate::parse_from_str(&format!("{}-01", config.date_of_birth), "%Y-%m-%d")
        .expect("Invalid date_of_birth format in config. Expected YYYY-MM");

    let years = config.life_expectancy;
    let rows = (years + 3) / 4;
    let cols = 48;

    let cell_size = (grid_size.x.min(grid_size.y * cols as f32 / rows as f32) / cols as f32).floor();
    let grid_width = cell_size * cols as f32;
    let grid_height = cell_size * rows as f32;

    let offset = Vec2::new(
        (grid_size.x - grid_width) / 2.0,
        (grid_size.y - grid_height) / 2.0
    );

    for i in 0..rows {
        for j in 0..cols {
            let current_date = dob + chrono::Duration::days(((i * cols + j) * 30) as i64);
            let color = get_color_for_date(&current_date, &config.life_periods);
            let rect = egui::Rect::from_min_size(
                ui.min_rect().min + offset + Vec2::new(j as f32 * cell_size, i as f32 * cell_size),
                Vec2::new(cell_size, cell_size),
            );
            ui.painter().rect_filled(rect, 0.0, color);
            ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
        }
    }
}

pub fn draw_yearly_view(ui: &mut egui::Ui, grid_size: Vec2, config: &RuntimeConfig, selected_year: i32) {
    if let Some(events) = config.yearly_events.get(&selected_year) {
        let rows = 13;
        let cols = 28;

        let cell_size = (grid_size.x.min(grid_size.y * cols as f32 / rows as f32) / cols as f32).floor();
        let grid_width = cell_size * cols as f32;
        let grid_height = cell_size * rows as f32;

        let offset = Vec2::new(
            (grid_size.x - grid_width) / 2.0,
            (grid_size.y - grid_height) / 2.0
        );

        for row in 0..rows {
            for col in 0..cols {
                let day = row * cols + col + 1;
                let color = if day <= 365 {
                    let date = NaiveDate::from_ymd_opt(selected_year, 1, 1).unwrap() + chrono::Duration::days(day as i64 - 1);
                    get_color_for_yearly_event(&date, events)
                } else {
                    egui::Color32::GRAY
                };
                let rect = egui::Rect::from_min_size(
                    ui.min_rect().min + offset + Vec2::new(col as f32 * cell_size, row as f32 * cell_size),
                    Vec2::new(cell_size, cell_size),
                );
                ui.painter().rect_filled(rect, 0.0, color);
                ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
            }
        }
    }
}

pub fn draw_legend(ui: &mut egui::Ui, config: &RuntimeConfig, view: &str, selected_year: i32) -> Option<LegendItem> {
    ui.label("Legend:");
    ui.add_space(5.0);

    let legend_height = 20.0;
    let mut selected_item = None;

    if view == "Lifetime" {
        let mut sorted_periods = config.life_periods.clone();
        sorted_periods.sort_by(|a, b| a.start.cmp(&b.start));

        for period in sorted_periods {
            let color = hex_to_color32(&period.color);
            let (rect, response) = ui.allocate_exact_size(egui::vec2(ui.available_width(), legend_height), egui::Sense::click());
            ui.painter().rect_filled(rect, 0.0, color);
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("{} (from {})", period.name, period.start),
                egui::TextStyle::Body.resolve(ui.style()),
                egui::Color32::BLACK,
            );

            if response.clicked() {
                selected_item = Some(LegendItem {
                    id: period.id,
                    name: period.name.clone(),
                    start: period.start.clone(),
                    color: period.color.clone(),
                    is_yearly: false,
                });
            }
        }
    } else if let Some(events) = config.yearly_events.get(&selected_year) {
        let mut sorted_events = events.clone();
        sorted_events.sort_by(|a, b| a.start.cmp(&b.start));

        for event in sorted_events {
            let color = hex_to_color32(&event.color);
            let (rect, response) = ui.allocate_exact_size(egui::vec2(ui.available_width(), legend_height), egui::Sense::click());
            ui.painter().rect_filled(rect, 0.0, color);
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("{} (from {})", event.color, event.start),
                egui::TextStyle::Body.resolve(ui.style()),
                egui::Color32::BLACK,
            );

            if response.clicked() {
                selected_item = Some(LegendItem {
                    id: event.id,
                    name: event.color.clone(),
                    start: event.start.clone(),
                    color: event.color.clone(),
                    is_yearly: true,
                });
            }
        }
    }

    selected_item
}

fn get_color_for_date(date: &NaiveDate, life_periods: &[RuntimeLifePeriod]) -> egui::Color32 {
    let current_date = Utc::now().naive_utc().date();
    
    if date > &current_date {
        return egui::Color32::WHITE;
    }

    for period in life_periods.iter().rev() {
        let start = NaiveDate::parse_from_str(&format!("{}-01", period.start), "%Y-%m-%d")
            .unwrap_or_else(|e| panic!("Failed to parse start date '{}' for period '{}': {:?}", period.start, period.name, e));
        if &start <= date {
            return hex_to_color32(&period.color);
        }
    }
    egui::Color32::WHITE
}

fn get_color_for_yearly_event(date: &NaiveDate, events: &[RuntimeYearlyEvent]) -> egui::Color32 {
    let current_date = Utc::now().naive_utc().date();
    
    if date > &current_date {
        return egui::Color32::WHITE;
    }

    for event in events.iter().rev() {
        let start = NaiveDate::parse_from_str(&event.start, "%Y-%m-%d")
            .unwrap_or_else(|e| panic!("Failed to parse start date '{}' for event: {:?}", event.start, e));
        if &start <= date {
            return hex_to_color32(&event.color);
        }
    }
    egui::Color32::WHITE
}