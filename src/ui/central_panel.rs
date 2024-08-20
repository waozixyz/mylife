use crate::ui::{draw_lifetime_view, draw_yearly_view};
use crate::utils::layout_utils::calculate_centered_rect;
use crate::MyLifeApp;
use eframe::egui;

pub fn draw_central_panel(app: &mut MyLifeApp, ctx: &egui::Context, central_height: f32) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let available_rect = ui.available_rect_before_wrap();

        let min_width = 800.0;
        let min_height = central_height.max(400.0);

        let desired_grid_size = egui::vec2(min_width, min_height);
        let centered_rect = calculate_centered_rect(available_rect, desired_grid_size);
        let grid_response = ui.allocate_ui_at_rect(centered_rect, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(240, 240, 240))
                        .show(ui, |ui| {
                            if app.view == "Lifetime" {
                                draw_lifetime_view(ui, centered_rect.size(), &app.config);
                            } else {
                                draw_yearly_view(
                                    ui,
                                    centered_rect.size(),
                                    &app.config,
                                    app.selected_year,
                                );
                            }
                        });
                });
        });

        if grid_response.response.clicked() {
            // Handle click events if needed
        }
    });
}
