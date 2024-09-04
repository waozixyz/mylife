use crate::models::{RuntimeConfig, RuntimeLifePeriodEvent};
use crate::utils::color_utils::hex_to_color32;
use chrono::NaiveDate;
use eframe::egui;
use eframe::epaint::Vec2;
use uuid::Uuid;
pub fn draw_event_view(
    ui: &mut egui::Ui,
    available_size: Vec2,
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

        // Find the earliest event start date and end date
        let start_date = events.iter()
            .filter_map(|event| NaiveDate::parse_from_str(&event.start, "%Y-%m-%d").ok())
            .min()
            .unwrap_or_else(|| {
                NaiveDate::parse_from_str(&life_period.start, "%Y-%m").unwrap_or(chrono::Local::now().date_naive())
            });

        let end_date = if let Some(next_period) = config.life_periods.iter()
            .find(|p| p.start > life_period.start) {
            NaiveDate::parse_from_str(&next_period.start, "%Y-%m")
                .unwrap_or(chrono::Local::now().date_naive())
        } else {
            chrono::Local::now().date_naive()
        };

        let total_days = (end_date - start_date).num_days() as usize;
        let cols = 28;
        let rows = (total_days + cols - 1) / cols; // Round up to nearest multiple of 28

        // Debug information
        let debug_height = ui.text_style_height(&egui::TextStyle::Body) * 7.0; // Approximate height for 7 lines of debug text
        ui.label(format!("Available size: {:?}", available_size));
        ui.label(format!("Total days: {}", total_days));
        ui.label(format!("Rows: {}", rows));

        // Adjust available size to account for debug information
        let adjusted_available_size = Vec2::new(available_size.x, available_size.y - debug_height);

        // Calculate cell size based on adjusted available space
        let cell_width = adjusted_available_size.x / cols as f32;
        let cell_height = adjusted_available_size.y / rows as f32;
        let cell_size = cell_width.min(cell_height).floor();

        // Debug information
        ui.label(format!("Cell width: {:.2}", cell_width));
        ui.label(format!("Cell height: {:.2}", cell_height));
        ui.label(format!("Cell size: {:.2}", cell_size));

        let grid_width = cell_size * cols as f32;
        let grid_height = cell_size * rows as f32;

        // Debug information
        ui.label(format!("Grid size: {:.2} x {:.2}", grid_width, grid_height));

        let offset = Vec2::new(
            (adjusted_available_size.x - grid_width) / 2.0,
            0.0, // Align to the top
        );

        let grid_rect = egui::Rect::from_min_size(
            ui.min_rect().min + offset + Vec2::new(0.0, debug_height), // Add debug_height to y-coordinate
            Vec2::new(grid_width, grid_height),
        );

        // Create a ScrollArea to allow scrolling if the grid is too large
        egui::ScrollArea::both().show(ui, |ui| {
            ui.set_min_size(adjusted_available_size);

            for day in 0..total_days {
                let row = day / cols;
                let col = day % cols;
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
        });
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