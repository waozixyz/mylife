#[cfg(target_arch = "wasm32")]
use crate::config::DEFAULT_CONFIG_YAML;
#[cfg(not(target_arch = "wasm32"))]
use crate::models::{Config, LegendItem, LifePeriod, RuntimeConfig, YearlyEvent};
#[cfg(target_arch = "wasm32")]
use crate::models::{Config, LegendItem, RuntimeConfig, RuntimeLifePeriod, RuntimeYearlyEvent};
use crate::ui::{draw_legend, draw_lifetime_view, draw_yearly_view};
use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
use crate::utils::get_yaml_files_in_data_folder;
use crate::utils::load_config;
use catppuccin_egui::{FRAPPE, LATTE, MACCHIATO, MOCHA};
use log::debug;
#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
#[cfg(target_arch = "wasm32")]
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum CatppuccinTheme {
    Frappe,
    Latte,
    Macchiato,
    Mocha,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct MyLifeApp {
    theme: CatppuccinTheme,
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
#[cfg(target_arch = "wasm32")]
impl Default for Config {
    fn default() -> Self {
        serde_yaml::from_str(DEFAULT_CONFIG_YAML).unwrap_or_else(|_| Config {
            name: "John Doe".to_string(),
            date_of_birth: "2000-01".to_string(),
            life_expectancy: 80,
            life_periods: vec![],
            yearly_events: HashMap::new(),
        })
    }
}
#[cfg(target_arch = "wasm32")]
impl From<Config> for RuntimeConfig {
    fn from(config: Config) -> Self {
        RuntimeConfig {
            name: config.name,
            date_of_birth: config.date_of_birth,
            life_expectancy: config.life_expectancy,
            life_periods: config
                .life_periods
                .into_iter()
                .map(|p| RuntimeLifePeriod {
                    id: Uuid::new_v4(),
                    name: p.name,
                    start: p.start,
                    color: p.color,
                })
                .collect(),
            yearly_events: config
                .yearly_events
                .into_iter()
                .map(|(year, events)| {
                    (
                        year,
                        events
                            .into_iter()
                            .map(|e| RuntimeYearlyEvent {
                                id: Uuid::new_v4(),
                                color: e.color,
                                start: e.start,
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

impl Default for MyLifeApp {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        let config = load_config(DEFAULT_CONFIG_YAML);

        #[cfg(not(target_arch = "wasm32"))]
        let config = {
            let default_yaml = "default.yaml".to_string();
            load_config(&default_yaml)
        };

        Self {
            config,
            view: "Lifetime".to_string(),
            selected_year: 2024,
            #[cfg(not(target_arch = "wasm32"))]
            yaml_files: get_yaml_files_in_data_folder(),
            #[cfg(target_arch = "wasm32")]
            yaml_content: DEFAULT_CONFIG_YAML.to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            selected_yaml: "default.yaml".to_string(),
            #[cfg(target_arch = "wasm32")]
            selected_yaml: "Default".to_string(),
            value: 2.7,
            selected_legend_item: None,
            original_legend_item: None,
            theme: CatppuccinTheme::Mocha,
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
        } else if let Some(period) = self
            .config
            .life_periods
            .iter_mut()
            .find(|p| p.id == item.id)
        {
            period.name = item.name.clone();
            period.start = item.start.clone();
            period.color = item.color.clone();
        }
        self.save_config();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_config(&self) {
        let config = runtime_config_to_config(&self.config);
        let yaml = serde_yaml::to_string(&config).unwrap();
        std::fs::write(Path::new("data").join(&self.selected_yaml), yaml)
            .expect("Unable to write file");
    }

    #[cfg(target_arch = "wasm32")]
    fn save_config(&mut self) {
        self.yaml_content = serde_yaml::to_string(&self.config).unwrap();
        // For WASM, we might want to trigger a download or update some web storage here
    }
}
#[cfg(not(target_arch = "wasm32"))]
fn runtime_config_to_config(runtime_config: &RuntimeConfig) -> Config {
    Config {
        name: runtime_config.name.clone(),
        date_of_birth: runtime_config.date_of_birth.clone(),
        life_expectancy: runtime_config.life_expectancy,
        life_periods: runtime_config
            .life_periods
            .iter()
            .map(|p| LifePeriod {
                name: p.name.clone(),
                start: p.start.clone(),
                color: p.color.clone(),
            })
            .collect(),
        yearly_events: runtime_config
            .yearly_events
            .iter()
            .map(|(year, events)| {
                (
                    *year,
                    events
                        .iter()
                        .map(|e| YearlyEvent {
                            color: e.color.clone(),
                            start: e.start.clone(),
                        })
                        .collect(),
                )
            })
            .collect(),
    }
}

impl eframe::App for MyLifeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(
            ctx,
            match self.theme {
                CatppuccinTheme::Frappe => FRAPPE,
                CatppuccinTheme::Latte => LATTE,
                CatppuccinTheme::Macchiato => MACCHIATO,
                CatppuccinTheme::Mocha => MOCHA,
            },
        );
        let screen_rect = ctx.screen_rect();
        let top_height = 50.0;
        let bottom_height = screen_rect.height() * 0.2;
        let central_height = screen_rect.height() - top_height - bottom_height;

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

                    // Add new theme dropdown
                    egui::ComboBox::from_label("Theme")
                        .selected_text(format!("{:?}", self.theme))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.theme, CatppuccinTheme::Frappe, "Frappe");
                            ui.selectable_value(&mut self.theme, CatppuccinTheme::Latte, "Latte");
                            ui.selectable_value(
                                &mut self.theme,
                                CatppuccinTheme::Macchiato,
                                "Macchiato",
                            );
                            ui.selectable_value(&mut self.theme, CatppuccinTheme::Mocha, "Mocha");
                        });
                });

                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            egui::ComboBox::from_label("YAML File")
                                .selected_text(&self.selected_yaml)
                                .show_ui(ui, |ui| {
                                    for yaml_file in &self.yaml_files {
                                        if ui
                                            .selectable_value(
                                                &mut self.selected_yaml,
                                                yaml_file.clone(),
                                                yaml_file,
                                            )
                                            .changed()
                                        {
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
                                ui.selectable_value(
                                    &mut self.view,
                                    "Lifetime".to_string(),
                                    "Lifetime",
                                );
                                ui.selectable_value(&mut self.view, "Yearly".to_string(), "Yearly");
                            });

                        if self.view == "Lifetime" {
                            egui::ComboBox::from_label("Life Expectancy")
                                .selected_text(self.config.life_expectancy.to_string())
                                .show_ui(ui, |ui| {
                                    for year in 60..=120 {
                                        ui.selectable_value(
                                            &mut self.config.life_expectancy,
                                            year,
                                            year.to_string(),
                                        );
                                    }
                                });
                        } else if self.view == "Yearly" {
                            egui::ComboBox::from_label("Year")
                                .selected_text(self.selected_year.to_string())
                                .show_ui(ui, |ui| {
                                    for year in self.config.yearly_events.keys() {
                                        ui.selectable_value(
                                            &mut self.selected_year,
                                            *year,
                                            year.to_string(),
                                        );
                                    }
                                });
                        }
                    });
                });
            });

        egui::TopBottomPanel::bottom("legend")
            .min_height(bottom_height)
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Some(legend_item) =
                        draw_legend(ui, &self.config, &self.view, self.selected_year)
                    {
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

            egui::Window::new("Edit Legend Item").show(ctx, |ui| {
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
                            if chrono::NaiveDate::parse_from_str(
                                &format!("{}-01", start_date),
                                "%Y-%m-%d",
                            )
                            .is_ok()
                            {
                                item.start = start_date;
                                changed = true;
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Color:");
                        let mut color =
                            egui::Color32::from_hex(&item.color).unwrap_or(egui::Color32::WHITE);
                        if ui.color_edit_button_srgba(&mut color).changed() {
                            item.color =
                                format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b());
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

        fn calculate_centered_rect(available: egui::Rect, desired_size: egui::Vec2) -> egui::Rect {
            let size = egui::Vec2::new(
                desired_size.x.min(available.width()),
                desired_size.y.min(available.height()),
            );
            let pos = available.center() - (size / 2.0);
            egui::Rect::from_min_size(pos, size)
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_rect = ui.available_rect_before_wrap();
            let min_width = 800.0;
            let min_height = central_height.max(400.0);

            // Calculate the desired grid size
            let desired_grid_size = egui::vec2(min_width, min_height);
            let centered_rect = calculate_centered_rect(available_rect, desired_grid_size);
            // Create a new UI for our centered grid
            let grid_response = ui.allocate_ui_at_rect(centered_rect, |ui| {
                egui::ScrollArea::both()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(240, 240, 240))
                            .show(ui, |ui| {
                                if self.view == "Lifetime" {
                                    draw_lifetime_view(ui, centered_rect.size(), &self.config);
                                } else {
                                    draw_yearly_view(
                                        ui,
                                        centered_rect.size(),
                                        &self.config,
                                        self.selected_year,
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
                    }
                    Err(e) => {
                        log::error!("Failed to parse YAML content: {}. Using default config.", e);
                        app.config = load_config(DEFAULT_CONFIG_YAML);
                    }
                }
                ctx.request_repaint();
            }
            Err(e) => {
                log::error!("Invalid UTF-8 in file: {}. Using default config.", e);
                app.config = load_config(DEFAULT_CONFIG_YAML);
                ctx.request_repaint();
            }
        }
    }
}
