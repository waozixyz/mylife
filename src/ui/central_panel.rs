use crate::ui::{draw_lifetime_view, draw_event_view};
use crate::utils::layout_utils::calculate_centered_rect;
use crate::MyLifeApp;
use eframe::egui;
use uuid::Uuid;

pub fn draw_central_panel(app: &mut MyLifeApp, ctx: &egui::Context, central_height: f32) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let available_rect = ui.available_rect_before_wrap();

        let min_width = 800.0;
        let min_height = central_height.max(400.0);

        let desired_grid_size = egui::vec2(min_width, min_height);
        let centered_rect = calculate_centered_rect(available_rect, desired_grid_size);
        
        ui.allocate_ui_at_rect(centered_rect, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::Frame::none()
                        .show(ui, |ui| {
                            match app.view.as_str() {
                                "Lifetime" => {
                                    if let Some(clicked_period) = draw_lifetime_view(ui, centered_rect.size(), &app.config, &mut app.selected_life_period) {
                                        app.view = "EventView".to_string();
                                        app.selected_life_period = Some(clicked_period);
                                    }
                                },
                                "EventView" => {
                                    if let Some(period_id) = app.selected_life_period {
                                        draw_event_view(ui, centered_rect.size(), &app.config, period_id);
                                        
                                        ui.vertical(|ui| {
                                            if ui.button("Back to Lifetime View").clicked() {
                                                app.view = "Lifetime".to_string();
                                                app.selected_life_period = None;
                                            }
                                        });
                                    } else {
                                        ui.label("No life period selected");
                                    }
                                },
                                _ => {
                                    ui.label("Unknown view");
                                }
                            }
                        });
                });
        });
    });
}