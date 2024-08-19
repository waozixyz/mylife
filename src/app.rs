use eframe::egui;
use crate::ui::{draw_lifetime_view, draw_yearly_view, draw_legend};
use crate::utils::{load_config, get_yaml_files_in_data_folder};
#[cfg(target_arch = "wasm32")]
use crate::config::DEFAULT_CONFIG_YAML;
use crate::models::{Config, RuntimeConfig, LifePeriod, YearlyEvent, LegendItem};
use std::path::Path;
use log::debug;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct MyLifeApp {
    config: RuntimeConfig,
    view: String,
    selected_year: i32,
    #[cfg(not(target_arch = "wasm32"))]
    yaml_files: Vec<String>,
    #[cfg(target_arch = "wasm32")]
    yaml_content: String,
    selected_yaml: String,
    #[serde(skip)]
    value: f32,
    selected_legend_item: Option<LegendItem>,
    #[serde(skip)]
    original_legend_item: Option<LegendItem>,
}

impl Default for MyLifeApp {
    fn default() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let config = load_config(DEFAULT_CONFIG_YAML);
            } else {
                let yaml_files = get_yaml_files_in_data_folder();
                let default_yaml = "default.yaml".to_string();
                let config = load_config(&default_yaml);
            } 
        }

        Self {
            config,
            view: "Lifetime".to_string(),
            selected_year: 2024,
            #[cfg(not(target_arch = "wasm32"))]
            yaml_files,
            #[cfg(target_arch = "wasm32")]
            yaml_content: DEFAULT_CONFIG_YAML.to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            selected_yaml: default_yaml,
            #[cfg(target_arch = "wasm32")]
            selected_yaml: "Default".to_string(),
            value: 2.7,
            selected_legend_item: None,
            original_legend_item: None,
        }
    }
}


impl MyLifeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    fn update_config_item(&mut self, item: &LegendItem) {
        if item.is_yearly {
            if let Some(events) = self.config.yearly_events.get_mut(&self.selected_year) {
                if let Some(event) = events.iter_mut().find(|e| e.id == item.id) {
                    event.color = item.name.clone(); // For yearly events, name is stored in color
                    event.start = item.start.clone();
                }
            }
        } else {
            if let Some(period) = self.config.life_periods.iter_mut().find(|p| p.id == item.id) {
                period.name = item.name.clone();
                period.start = item.start.clone();
                period.color = item.color.clone();
            }
        }
        self.save_config();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_config(&self) {
        let config = runtime_config_to_config(&self.config);
        let yaml = serde_yaml::to_string(&config).unwrap();
        std::fs::write(Path::new("data").join(&self.selected_yaml), yaml).expect("Unable to write file");
    }

    #[cfg(target_arch = "wasm32")]
    fn save_config(&mut self) {
        self.yaml_content = serde_yaml::to_string(&self.config).unwrap();
        // For WASM, we might want to trigger a download or update some web storage here
    }
}

fn runtime_config_to_config(runtime_config: &RuntimeConfig) -> Config {
    Config {
        name: runtime_config.name.clone(),
        date_of_birth: runtime_config.date_of_birth.clone(),
        life_expectancy: runtime_config.life_expectancy,
        life_periods: runtime_config.life_periods.iter().map(|p| LifePeriod {
            name: p.name.clone(),
            start: p.start.clone(),
            color: p.color.clone(),
        }).collect(),
        yearly_events: runtime_config.yearly_events.iter().map(|(year, events)| {
            (*year, events.iter().map(|e| YearlyEvent {
                color: e.color.clone(),
                start: e.start.clone(),
            }).collect())
        }).collect(),
    }
}

