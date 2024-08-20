use crate::models::{
    CatppuccinTheme, LegendItem, MyLifeApp, RuntimeLifePeriod, RuntimeYearlyEvent,
};

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
            temp_start_date: "".to_string(),
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
            let events = self
                .config
                .yearly_events
                .entry(self.selected_year)
                .or_insert_with(Vec::new);
            if let Some(event) = events.iter_mut().find(|e| e.id == item.id) {
                event.name = item.name.clone();
                event.color = item.color.clone();
                event.start = item.start.clone();
            } else {
                // This is a new item
                events.push(RuntimeYearlyEvent {
                    id: item.id,
                    name: item.name.clone(),
                    color: item.color.clone(),
                    start: item.start.clone(),
                });
            }
        } else {
            if let Some(period) = self
                .config
                .life_periods
                .iter_mut()
                .find(|p| p.id == item.id)
            {
                period.name = item.name.clone();
                period.start = item.start.clone();
                period.color = item.color.clone();
            } else {
                // This is a new item
                self.config.life_periods.push(RuntimeLifePeriod {
                    id: item.id,
                    name: item.name.clone(),
                    start: item.start.clone(),
                    color: item.color.clone(),
                });
            }
        }
        save_config(&self.config, &self.selected_yaml);
    }
}

impl eframe::App for MyLifeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(new_config) = NEW_CONFIG.lock().unwrap().take() {
                self.config = new_config;
                self.selected_yaml = "Loaded YAML".to_string();
                self.loaded_configs = vec![("Loaded YAML".to_string(), self.config.clone())];
                self.selected_config_index = 0;
                println!("App updated. Current name: {}", self.config.name);
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
            let mut changed = false;

            if self.original_legend_item.is_none() {
                self.original_legend_item = Some(item.clone());
                self.temp_start_date = item.start.clone();
                println!("Initializing temp_start_date: {}", self.temp_start_date);
            }

            egui::Window::new("Edit Legend Item").show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        if ui.text_edit_singleline(&mut item.name).changed() {
                            changed = true;
                            println!("Name changed to: {}", item.name);
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Start:");
                        let response = ui.text_edit_singleline(&mut self.temp_start_date);

                        if response.changed() {
                            println!(
                                "Date input changed. Current value: {}",
                                self.temp_start_date
                            );
                            if is_valid_date(&self.temp_start_date, !item.is_yearly) {
                                item.start = self.temp_start_date.clone();
                                changed = true;
                                println!(
                                    "Valid date entered. Updated item.start to: {}",
                                    item.start
                                );
                            } else {
                                println!("Invalid date entered. Not updating item.start.");
                            }
                        }

                        if response.lost_focus() {
                            println!(
                                "Date input lost focus. Current value: {}",
                                self.temp_start_date
                            );
                            if is_valid_date(&self.temp_start_date, !item.is_yearly) {
                                item.start = self.temp_start_date.clone();
                                changed = true;
                                println!(
                                    "Valid date on lost focus. Updated item.start to: {}",
                                    item.start
                                );
                            } else {
                                println!("Invalid date on lost focus. Not updating item.start.");
                            }
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

                    ui.horizontal(|ui| {
                        if ui.button("Close").clicked() {
                            should_close = true;
                        }
                    });
                });
            });

            if changed {
                println!(
                    "Item changed, updating config. Current start date: {}",
                    item.start
                );
                self.update_config_item(&item);
            }

            if should_close {
                println!(
                    "Closing edit window. Final temp_start_date: {}",
                    self.temp_start_date
                );
                if is_valid_date(&self.temp_start_date, !item.is_yearly) {
                    item.start = self.temp_start_date.clone();
                    self.update_config_item(&item);
                    println!("Valid date on close. Final item.start: {}", item.start);
                } else {
                    println!("Invalid date on close. Not updating item.start.");
                }
                self.selected_legend_item = None;
                self.original_legend_item = None;
                self.temp_start_date.clear();
            } else {
                self.selected_legend_item = Some(item);
            }
        }
        draw_central_panel(self, ctx, central_height);
    }
}
