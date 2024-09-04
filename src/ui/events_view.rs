use crate::models::{RuntimeConfig, RuntimeLifePeriodEvent};
use crate::utils::color_utils::hex_to_color32;
use chrono::NaiveDate;
use eframe::egui;
use eframe::epaint::Vec2;
use uuid::Uuid;
use web_sys::console;
pub fn draw_event_view(
    ui: &mut egui::Ui,
    available_size: Vec2,
    config: &RuntimeConfig,
    selected_life_period_id: Uuid,
) {
    if let Some((index, life_period)) = config.life_periods.iter().enumerate().find(|(_, p)| p.id == selected_life_period_id) {
        let events = &life_period.events;
        
        if events.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("No events in this life period.");
            });
            return;
        }

        let start_date = events.iter()
            .filter_map(|event| NaiveDate::parse_from_str(&event.start, "%Y-%m-%d").ok())
            .min()
            .unwrap_or_else(|| {
                NaiveDate::parse_from_str(&life_period.start, "%Y-%m").unwrap_or(chrono::Local::now().date_naive())
            });

        let end_date = if index < config.life_periods.len() - 1 {
            NaiveDate::parse_from_str(&format!("{}-01", &config.life_periods[index + 1].start), "%Y-%m-%d")
                .unwrap_or_else(|_| chrono::Local::now().date_naive())
        } else {
            chrono::Local::now().date_naive()
        };

        console::log_1(&format!("start date: {}", start_date).into());
        console::log_1(&format!("end date: {}", end_date).into());
    
        if index < config.life_periods.len() - 1 {
            console::log_1(&format!("next life period start: {}", config.life_periods[index + 1].start).into());
        } else {
            console::log_1(&"This is the last life period".into());
        }
        let total_days = (end_date - start_date).num_days() as usize;
        let cols = 28;
        let rows = (total_days + cols - 1) / cols;

        let cell_width = available_size.x / cols as f32;
        let cell_height = available_size.y / rows as f32;
        let cell_size = cell_width.min(cell_height).floor();

        let grid_width = cell_size * cols as f32;
        let grid_height = cell_size * rows as f32;

        let offset = Vec2::new(
            (available_size.x - grid_width) / 2.0,
            0.0,
        );

        let grid_rect = egui::Rect::from_min_size(
            ui.min_rect().min + offset,
            Vec2::new(grid_width, grid_height),
        );

        egui::ScrollArea::both().show(ui, |ui| {
            ui.set_min_size(available_size);

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