use crate::models::{RuntimeConfig, RuntimeLifePeriod};
use crate::utils::color_utils::get_color_for_date;
use chrono::NaiveDate;
use eframe::egui;
use eframe::epaint::Vec2;
use uuid::Uuid;

pub fn draw_lifetime_view(
    ui: &mut egui::Ui,
    grid_size: Vec2,
    config: &RuntimeConfig,
    selected_life_period: &mut Option<Uuid>
) -> Option<Uuid> {
    let dob = NaiveDate::parse_from_str(&format!("{}-01", config.date_of_birth), "%Y-%m-%d")
        .expect("Invalid date_of_birth format in config. Expected YYYY-MM");

    let years = config.life_expectancy;
    let (rows, cols) = if grid_size.y > grid_size.x {
        ((years + 1) / 2, 24)
    } else {
        ((years + 3) / 4, 48)
    };

    let cell_size = (grid_size.x / cols as f32)
        .min(grid_size.y / rows as f32)
        .floor();

    let grid_width = cell_size * cols as f32;
    let grid_height = cell_size * rows as f32;

    let offset = Vec2::new(
        (grid_size.x - grid_width) / 2.0,
        (grid_size.y - grid_height) / 2.0
    );

    let grid_rect = egui::Rect::from_min_size(
        ui.min_rect().min + offset,
        Vec2::new(grid_width, grid_height),
    );

    let mut clicked_period = None;
    let current_date = chrono::Local::now().date_naive();
    let mut hovered_period: Option<&RuntimeLifePeriod> = None;

    // First pass: Draw cells and detect hover
    for i in 0..rows {
        for j in 0..cols {
            let cell_date = dob + chrono::Duration::days(((i * cols + j) * 30) as i64);
            let rect = egui::Rect::from_min_size(
                grid_rect.min + Vec2::new(j as f32 * cell_size, i as f32 * cell_size),
                Vec2::new(cell_size, cell_size),
            );

            if cell_date <= current_date {
                let (color, period) = get_color_and_period_for_date(&cell_date, &config.life_periods);
                let response = ui.put(rect, egui::Button::new("").fill(color));
                
                if response.clicked() {
                    if let Some(period) = period {
                        if !period.events.is_empty() {
                            clicked_period = Some(period.id);
                        }
                    }
                }

                if response.hovered() {
                    hovered_period = period;
                }
            } else {
                ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);
            }
            
            ui.painter()
                .rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
        }
    }
    // Second pass: Draw hover effect for the entire life period
    if let Some(hovered_period) = hovered_period {
        // Only proceed if the hovered period has events
        if !hovered_period.events.is_empty() {
            for i in 0..rows {
                let mut period_start_col = None;
                let mut period_end_col = None;

                for j in 0..cols {
                    let cell_date = dob + chrono::Duration::days(((i * cols + j) * 30) as i64);
                    if cell_date <= current_date {
                        let (_, period) = get_color_and_period_for_date(&cell_date, &config.life_periods);
                        if let Some(period) = period {
                            if period.id == hovered_period.id {
                                if period_start_col.is_none() {
                                    period_start_col = Some(j);
                                }
                                period_end_col = Some(j);
                            } else if period_start_col.is_some() {
                                draw_period_borders(
                                    ui,
                                    i as usize,
                                    period_start_col.unwrap() as usize,
                                    period_end_col.unwrap() as usize,
                                    cols as usize,
                                    rows as usize,
                                    cell_size,
                                    grid_rect,
                                    dob,
                                    config,
                                    hovered_period
                                );
                                period_start_col = None;
                                period_end_col = None;
                            }
                        }
                    }
                }

                if let (Some(start), Some(end)) = (period_start_col, period_end_col) {
                    draw_period_borders(
                        ui,
                        i as usize,
                        start as usize,
                        end as usize,
                        cols as usize,
                        rows as usize,
                        cell_size,
                        grid_rect,
                        dob,
                        config,
                        hovered_period
                    );
                }
            }
        }
    }

    if let Some(period_id) = clicked_period {
        *selected_life_period = Some(period_id);
    }

    clicked_period
}

fn draw_period_borders(
    ui: &mut egui::Ui,
    row: usize,
    start_col: usize,
    end_col: usize,
    total_cols: usize,
    total_rows: usize,
    cell_size: f32,
    grid_rect: egui::Rect,
    dob: NaiveDate,
    config: &RuntimeConfig,
    hovered_period: &RuntimeLifePeriod,
) {
    let border_color = egui::Color32::from_rgb(200, 0, 200);
    let border_thickness = 2.0;

    let top_left = grid_rect.min + Vec2::new(start_col as f32 * cell_size, row as f32 * cell_size);
    let top_right = top_left + Vec2::new((end_col - start_col + 1) as f32 * cell_size, 0.0);
    let bottom_left = top_left + Vec2::new(0.0, cell_size);
    let bottom_right = bottom_left + Vec2::new((end_col - start_col + 1) as f32 * cell_size, 0.0);

    // Top border
    ui.painter().line_segment([top_left, top_right], (border_thickness, border_color));

    // Bottom border
    ui.painter().line_segment([bottom_left, bottom_right], (border_thickness, border_color));

    // Left border
    if start_col == 0 || (start_col > 0 && get_color_and_period_for_date(&(dob + chrono::Duration::days(((row * total_cols + start_col - 1) * 30) as i64)), &config.life_periods).1.map_or(true, |p| p.id != hovered_period.id)) {
        ui.painter().line_segment([top_left, bottom_left], (border_thickness, border_color));
    }

    // Right border
    if end_col == total_cols - 1 || (end_col < total_cols - 1 && get_color_and_period_for_date(&(dob + chrono::Duration::days(((row * total_cols + end_col + 1) * 30) as i64)), &config.life_periods).1.map_or(true, |p| p.id != hovered_period.id)) {
        ui.painter().line_segment([top_right, bottom_right], (border_thickness, border_color));
    }

    // Bottom border for the last row
    if row == total_rows - 1 && end_col == total_cols - 1 {
        ui.painter().line_segment([bottom_left, bottom_right], (border_thickness, border_color));
    }
}

fn get_color_and_period_for_date<'a>(date: &NaiveDate, life_periods: &'a [RuntimeLifePeriod]) -> (egui::Color32, Option<&'a RuntimeLifePeriod>) {
    let mut current_period = None;
    for period in life_periods.iter().rev() {
        let period_start = NaiveDate::parse_from_str(&format!("{}-01", period.start), "%Y-%m-%d")
            .expect("Invalid start date format in life period");
        if date >= &period_start {
            current_period = Some(period);
            break;
        }
    }
    match current_period {
        Some(period) => (egui::Color32::from_hex(&period.color).unwrap_or(egui::Color32::GRAY), Some(period)),
        None => (egui::Color32::GRAY, None),
    }
}