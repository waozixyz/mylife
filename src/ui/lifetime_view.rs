use dioxus::prelude::*;
use crate::models::{MyLifeApp, LifePeriod};
use chrono::{NaiveDate, Duration, Local};
use uuid::Uuid;
use dioxus_logger::tracing::{info, error, debug};

#[derive(PartialEq, Clone)]
struct CellData {
    color: String,
    period: Option<LifePeriod>,
    date: NaiveDate,
}

#[component]
pub fn LifetimeView(on_period_click: EventHandler<Uuid>) -> Element {
    let app_state = use_context::<Signal<MyLifeApp>>();
    let mut hovered_period = use_signal(|| None::<Uuid>);

    let years = app_state().config.life_expectancy;
    let rows: u32 = (years + 3) / 4;
    let cols: u32 = 48;

    let cell_size: u32 = 16; 
    let width: u32 = cols * cell_size;
    let height: u32 = rows * cell_size;

    let cell_data = use_memo(move || {
        let dob = NaiveDate::parse_from_str(&format!("{}-01", app_state().config.date_of_birth), "%Y-%m-%d")
            .expect("Invalid date_of_birth format in config. Expected YYYY-MM");
        
        let current_date = Local::now().date_naive();

        (0..rows * cols).map(|index| {
            let year = index / 12;
            let month = index % 12;
            let cell_date = dob + Duration::days((year * 365 + month * 30) as i64);
            let (color, period) = get_color_and_period_for_date(cell_date, &app_state().config.life_periods, current_date);
            
            CellData {
                color,
                period,
                date: cell_date,
            }
        }).collect::<Vec<_>>()
    });

    let mut handle_mouse_enter = move |period_id: Option<Uuid>| {
        hovered_period.set(period_id);
    };

    let mut handle_mouse_leave = move |_| {
        hovered_period.set(None);
    };

    rsx! {
        svg {
            width: "{width}",
            height: "{height}",
            view_box: "0 0 {width} {height}",
            onmouseleave: handle_mouse_leave,

            {cell_data().iter().enumerate().map(|(index, cell)| {
                let index = index as u32;
                let row = index / cols;
                let col = index % cols;
                let x = col * cell_size;
                let y = row * cell_size;
                let is_hovered = cell.period.as_ref().map_or(false, |p| Some(p.id) == (*hovered_period)());

                rsx! {
                    rect {
                        key: "{cell.date}",
                        x: "{x}",
                        y: "{y}",
                        width: "{cell_size}",
                        height: "{cell_size}",
                        fill: "{cell.color}",
                        stroke: if is_hovered { "#c800c8" } else { "gray" },
                        stroke_width: if is_hovered { "2" } else { "1" },
                        onclick: {
                            let period = cell.period.clone();
                            let on_period_click = on_period_click.clone();
                            move |_| {
                                if let Some(period) = &period {
                                    if !period.events.is_empty() {
                                        on_period_click.call(period.id);
                                    }
                                }
                            }
                        },
                        onmouseenter: {
                            let period_id = cell.period.as_ref().map(|p| p.id);
                            move |_| hovered_period.set(period_id)
                        },
                    }
                }
            })}
        }
    }
}

fn get_color_and_period_for_date(date: NaiveDate, life_periods: &[LifePeriod], current_date: NaiveDate) -> (String, Option<LifePeriod>) {
    debug!("Searching for period for date: {}", date);

    for (index, period) in life_periods.iter().rev().enumerate() {
        let period_start = match NaiveDate::parse_from_str(&format!("{}-01", period.start), "%Y-%m-%d") {
            Ok(date) => date,
            Err(e) => {
                error!("Failed to parse start date for period: {}. Error: {}", period.start, e);
                continue;
            }
        };
        
        let period_end = if index == 0 {
            // This is the last period, use current date as end
            current_date
        } else {
            // Use the start of the next period as the end of this period
            match NaiveDate::parse_from_str(&format!("{}-01", life_periods[life_periods.len() - index].start), "%Y-%m-%d") {
                Ok(date) => date,
                Err(e) => {
                    error!("Failed to parse start date for next period: {}. Error: {}", life_periods[life_periods.len() - index].start, e);
                    continue;
                }
            }
        };
        
        if date >= period_start && date < period_end {
            debug!("Found matching period: start={}, end={}, color={}", period_start, period_end, period.color);
            return (period.color.clone(), Some(period.clone()));
        }
    }

    ("#fafafa".to_string(), None)
}