use crate::models::RuntimeConfig;
use crate::utils::color_utils::get_color_for_date;
use chrono::NaiveDate;
use eframe::egui;
use eframe::epaint::Vec2;

pub fn draw_lifetime_view(ui: &mut egui::Ui, grid_size: Vec2, config: &RuntimeConfig) {
    let dob = NaiveDate::parse_from_str(&format!("{}-01", config.date_of_birth), "%Y-%m-%d")
        .expect("Invalid date_of_birth format in config. Expected YYYY-MM");

    let years = config.life_expectancy;
    let rows = (years + 3) / 4;
    let cols = 48;

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
    for i in 0..rows {
        for j in 0..cols {
            let current_date = dob + chrono::Duration::days(((i * cols + j) * 30) as i64);
            let color = get_color_for_date(&current_date, &config.life_periods);
            let rect = egui::Rect::from_min_size(
                grid_rect.min + Vec2::new(j as f32 * cell_size, i as f32 * cell_size),
                Vec2::new(cell_size, cell_size),
            );
            ui.painter().rect_filled(rect, 0.0, color);
            ui.painter()
                .rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
        }
    }
}
