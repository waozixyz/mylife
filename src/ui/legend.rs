use crate::models::{LegendItem, RuntimeConfig};
use crate::utils::color_utils::hex_to_color32;
use eframe::egui;
use uuid::Uuid;

pub fn draw_legend(
    ui: &mut egui::Ui,
    config: &RuntimeConfig,
    view: &str,
    selected_life_period: Option<Uuid>,
) -> Option<LegendItem> {
    ui.add_space(5.0);

    let legend_height = 20.0;
    let mut selected_item = None;

    match view {
        "Lifetime" => {
            let mut sorted_periods = config.life_periods.clone();
            sorted_periods.sort_by(|a, b| a.start.cmp(&b.start));

            for period in sorted_periods {
                let color = hex_to_color32(&period.color);
                let (rect, response) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), legend_height),
                    egui::Sense::click(),
                );
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
                        is_event: false,
                    });
                }
            }
        }
        "EventView" => {
            if let Some(period_id) = selected_life_period {
                if let Some(period) = config.life_periods.iter().find(|p| p.id == period_id) {
                    for event in &period.events {
                        let color = hex_to_color32(&event.color);
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(ui.available_width(), legend_height),
                            egui::Sense::click(),
                        );
                        ui.painter().rect_filled(rect, 0.0, color);
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("{} ({})", event.name, event.start),
                            egui::TextStyle::Body.resolve(ui.style()),
                            egui::Color32::BLACK,
                        );

                        if response.clicked() {
                            selected_item = Some(LegendItem {
                                id: event.id,
                                name: event.name.clone(),
                                start: event.start.clone(),
                                color: event.color.clone(),
                                is_event: true,
                            });
                        }
                    }
                }
            }
        }
        _ => {}
    }

    selected_item
}