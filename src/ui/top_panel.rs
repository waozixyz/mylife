use crate::config_manager::get_config_manager;
use crate::models::CatppuccinTheme;
use crate::MyLifeApp;
use eframe::egui;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
use crate::utils::config_utils::load_config_async;

#[cfg(target_arch = "wasm32")]
use crate::utils::wasm_config::NEW_CONFIG;

pub fn draw_top_panel(app: &mut MyLifeApp, ctx: &egui::Context, top_height: f32) {
    egui::TopBottomPanel::top("top_panel")
        .exact_height(top_height)
        .show(ctx, |ui| {

            ui.vertical(|ui| {
                draw_top_row(app, ui);
                draw_bottom_row(app, ctx, ui);
            });
        });


    // Settings modal (unchanged)
    if app.show_settings {
        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                // Theme selector
                egui::ComboBox::from_label("Theme")
                    .selected_text(format!("{:?}", app.theme))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.theme, CatppuccinTheme::Frappe, "Frappe");
                        ui.selectable_value(&mut app.theme, CatppuccinTheme::Latte, "Latte");
                        ui.selectable_value(&mut app.theme, CatppuccinTheme::Macchiato, "Macchiato");
                        ui.selectable_value(&mut app.theme, CatppuccinTheme::Mocha, "Mocha");
                    });

                // Life expectancy selector (only show in Lifetime view)
                if app.view == "Lifetime" {
                    egui::ComboBox::from_label("Life Expectancy")
                        .selected_text(app.config.life_expectancy.to_string())
                        .show_ui(ui, |ui| {
                            for year in 60..=120 {
                                ui.selectable_value(
                                    &mut app.config.life_expectancy,
                                    year,
                                    year.to_string(),
                                );
                            }
                        });
                }

                if ui.button("Close").clicked() {
                    app.show_settings = false;
                }
            });
    }
}

fn draw_top_row(app: &mut MyLifeApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {

        // Back button (only in EventView)
        if app.view == "EventView" {
            if ui.button("⬅").clicked() {
                app.view = "Lifetime".to_string();
                app.selected_life_period = None;
            }
        }

        // Configuration selector
        egui::ComboBox::from_label("Configuration")
            .selected_text(&app.selected_yaml)
            .show_ui(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                for yaml_file in &app.yaml_files {
                    if ui.selectable_value(&mut app.selected_yaml, yaml_file.clone(), yaml_file).changed() {
                        app.config = get_config_manager()
                            .load_config(&app.selected_yaml)
                            .expect("Failed to load config");
                    }
                }
                #[cfg(target_arch = "wasm32")]
                for (index, (name, _)) in app.loaded_configs.iter().enumerate() {
                    if ui.selectable_value(&mut app.selected_config_index, index, name).clicked() {
                        app.config = app.loaded_configs[index].1.clone();
                        app.selected_yaml = name.clone();
                    }
                }
            });

        // Push the settings button to the right
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("⚙").clicked() {
                app.show_settings = true;
            }
        });
    });
}

fn draw_bottom_row(app: &mut MyLifeApp, ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        #[cfg(target_arch = "wasm32")]
        {
            if ui.button("Load YAML").clicked() {
                let ctx = ctx.clone();
                spawn_local(async move {
                    match load_config_async().await {
                        Some((name, new_config)) => {
                            ctx.request_repaint();
                            // Update the NEW_CONFIG
                            *NEW_CONFIG.lock().unwrap() = Some((name, new_config));
                        }
                        None => {
                            let _ = rfd::MessageDialog::new()
                                .set_title("Error")
                                .set_description("Failed to load YAML configuration. Check the console for details.")
                                .show();
                        }
                    }
                });
            }

            if ui.button("Save YAML").clicked() {
                get_config_manager()
                    .save_config(&app.config, &app.selected_yaml)
                    .expect("Failed to save config");
            }
        }
        // Quit button (non-WASM only)
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Quit").clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}