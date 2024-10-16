use crate::models::{CellData, LifePeriod, SizeInfo, Yaml};
use chrono::{Duration, Local, NaiveDate};
use dioxus::prelude::*;
use dioxus_logger::tracing::{debug, error};
use uuid::Uuid;

fn calculate_grid_dimensions(size_info: &SizeInfo, life_expectancy: u32) -> (usize, usize) {
    let is_landscape = size_info.window_width > size_info.window_height;
    let cols = if is_landscape { 48 } else { 24 };
    let rows = if is_landscape {
        (life_expectancy as usize + 7) / 4
    } else {
        (life_expectancy as usize + 3) / 2
    };
    (cols, rows)
}

fn calculate_cell_size(size_info: &SizeInfo, cols: usize, rows: usize) -> (f32, f32) {
    let is_landscape = size_info.window_width > size_info.window_height;
    let scale = 32.0;
    let gap_ratio = 0.1; // 10% of cell size for gap

    let cell_size = if is_landscape {
        size_info.window_height as f32 / rows as f32 / scale
    } else {
        size_info.window_width as f32 / cols as f32 / scale
    };

    let gap = cell_size * gap_ratio;
    (cell_size, gap)
}

fn generate_lifetime_data(
    yaml: &Yaml,
    size_info: &SizeInfo,
) -> (Vec<CellData>, usize, usize, f32, f32, f32, f32) {
    let years = yaml.life_expectancy;
    let (cols, rows) = calculate_grid_dimensions(size_info, years);
    let (cell_size, gap) = calculate_cell_size(size_info, cols, rows);

    let total_width = cols as f32 * (cell_size + gap) - gap;
    let total_height = rows as f32 * (cell_size + gap) - gap;

    let dob = parse_date(
        &yaml.date_of_birth,
        "Invalid date_of_birth format in yaml. Expected YYYY-MM",
    );
    let current_date = Local::now().date_naive();

    let cell_data: Vec<CellData> = (0..years * 12)
        .map(|index| {
            let year = index / 12;
            let month = index % 12;
            let cell_date = dob + Duration::days((year * 365 + month * 30) as i64);
            let (color, period) =
                get_color_and_period_for_date(cell_date, &yaml.life_periods, current_date);

            CellData {
                color,
                period,
                date: cell_date,
            }
        })
        .collect();

    (
        cell_data,
        cols,
        rows,
        cell_size,
        gap,
        total_width,
        total_height,
    )
}

#[component]
pub fn LifetimeView(on_period_click: EventHandler<Uuid>) -> Element {
    let yaml_state = use_context::<Signal<Yaml>>();
    let mut hovered_period = use_signal(|| None::<Uuid>);
    let size_info = use_context::<Signal<SizeInfo>>();

    let (cell_data, cols, _rows, cell_size, gap, total_width, total_height) =
        use_memo(move || generate_lifetime_data(&yaml_state(), &size_info()))();

    let handle_mouse_leave = move |_| {
        hovered_period.set(None);
    };

    rsx! {
        div {
            class: "lifetime-view-container",
            svg {
                class: "lifetime-view-svg",
                preserve_aspect_ratio: "xMinYMin meet",
                view_box: "0 0 {total_width} {total_height}",
                onmouseleave: handle_mouse_leave,

                {cell_data.iter().enumerate().map(|(index, cell)| {
                    let row = index / cols;
                    let col = index % cols;
                    let is_hovered = cell.period.as_ref().map_or(false, |p| p.id == (*hovered_period)());

                    let x = col as f32 * (cell_size + gap);
                    let y = row as f32 * (cell_size + gap);
                    rsx! {
                        rect {
                            key: "{cell.date}",
                            x: "{x}",
                            y: "{y}",
                            width: "{cell_size}",
                            height: "{cell_size}",
                            fill: "{cell.color}",
                            stroke: if is_hovered { "#c800c8" } else { "gray" },
                            stroke_width: if is_hovered { "0.05" } else { "0.02" },
                            onclick: {
                                let period = cell.period.clone();
                                let on_period_click = on_period_click;
                                move |_| {
                                    if let Some(period) = &period {
                                        on_period_click.call(period.id.unwrap_or_default());
                                    }
                                }
                            },
                            onmouseenter: {
                                let period_id = cell.period.as_ref().map(|p| p.id);
                                move |_| hovered_period.set(period_id.flatten())
                            },
                        }
                    }
                })}
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn generate_svg_content(yaml: &Yaml, size_info: &SizeInfo) -> String {
    let (cell_data, cols, _rows, cell_size, gap, total_width, total_height) =
        generate_lifetime_data(yaml, size_info);

    let mut svg = format!(
        r#"<svg class="lifetime-view-svg" preserveAspectRatio="xMidYMid meet" viewBox="0 0 {} {}">"#,
        total_width, total_height
    );

    for (index, cell) in cell_data.iter().enumerate() {
        let row = index / cols;
        let col = index % cols;
        let x = col as f32 * (cell_size + gap);
        let y = row as f32 * (cell_size + gap);

        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="gray" stroke-width="0.02"/>"#,
            x, y, cell_size, cell_size, cell.color
        ));
    }

    svg.push_str("</svg>");
    svg
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_svg_content() -> Option<String> {
    let yaml_state = use_context::<Signal<Yaml>>();
    let size_info = use_context::<Signal<SizeInfo>>();

    let yaml = yaml_state();
    let size_info = size_info();
    Some(generate_svg_content(&yaml, &size_info))
}

fn get_color_and_period_for_date(
    date: NaiveDate,
    life_periods: &[LifePeriod],
    current_date: NaiveDate,
) -> (String, Option<LifePeriod>) {
    debug!("Searching for period for date: {}", date);

    for (index, period) in life_periods.iter().rev().enumerate() {
        let period_start = parse_date(
            &period.start,
            &format!("Failed to parse start date for period: {}", period.start),
        );
        let period_end = if index == 0 {
            current_date
        } else {
            parse_date(
                &life_periods[life_periods.len() - index].start,
                &format!(
                    "Failed to parse start date for next period: {}",
                    life_periods[life_periods.len() - index].start
                ),
            )
        };

        if date >= period_start && date < period_end {
            debug!(
                "Found matching period: start={}, end={}, color={}",
                period_start, period_end, period.color
            );
            return (period.color.clone(), Some(period.clone()));
        }
    }

    ("#fafafa".to_string(), None)
}

fn parse_date(date_str: &str, error_msg: &str) -> NaiveDate {
    NaiveDate::parse_from_str(&format!("{}-01", date_str), "%Y-%m-%d").unwrap_or_else(|e| {
        error!("{}: {}", error_msg, e);
        panic!("{}", error_msg);
    })
}
