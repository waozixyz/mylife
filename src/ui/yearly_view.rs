use crate::models::RuntimeConfig;
use crate::utils::color_utils::get_color_for_yearly_event;
use chrono::NaiveDate;
use eframe::egui;
use eframe::epaint::Vec2;

pub fn draw_yearly_view(
    ui: &mut egui::Ui,
    grid_size: Vec2,
    config: &RuntimeConfig,
    selected_year: i32,
) {
    if let Some(events) = config.yearly_events.get(&selected_year) {
        let rows = 13;
        let cols = 28;

        // Calculate cell size based on available space
        let cell_size = (grid_size.x / cols as f32)
            .min(grid_size.y / rows as f32)
            .floor();

        // Calculate actual grid dimensions
        let grid_width = cell_size * cols as f32;
        let grid_height = cell_size * rows as f32;

        // Calculate offset to center the grid
        let offset = Vec2::new(
            (grid_size.x - grid_width) / 2.0,
            (grid_size.y - grid_height) / 2.0,
        );

        // Create a new rectangle for our grid
        let grid_rect = egui::Rect::from_min_size(
            ui.min_rect().min + offset,
            Vec2::new(grid_width, grid_height),
        );

        // Draw the grid
        for row in 0..rows {
            for col in 0..cols {
                let day = row * cols + col + 1;
                let color = if day <= 365 {
                    let date = NaiveDate::from_ymd_opt(selected_year, 1, 1).unwrap()
                        + chrono::Duration::days(day as i64 - 1);
                    get_color_for_yearly_event(&date, events)
                } else {
                    egui::Color32::GRAY
                };
                let rect = egui::Rect::from_min_size(
                    grid_rect.min + Vec2::new(col as f32 * cell_size, row as f32 * cell_size),
                    Vec2::new(cell_size, cell_size),
                );
                ui.painter().rect_filled(rect, 0.0, color);
                ui.painter()
                    .rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
            }
        }
    }
}
