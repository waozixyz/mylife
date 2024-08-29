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
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::ComboBox::from_label("Theme")
                    .selected_text(format!("{:?}", app.theme))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.theme, CatppuccinTheme::Frappe, "Frappe");
                        ui.selectable_value(&mut app.theme, CatppuccinTheme::Latte, "Latte");
                        ui.selectable_value(&mut app.theme, CatppuccinTheme::Macchiato, "Macchiato");
                        ui.selectable_value(&mut app.theme, CatppuccinTheme::Mocha, "Mocha");
                    });
            });

            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
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
                    #[cfg(target_arch = "wasm32")]
                    {
                        if ui.button("Load YAML").clicked() {
                            let ctx = ctx.clone();
                            spawn_local(async move {
                                if let Some(new_config) = load_config_async().await {
                                    *NEW_CONFIG.lock().unwrap() = Some(new_config);
                                    ctx.request_repaint();
                                }
                            });
                        }

                        if ui.button("Save YAML").clicked() {
                            get_config_manager()
                                .save_config(&app.config, "config.yaml")
                                .expect("Failed to save config");
                        }
                    }

                    egui::ComboBox::from_label("View")
                        .selected_text(&app.view)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.view, "Lifetime".to_string(), "Lifetime");
                            if app.selected_life_period.is_some() {
                                ui.selectable_value(&mut app.view, "EventView".to_string(), "Event View");
                            }
                        });

                    match app.view.as_str() {
                        "Lifetime" => {
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
                        "EventView" => {
                            if let Some(period_id) = app.selected_life_period {
                                if let Some(period) = app.config.life_periods.iter().find(|p| p.id == period_id) {
                                    ui.label(&period.name);
                                }
                            }
                            if ui.button("Back to Lifetime").clicked() {
                                app.view = "Lifetime".to_string();
                                app.selected_life_period = None;
                            }
                        }
                        _ => {}
                    }
                });
            });
        });
}
