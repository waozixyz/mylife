use crate::models::{
    CatppuccinTheme, LegendItem, MyLifeApp, RuntimeLifePeriod, RuntimeLifePeriodEvent
};
use crate::ui::{draw_bottom_panel, draw_central_panel, draw_top_panel};
use crate::utils::color_utils::{color32_to_hex, hex_to_color32};
use crate::utils::config_utils::{save_config, get_available_configs, get_config};
use crate::utils::date_utils::is_valid_date;

use catppuccin_egui::{FRAPPE, LATTE, MACCHIATO, MOCHA};
use eframe::egui;

#[cfg(target_arch = "wasm32")]
use crate::utils::config_utils::load_config_async;

impl Default for MyLifeApp {
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let default_config = get_config();
        #[cfg(target_arch = "wasm32")]
        let default_config = get_config();

        Self {
            config: default_config.clone(),
            view: "Lifetime".to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            yaml_files: get_available_configs(),
            #[cfg(not(target_arch = "wasm32"))]
            selected_yaml: "default.yaml".to_string(),
            #[cfg(target_arch = "wasm32")]
            selected_yaml: "Default".to_string(),
            selected_legend_item: None,
            original_legend_item: None,
            theme: CatppuccinTheme::Mocha,
            #[cfg(target_arch = "wasm32")]
            loaded_configs: get_available_configs(),
            #[cfg(target_arch = "wasm32")]
            selected_config_index: 0,
            temp_start_date: "".to_string(),
            selected_life_period: None,
            value: 0.0,
            #[cfg(target_arch = "wasm32")]
            loaded_app: None,
            #[cfg(target_arch = "wasm32")]
            loaded_config: None,
            #[cfg(target_arch = "wasm32")]
            yaml_content: String::new(),
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
        if item.is_event {
            if let Some(period) = self.config.life_periods.iter_mut().find(|p| p.id == self.selected_life_period.unwrap()) {
                if let Some(event) = period.events.iter_mut().find(|e| e.id == item.id) {
                    event.name = item.name.clone();
                    event.color = item.color.clone();
                    event.start = item.start.clone();
                } else {
                    period.events.push(RuntimeLifePeriodEvent {
                        id: item.id,
                        name: item.name.clone(),
                        color: item.color.clone(),
                        start: item.start.clone(),
                    });
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
        } else {
            self.config.life_periods.push(RuntimeLifePeriod {
                id: item.id,
                name: item.name.clone(),
                start: item.start.clone(),
                color: item.color.clone(),
                events: Vec::new(),
            });
        }
        save_config(&self.config, &self.selected_yaml);
    }
    #[cfg(target_arch = "wasm32")]
    fn load_custom_config(&mut self) {
        let future = {
            let mut app = self.clone();
            async move {
                if let Some(config) = load_config_async().await {
                    app.loaded_config = Some(config);
                }
            }
        };
        wasm_bindgen_futures::spawn_local(future);
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

        draw_top_panel(self, ctx, top_height);
        draw_central_panel(self, ctx, top_height, bottom_height);
        draw_bottom_panel(self, ctx, bottom_height);

        #[cfg(target_arch = "wasm32")]
        {
            egui::Window::new("Config Selection").show(ctx, |ui| {
                egui::ComboBox::from_label("Select Config")
                    .selected_text(&self.loaded_configs[self.selected_config_index].0)
                    .show_ui(ui, |ui| {
                        for (i, (name, _)) in self.loaded_configs.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_config_index, i, name);
                        }
                    });

                if ui.button("Load Selected Config").clicked() {
                    self.config = self.loaded_configs[self.selected_config_index].1.clone();
                    self.selected_yaml = self.loaded_configs[self.selected_config_index].0.clone();
                }

                if ui.button("Load Custom Config").clicked() {
                    self.load_custom_config();
                }
            });

            if let Some(config) = self.loaded_config.take() {
                self.config = config;
                self.selected_yaml = "Custom".to_string();
            }
        }

        if let Some(mut item) = self.selected_legend_item.clone() {
            let mut should_close = false;
            let mut changed = false;

            if self.original_legend_item.is_none() {
                self.original_legend_item = Some(item.clone());
                self.temp_start_date = item.start.clone();
            }

            egui::Window::new("Edit Legend Item").show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        if ui.text_edit_singleline(&mut item.name).changed() {
                            changed = true;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Start:");
                        let response = ui.text_edit_singleline(&mut self.temp_start_date);

                        if response.changed() || response.lost_focus() {
                            if is_valid_date(&self.temp_start_date, !item.is_event) {
                                item.start = self.temp_start_date.clone();
                                changed = true;
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
                self.update_config_item(&item);
            }

            if should_close {
                if is_valid_date(&self.temp_start_date, !item.is_event) {
                    item.start = self.temp_start_date.clone();
                    self.update_config_item(&item);
                }
                self.selected_legend_item = None;
                self.original_legend_item = None;
                self.temp_start_date.clear();
            } else {
                self.selected_legend_item = Some(item);
            }
        }
    }
}