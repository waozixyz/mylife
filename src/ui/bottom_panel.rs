use crate::ui::draw_legend;
use crate::MyLifeApp;
use eframe::egui;

pub fn draw_bottom_panel(app: &mut MyLifeApp, ctx: &egui::Context, bottom_height: f32) {
    egui::TopBottomPanel::bottom("legend")
        .min_height(bottom_height)
        .resizable(true)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(legend_item) =
                    draw_legend(ui, &app.config, &app.view, app.selected_year)
                {
                    app.selected_legend_item = Some(legend_item);
                }
            });
        });
}
