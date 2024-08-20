use crate::models::LegendItem;
use crate::ui::draw_legend;
use crate::MyLifeApp;
use chrono::{Datelike, Local};
use eframe::egui;
use uuid::Uuid;

pub fn draw_bottom_panel(app: &mut MyLifeApp, ctx: &egui::Context, bottom_height: f32) {
    egui::TopBottomPanel::bottom("legend")
        .min_height(bottom_height)
        .resizable(true)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Legend:");
                    if ui.button("Add New Item").clicked() {
                        let now = Local::now();
                        let default_start = if app.view == "Lifetime" {
                            format!("{}-{:02}", now.year(), now.month())
                        } else {
                            format!("{}-{:02}-{:02}", now.year(), now.month(), now.day())
                        };

                        let new_item = if app.view == "Lifetime" {
                            LegendItem {
                                id: Uuid::new_v4(),
                                name: "New Period".to_string(),
                                start: default_start,
                                color: "#000000".to_string(),
                                is_yearly: false,
                            }
                        } else {
                            LegendItem {
                                id: Uuid::new_v4(),
                                name: "New Event".to_string(),
                                start: default_start,
                                color: "#000000".to_string(),
                                is_yearly: true,
                            }
                        };

                        app.selected_legend_item = Some(new_item);
                    }
                });

                ui.add_space(5.0);

                if let Some(legend_item) =
                    draw_legend(ui, &app.config, &app.view, app.selected_year)
                {
                    app.selected_legend_item = Some(legend_item);
                }
            });
        });
}
