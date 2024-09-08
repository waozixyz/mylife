use dioxus::prelude::*;
use crate::models::{LifePeriod, Yaml};
use chrono::{NaiveDate, Duration, Local};
use uuid::Uuid;
use dioxus_logger::tracing::{error, debug};

#[derive(PartialEq, Clone)]
struct CellData {
    color: String,
    period: Option<LifePeriod>,
    date: NaiveDate,
}

#[component]
pub fn LifetimeView(on_period_click: EventHandler<Uuid>) -> Element {
    let yaml_state = use_context::<Signal<Yaml>>();
    let mut hovered_period = use_signal(|| None::<Uuid>);

    let years = yaml_state().life_expectancy;
    let cols = 48;
    let rows = (years + 3) / 4; 

    let cell_data = use_memo(move || {
        let dob = NaiveDate::parse_from_str(&format!("{}-01", yaml_state().date_of_birth), "%Y-%m-%d")
            .expect("Invalid date_of_birth format in yaml. Expected YYYY-MM");
        
        let current_date = Local::now().date_naive();

        (0..years * 12).map(|index| {
            let year = index / 12;
            let month = index % 12;
            let cell_date = dob + Duration::days((year * 365 + month * 30) as i64);
            let (color, period) = get_color_and_period_for_date(cell_date, &yaml_state().life_periods, current_date);
            
            CellData {
                color,
                period,
                date: cell_date,
            }
        }).collect::<Vec<_>>()
    });


    let handle_mouse_leave = move |_| {
        hovered_period.set(None);
    };

    rsx! {
        div {
            class: "lifetime-view-container",
            svg {
                class: "lifetime-view-svg",
                preserve_aspect_ratio: "xMidYMid meet",
                view_box: "0 0 {cols} {rows}",
                onmouseleave: handle_mouse_leave,

                {cell_data().iter().enumerate().map(|(index, cell)| {
                    let row = index / cols;
                    let col = index % cols;
                    let is_hovered = cell.period.as_ref().map_or(false, |p| Some(p.id) == (*hovered_period)());

                    rsx! {
                        rect {
                            key: "{cell.date}",
                            x: "{col}",
                            y: "{row}",
                            width: "1",
                            height: "1",
                            fill: "{cell.color}",
                            stroke: if is_hovered { "#c800c8" } else { "gray" },
                            stroke_width: if is_hovered { "0.05" } else { "0.02" },
                            onclick: {
                                let period = cell.period.clone();
                                let on_period_click = on_period_click;
                                move |_| {
                                    if let Some(period) = &period {
                                        on_period_click.call(period.id);
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