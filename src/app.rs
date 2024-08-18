use eframe::{egui, epaint::Vec2};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    date_of_birth: String,
    life_expectancy: u32,
    life_periods: Vec<LifePeriod>,
    yearly_events: HashMap<i32, Vec<YearlyEvent>>,
}

#[derive(Serialize, Deserialize)]
struct LifePeriod {
    name: String,
    start: String,
    color: String,
}

#[derive(Serialize, Deserialize)]
struct YearlyEvent {
    name: String,
    start: String,
    color: String,
}

// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    config: Config,
    view: String,
    selected_year: i32,
    yaml_files: Vec<String>,
    selected_yaml: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let yaml_files = get_yaml_files_in_data_folder();
        let default_yaml = "default.yaml".to_string();
        let config: Config = load_config(&default_yaml);
        Self {
            config,
            view: "Lifetime".to_string(),
            selected_year: 2024,
            yaml_files,
            selected_yaml: default_yaml,
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn draw_lifetime_view(&self, ui: &mut egui::Ui, grid_size: Vec2) {
        let dob = NaiveDate::parse_from_str(&format!("{}-01", self.config.date_of_birth), "%Y-%m-%d")
            .expect("Invalid date_of_birth format in config. Expected YYYY-MM");

        let years = self.config.life_expectancy;
        let rows = (years + 3) / 4;
        let cols = 48;

        let cell_size = (grid_size.x.min(grid_size.y * cols as f32 / rows as f32) / cols as f32).floor();
        let grid_width = cell_size * cols as f32;
        let grid_height = cell_size * rows as f32;

        let offset = Vec2::new(
            (grid_size.x - grid_width) / 2.0,
            (grid_size.y - grid_height) / 2.0
        );

        for i in 0..rows {
            for j in 0..cols {
                let current_date = dob + chrono::Duration::days(((i * cols + j) * 30) as i64);
                let color = self.get_color_for_date(&current_date);
                let rect = egui::Rect::from_min_size(
                    ui.min_rect().min + offset + Vec2::new(j as f32 * cell_size, i as f32 * cell_size),
                    Vec2::new(cell_size, cell_size),
                );
                ui.painter().rect_filled(rect, 0.0, color);
                ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
            }
        }
    }
    
    fn draw_yearly_view(&self, ui: &mut egui::Ui, grid_size: Vec2) {
        if let Some(events) = self.config.yearly_events.get(&self.selected_year) {
            let rows = 13;
            let cols = 28;

            let cell_size = (grid_size.x.min(grid_size.y * cols as f32 / rows as f32) / cols as f32).floor();
            let grid_width = cell_size * cols as f32;
            let grid_height = cell_size * rows as f32;

            let offset = Vec2::new(
                (grid_size.x - grid_width) / 2.0,
                (grid_size.y - grid_height) / 2.0
            );

            for row in 0..rows {
                for col in 0..cols {
                    let day = row * cols + col + 1;
                    let color = if day <= 365 {
                        let date = NaiveDate::from_ymd_opt(self.selected_year, 1, 1).unwrap() + chrono::Duration::days(day as i64 - 1);
                        self.get_color_for_yearly_event(&date, events)
                    } else {
                        egui::Color32::GRAY
                    };
                    let rect = egui::Rect::from_min_size(
                        ui.min_rect().min + offset + Vec2::new(col as f32 * cell_size, row as f32 * cell_size),
                        Vec2::new(cell_size, cell_size),
                    );
                    ui.painter().rect_filled(rect, 0.0, color);
                    ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
                }
            }
        }
    }

    fn draw_legend(&self, ui: &mut egui::Ui) {
        ui.label("Legend:");
        ui.add_space(5.0);

        let items_per_row = 3;
        let _item_width = ui.available_width() / items_per_row as f32;

        if self.view == "Lifetime" {
            egui::Grid::new("legend_grid")
                .spacing([10.0, 5.0])
                .show(ui, |ui| {
                    for (index, period) in self.config.life_periods.iter().enumerate() {
                        let color = hex_to_color32(&period.color);
                        ui.horizontal(|ui| {
                            let (rect, _) = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                            ui.painter().rect_filled(rect, 0.0, color);
                            ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
                            ui.label(format!("{} (from {})", period.name, period.start));
                        });
                        if (index + 1) % items_per_row == 0 {
                            ui.end_row();
                        }
                    }
                });
        } else if let Some(events) = self.config.yearly_events.get(&self.selected_year) {
            egui::Grid::new("legend_grid")
                .spacing([10.0, 5.0])
                .show(ui, |ui| {
                    for (index, event) in events.iter().enumerate() {
                        let color = hex_to_color32(&event.color);
                        ui.horizontal(|ui| {
                            let (rect, _) = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                            ui.painter().rect_filled(rect, 0.0, color);
                            ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
                            ui.label(format!("{} (from {})", event.name, event.start));
                        });
                        if (index + 1) % items_per_row == 0 {
                            ui.end_row();
                        }
                    }
                });
        }
    }

    fn get_color_for_date(&self, date: &NaiveDate) -> egui::Color32 {
        for period in self.config.life_periods.iter().rev() {
            let start = NaiveDate::parse_from_str(&format!("{}-01", period.start), "%Y-%m-%d")
                .unwrap_or_else(|e| panic!("Failed to parse start date '{}' for period '{}': {:?}", period.start, period.name, e));
            if &start <= date {
                return hex_to_color32(&period.color);
            }
        }
        egui::Color32::WHITE
    }

    fn get_color_for_yearly_event(&self, date: &NaiveDate, events: &[YearlyEvent]) -> egui::Color32 {
        for event in events.iter().rev() {
            let start = NaiveDate::parse_from_str(&event.start, "%Y-%m-%d")
                .unwrap_or_else(|e| panic!("Failed to parse start date '{}' for event '{}': {:?}", event.start, event.name, e));
            if &start <= date {
                return hex_to_color32(&event.color);
            }
        }
        egui::Color32::WHITE
    }
}
impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.config.name);
            
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("YAML File")
                    .selected_text(&self.selected_yaml)
                    .show_ui(ui, |ui| {
                        for yaml_file in &self.yaml_files {
                            if ui.selectable_value(&mut self.selected_yaml, yaml_file.clone(), yaml_file).changed() {
                                self.config = load_config(&self.selected_yaml);
                            }
                        }
                    });

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
            
            ui.add_space(20.0);

            let available_size = ui.available_size();
            let grid_size = Vec2::new(
                available_size.x.min(800.0),
                (available_size.y - 150.0).min(600.0), // Reserve space for legend and controls
            );

            egui::Frame::none()
                .fill(egui::Color32::from_rgb(240, 240, 240))
                .show(ui, |ui| {
                    if self.view == "Lifetime" {
                        self.draw_lifetime_view(ui, grid_size);
                    } else {
                        self.draw_yearly_view(ui, grid_size);
                    }
                });

            ui.add_space(20.0);
            self.draw_legend(ui);
        });
    }
}

fn hex_to_color32(hex: &str) -> egui::Color32 {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    egui::Color32::from_rgb(r, g, b)
}

fn get_yaml_files_in_data_folder() -> Vec<String> {
    let data_folder = Path::new("data");
    fs::read_dir(data_folder)
        .expect("Failed to read data folder")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "yaml" {
                Some(path.file_name()?.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect()
}

fn load_config(yaml_file: &str) -> Config {
    let yaml_path = Path::new("data").join(yaml_file);
    let yaml_content = fs::read_to_string(yaml_path)
        .unwrap_or_else(|_| panic!("Failed to read YAML file: {}", yaml_file));
    serde_yaml::from_str(&yaml_content)
        .unwrap_or_else(|_| panic!("Failed to parse YAML file: {}", yaml_file))
}
