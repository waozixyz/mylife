use crate::models::{CatppuccinTheme, LegendItem, MyLifeApp};
use crate::ui::{draw_bottom_panel, draw_central_panel, draw_top_panel};

use crate::utils::color_utils::{color32_to_hex, hex_to_color32};
use crate::utils::config_utils::save_config;
use crate::utils::date_utils::is_valid_date;

#[cfg(target_arch = "wasm32")]
use crate::utils::config_utils::get_default_config;

#[cfg(not(target_arch = "wasm32"))]
use crate::utils::config_utils::{get_available_configs, get_config};

use catppuccin_egui::{FRAPPE, LATTE, MACCHIATO, MOCHA};
use eframe::egui;
use log::debug;

#[cfg(target_arch = "wasm32")]
use crate::config::DEFAULT_CONFIG_YAML;
#[cfg(target_arch = "wasm32")]
use crate::utils::wasm_config::NEW_CONFIG;

impl Default for MyLifeApp {
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let default_config = get_config();
        #[cfg(target_arch = "wasm32")]
        let default_config = get_default_config();

        Self {
            config: default_config.clone(),
            view: "Lifetime".to_string(),
            selected_year: 2024,
            #[cfg(not(target_arch = "wasm32"))]
            yaml_files: get_available_configs(),
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
            #[cfg(target_arch = "wasm32")]
            loaded_configs: vec![("Default".to_string(), default_config)],
            #[cfg(target_arch = "wasm32")]
            selected_config_index: 0,
            #[cfg(target_arch = "wasm32")]
            loaded_app: None,
            #[cfg(target_arch = "wasm32")]
            loaded_config: None,
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
                    event.color = item.name.clone();
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
        save_config(&self.config, &self.selected_yaml);
    }
}

impl eframe::App for MyLifeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        log::info!("Entering update method. Current name: {}", self.config.name);

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(new_config) = NEW_CONFIG.lock().unwrap().take() {
                self.config = new_config;
                self.selected_yaml = "Loaded YAML".to_string();
                self.loaded_configs = vec![("Loaded YAML".to_string(), self.config.clone())];
                self.selected_config_index = 0;
                log::info!("App updated. Current name: {}", self.config.name);
            }
        }

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

        draw_top_panel(self, ctx, top_height);
        draw_bottom_panel(self, ctx, bottom_height);

        if let Some(mut item) = self.selected_legend_item.clone() {
            let mut should_close = false;

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
                        if ui.text_edit_singleline(&mut start_date).changed()
                            && is_valid_date(&start_date)
                        {
                            item.start = start_date;
                            changed = true;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Color:");
                        let mut color = hex_to_color32(&item.color);
                        if ui.color_edit_button_srgba(&mut color).changed() {
                            item.color = color32_to_hex(color);
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
        draw_central_panel(self, ctx, central_height);
    }
}
