use crate::ui::{draw_lifetime_view, draw_event_view};
use crate::MyLifeApp;
use eframe::egui;

pub fn draw_central_panel(app: &mut MyLifeApp, ctx: &egui::Context, top_height: f32, bottom_height: f32) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let screen_rect = ctx.screen_rect();
        let available_rect = ui.available_rect_before_wrap();

        // Calculate the actual available height for the central panel
        let total_height = screen_rect.height();
        let central_height = total_height - top_height - bottom_height;

        // Calculate the desired size based on available space
        let desired_width = available_rect.width().min(1200.0).max(600.0);
        let desired_height = central_height;
        let desired_size = egui::vec2(desired_width, desired_height);

        // Calculate the top-left corner of the central panel
        let central_panel_min = egui::pos2(
            (screen_rect.width() - desired_width) / 2.0,
            top_height
        );

        let central_rect = egui::Rect::from_min_size(central_panel_min, desired_size);
        
        ui.allocate_ui_at_rect(central_rect, |ui| {
            egui::Frame::none()
                .show(ui, |ui| {
                    match app.view.as_str() {
                        "Lifetime" => {
                            if let Some(clicked_period) = draw_lifetime_view(ui, central_rect.size(), &app.config, &mut app.selected_life_period) {
                                app.view = "EventView".to_string();
                                app.selected_life_period = Some(clicked_period);
                            }
                        },
                        "EventView" => {
                            if let Some(period_id) = app.selected_life_period {
                                draw_event_view(ui, central_rect.size(), &app.config, period_id);
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
}