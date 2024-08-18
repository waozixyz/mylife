use eframe::{egui, epaint::Vec2};
use crate::config::Config;
use crate::ui::{draw_lifetime_view, draw_yearly_view, draw_legend};
use crate::utils::{load_config, get_yaml_files_in_data_folder};
#[cfg(target_arch = "wasm32")]
use crate::config::DEFAULT_CONFIG_YAML;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
pub struct MyLifeApp {
    config: Config,
    view: String,
    selected_year: i32,
    #[cfg(not(target_arch = "wasm32"))]
    yaml_files: Vec<String>,
    #[cfg(target_arch = "wasm32")]
    yaml_content: String,
    selected_yaml: String,
    #[serde(skip)]
    value: f32,
}

impl Default for MyLifeApp {
    fn default() -> Self {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let config = load_config(DEFAULT_CONFIG_YAML);
            } else {
                let yaml_files = get_yaml_files_in_data_folder();
                let default_yaml = "default.yaml".to_string();
                let config: Config = load_config(&default_yaml);
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
}

impl eframe::App for MyLifeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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

        egui::CentralPanel::default().show(ctx, |ui| {
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
            
            ui.add_space(20.0);

            let available_size = ui.available_size();
            let grid_size = Vec2::new(
                available_size.x.min(800.0),
                (available_size.y - 150.0).min(600.0),
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

            ui.add_space(20.0);
            draw_legend(ui, &self.config, &self.view, self.selected_year);
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