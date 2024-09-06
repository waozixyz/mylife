use dioxus::prelude::*;
use crate::models::{MyLifeApp, RuntimeLifePeriod};
use chrono::{NaiveDate, Duration, Local};
use uuid::Uuid;
use dioxus_logger::tracing::{info, error, debug};

#[component]
pub fn LifetimeView(on_period_click: EventHandler<Uuid>) -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    
    let dob = use_memo(move || {
        NaiveDate::parse_from_str(&format!("{}-01", app_state().config.date_of_birth), "%Y-%m-%d")
            .expect("Invalid date_of_birth format in config. Expected YYYY-MM")
    });

    let years = app_state().config.life_expectancy;
    let cols = 48;
    let rows = (years + 3) / 4;  

    let current_date = Local::now().date_naive();

    rsx! {
        div {
            class: "lifetime-view",

            {(0..rows * cols).map(|index| {
                let year = index / 12;
                let month = index % 12;
                let cell_date = (*dob)() + Duration::days((year * 365 + month * 30) as i64);
                let (color, period) = get_color_and_period_for_date(cell_date, &app_state().config.life_periods, current_date);

                let is_hovered = period.as_ref().map_or(false, |p| Some(p.id) == app_state().hovered_period);
                let cell_class = if is_hovered { "lifetime-cell hovered" } else { "lifetime-cell" };

                rsx! {
                    div {
                        key: "{year}-{month}",
                        class: "{cell_class}",
                        style: "background-color: {color};",
                        onclick: {
                            let period = period.clone();
                            move |_| {
                                if let Some(period) = &period {
                                    if !period.events.is_empty() {
                                        on_period_click.call(period.id);
                                    }
                                }
                            }
                        },
                        onmouseenter: {
                            let period = period.clone();
                            move |_|  {
                                if let Some(period) = &period {
                                    if !period.events.is_empty() {
                                        app_state.write().hovered_period = Some(period.id);
                                    }
                                }
                            }
                        },
                        onmouseleave: move |_| app_state.write().hovered_period = None,
                    }
                }
            })}
        }
    }
}

fn get_color_and_period_for_date(date: NaiveDate, life_periods: &[RuntimeLifePeriod], current_date: NaiveDate) -> (String, Option<RuntimeLifePeriod>) {
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
        
        debug!("Checking period: start={}, end={}, color={}", period_start, period_end, period.color);
        
        if date >= period_start && date < period_end {
            debug!("Found matching period: start={}, end={}, color={}", period_start, period_end, period.color);
            return (period.color.clone(), Some(period.clone()));
        }
    }

    ("#fafafa".to_string(), None)
}