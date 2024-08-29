use crate::models::{RuntimeConfig, RuntimeLifePeriod};
use crate::utils::color_utils::get_color_for_date;
use chrono::NaiveDate;
use eframe::egui;
use eframe::epaint::Vec2;
use uuid::Uuid;

pub fn draw_lifetime_view(
    ui: &mut egui::Ui,
    grid_size: Vec2,
    config: &RuntimeConfig,
    selected_life_period: &mut Option<Uuid>
) -> Option<Uuid> {
    let dob = NaiveDate::parse_from_str(&format!("{}-01", config.date_of_birth), "%Y-%m-%d")
        .expect("Invalid date_of_birth format in config. Expected YYYY-MM");

    let years = config.life_expectancy;
    let rows = (years + 3) / 4;
    let cols = 48;

    let cell_size = (grid_size.x / cols as f32)
        .min(grid_size.y / rows as f32)
        .floor();

    let grid_width = cell_size * cols as f32;
    let grid_height = cell_size * rows as f32;

    let offset = Vec2::new(0.0, 0.0);

    let grid_rect = egui::Rect::from_min_size(
        ui.min_rect().min + offset,
        Vec2::new(grid_width, grid_height),
    );

    let mut clicked_period = None;
    let current_date = chrono::Local::now().date_naive();

    for i in 0..rows {
        for j in 0..cols {
            let cell_date = dob + chrono::Duration::days(((i * cols + j) * 30) as i64);
            let rect = egui::Rect::from_min_size(
                grid_rect.min + Vec2::new(j as f32 * cell_size, i as f32 * cell_size),
                Vec2::new(cell_size, cell_size),
            );

            if cell_date <= current_date {
                let (color, period) = get_color_and_period_for_date(&cell_date, &config.life_periods);
                let response = ui.put(rect, egui::Button::new("").fill(color));
                
                if response.clicked() {
                    if let Some(period) = period {
                        if !period.events.is_empty() {
                            clicked_period = Some(period.id);
                        }
                    }
                }
            } else {
                // Draw blank cell for future dates
                ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);
            }
            
            // Draw cell border
            ui.painter()
                .rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
        }
    }

    // If a period was clicked, update the selected_life_period
    if let Some(period_id) = clicked_period {
        *selected_life_period = Some(period_id);
    }

    clicked_period
}

fn get_color_and_period_for_date<'a>(date: &NaiveDate, life_periods: &'a [RuntimeLifePeriod]) -> (egui::Color32, Option<&'a RuntimeLifePeriod>) {
    let mut current_period = None;
    for period in life_periods.iter().rev() {
        let period_start = NaiveDate::parse_from_str(&format!("{}-01", period.start), "%Y-%m-%d")
            .expect("Invalid start date format in life period");
        if date >= &period_start {
            current_period = Some(period);
            break;
        }
    }
    match current_period {
        Some(period) => (egui::Color32::from_hex(&period.color).unwrap_or(egui::Color32::GRAY), Some(period)),
        None => (egui::Color32::GRAY, None),
    }
}