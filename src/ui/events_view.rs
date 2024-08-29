use crate::models::{RuntimeConfig, RuntimeLifePeriod, RuntimeLifePeriodEvent};
use crate::utils::color_utils::hex_to_color32;
use chrono::NaiveDate;
use eframe::egui;
use eframe::epaint::Vec2;
use uuid::Uuid;

pub fn draw_event_view(
    ui: &mut egui::Ui,
    grid_size: Vec2,
    config: &RuntimeConfig,
    selected_life_period_id: Uuid,
) {
    if let Some(life_period) = config.life_periods.iter().find(|p| p.id == selected_life_period_id) {
        let events = &life_period.events;
        
        if events.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("No events in this life period.");
            });
            return;
        }

        let start_date = NaiveDate::parse_from_str(&format!("{}-01", life_period.start), "%Y-%m-%d")
            .expect("Invalid start date format in life period");
        let end_date = if let Some(next_period) = config.life_periods.iter()
            .find(|p| p.start > life_period.start) {
            NaiveDate::parse_from_str(&format!("{}-01", next_period.start), "%Y-%m-%d")
                .expect("Invalid start date format in next life period")
        } else {
            // If there's no next period, use the current date
            chrono::Local::now().date_naive()
        };

        let total_days = (end_date - start_date).num_days() as usize;
        let rows = (total_days + 27) / 28; // Round up to nearest multiple of 28
        let cols = 28;

        let cell_size = (grid_size.x / cols as f32)
            .min(grid_size.y / rows as f32)
            .floor();

        let grid_width = cell_size * cols as f32;
        let grid_height = cell_size * rows as f32;

        let offset = Vec2::new(
            (grid_size.x - grid_width) / 2.0,
            (grid_size.y - grid_height) / 2.0,
        );

        let grid_rect = egui::Rect::from_min_size(
            ui.min_rect().min + offset,
            Vec2::new(grid_width, grid_height),
        );

        for day in 0..total_days {
            let row = day / 28;
            let col = day % 28;
            let date = start_date + chrono::Duration::days(day as i64);
            let color = get_color_for_event(&date, events, &end_date);
            let rect = egui::Rect::from_min_size(
                grid_rect.min + Vec2::new(col as f32 * cell_size, row as f32 * cell_size),
                Vec2::new(cell_size, cell_size),
            );
            ui.painter().rect_filled(rect, 0.0, color);
            ui.painter()
                .rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
        }
    } else {
        ui.centered_and_justified(|ui| {
            ui.label("Selected life period not found.");
        });
    }
}

fn get_color_for_event(date: &NaiveDate, events: &[RuntimeLifePeriodEvent], period_end: &NaiveDate) -> egui::Color32 {
    for (i, event) in events.iter().enumerate() {
        let event_start = NaiveDate::parse_from_str(&event.start, "%Y-%m-%d")
            .expect("Invalid start date format in event");
        let event_end = if i < events.len() - 1 {
            NaiveDate::parse_from_str(&events[i + 1].start, "%Y-%m-%d")
                .expect("Invalid start date format in next event")
        } else {
            *period_end
        };

        if date >= &event_start && date < &event_end {
            return hex_to_color32(&event.color);
        }
    }
    egui::Color32::TRANSPARENT
}