impl eframe::App for MyLifeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.screen_rect();
        let top_height = 20.0;
        let bottom_height = screen_rect.height() * 0.2;
        let central_height = screen_rect.height() * 0.8 - 20.0;

        egui::TopBottomPanel::top("top_panel").exact_height(top_height).show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::TopBottomPanel::bottom("legend")
        .exact_height(bottom_height)
        .resizable(false)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(legend_item) = draw_legend(ui, &self.config, &self.view, self.selected_year) {
                    self.selected_legend_item = Some(legend_item);
                }
            });
        });
        
        if let Some(mut item) = self.selected_legend_item.clone() {
            let mut should_close = false;

            // Store the original item when opening the edit window
            if self.original_legend_item.is_none() {
                self.original_legend_item = Some(item.clone());
            }

            egui::Window::new("Edit Legend Item")
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        let mut changed = false;
                        ui.horizontal(|ui| {
                            ui.label("Name:");
                            if ui.text_edit_singleline(&mut item.name).changed() {
                                changed = true;
                                debug!("Name changed to: {}", item.name);
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Start:");
                            let mut start_date = item.start.clone();
                            if ui.text_edit_singleline(&mut start_date).changed() {
                                // Only update if it's a valid date
                                if let Ok(_) = chrono::NaiveDate::parse_from_str(&format!("{}-01", start_date), "%Y-%m-%d") {
                                    item.start = start_date;
                                    changed = true;
                                }
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Color:");
                            let mut color = egui::Color32::from_hex(&item.color).unwrap_or(egui::Color32::WHITE);
                            if ui.color_edit_button_srgba(&mut color).changed() {
                                item.color = format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b());
                                changed = true;
                            }
                        });

                        if changed {
                            debug!("Item changed, updating config...");
                            self.update_config_item(&item);
                        }

                        ui.label(format!("Debug - Current item: {:?}", item));

                        ui.horizontal(|ui| {
                            if ui.button("Close").clicked() {
                                should_close = true;
                            }
                        });
                    });
                });

            if should_close {
                self.selected_legend_item = None;
                self.original_legend_item = None;
            } else {
                self.selected_legend_item = Some(item);
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_rect = ui.available_rect_before_wrap();
            let min_width = 800.0;
            let min_height = central_height.max(400.0);
        
            // Fixed top part
            ui.vertical_centered(|ui| {
                ui.heading(&self.config.name);
                ui.horizontal(|ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        egui::ComboBox::from_label("YAML File")
                            .selected_text(&self.selected_yaml)
                            .show_ui(ui, |ui| {
                                for yaml_file in &self.yaml_files {
                                    if ui.selectable_value(&mut self.selected_yaml, yaml_file.clone(), yaml_file).changed() {
                                        self.config = load_config(&self.selected_yaml);
                                    }
                                }
                            });
                    }
        
                    #[cfg(target_arch = "wasm32")]
                    if ui.button("Load YAML").clicked() {
                        let ctx_clone = ctx.clone();
                        let mut app_clone = self.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            load_yaml(&mut app_clone, &ctx_clone).await;
                        });
                    }
        
                    egui::ComboBox::from_label("View")
                        .selected_text(&self.view)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.view, "Lifetime".to_string(), "Lifetime");
                            ui.selectable_value(&mut self.view, "Yearly".to_string(), "Yearly");
                        });
                    
                    if self.view == "Lifetime" {
                        egui::ComboBox::from_label("Life Expectancy")
                            .selected_text(self.config.life_expectancy.to_string())
                            .show_ui(ui, |ui| {
                                for year in 60..=120 {
                                    ui.selectable_value(&mut self.config.life_expectancy, year, year.to_string());
                                }
                            });
                    } else if self.view == "Yearly" {
                        egui::ComboBox::from_label("Year")
                            .selected_text(self.selected_year.to_string())
                            .show_ui(ui, |ui| {
                                for year in self.config.yearly_events.keys() {
                                    ui.selectable_value(&mut self.selected_year, *year, year.to_string());
                                }
                            });
                    }
                });
            });
        
            ui.add_space(20.0);
        
            // Scrollable grid part
            egui::ScrollArea::both()
                .max_width(f32::INFINITY)
                .show(ui, |ui| {
                    let grid_size = egui::vec2(
                        available_rect.width().min(min_width),
                        (available_rect.height() - 100.0).min(min_height),
                    );
        
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(240, 240, 240))
                        .show(ui, |ui| {
                            if self.view == "Lifetime" {
                                draw_lifetime_view(ui, grid_size, &self.config);
                            } else {
                                draw_yearly_view(ui, grid_size, &self.config, self.selected_year);
                            }
                        });
                });
        });
    }
}

#[cfg(target_arch = "wasm32")]
async fn load_yaml(app: &mut MyLifeApp, ctx: &egui::Context) {
    let file = rfd::AsyncFileDialog::new()
        .add_filter("YAML", &["yaml", "yml"])
        .pick_file()
        .await;
    
    if let Some(file) = file {
        let content = file.read().await;
        match String::from_utf8(content) {
            Ok(yaml_content) => {
                app.yaml_content = yaml_content.clone();
                match serde_yaml::from_str(&yaml_content) {
                    Ok(new_config) => {
                        app.config = new_config;
                        log::info!("YAML file loaded successfully");
                    },
                    Err(e) => {
                        log::error!("Failed to parse YAML content: {}. Using default config.", e);
                        app.config = load_config(DEFAULT_CONFIG_YAML);
                    }
                }
                ctx.request_repaint();
            },
            Err(e) => {
                log::error!("Invalid UTF-8 in file: {}. Using default config.", e);
                app.config = load_config(DEFAULT_CONFIG_YAML);
                ctx.request_repaint();
            }
        }
    }
}