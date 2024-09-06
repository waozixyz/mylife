use dioxus::prelude::*;
use crate::models::{MyLifeApp, RuntimeConfig, RuntimeLifePeriod};
use chrono::{NaiveDate, Duration, Local};
use uuid::Uuid;
use std::rc::Rc;
use std::cell::RefCell;


#[component]
pub fn LifetimeView(on_period_click: EventHandler<Uuid>) -> Element {
    let mut app_state = use_context::<Signal<MyLifeApp>>();
    
    let dob = NaiveDate::parse_from_str(&format!("{}-01", app_state().config.date_of_birth), "%Y-%m-%d")
        .expect("Invalid date_of_birth format in config. Expected YYYY-MM");

    let years = app_state().config.life_expectancy;
    let cols = 48;  // 4 years * 12 months
    let rows = (years + 3) / 4;  // Ceiling division to get number of rows

    let current_date = Local::now().date_naive();

    rsx! {
        div {
            class: "lifetime-view",

            {(0..rows * cols).map(|index| {
                let year = index / 12;
                let month = index % 12;
                let cell_date = dob + Duration::days((year * 365 + month * 30) as i64);
                let (color, period) = get_color_and_period_for_date(cell_date, app_state().config.life_periods.clone());
                let period = Rc::new(RefCell::new(period));

                let is_hovered = period.borrow().as_ref().map_or(false, |p| Some(p.id) == app_state().hovered_period);
                let cell_class = if is_hovered { "lifetime-cell hovered" } else { "lifetime-cell" };
                rsx! {
                    div {
                        key: "{year}-{month}",
                        class: "{cell_class}",
                        onclick: {
                            let period = Rc::clone(&period);
                            move |_| {
                                if let Some(period) = period.borrow().as_ref() {
                                    if !period.events.is_empty() {
                                        on_period_click.call(period.id);
                                    }
                                }
                            }
                        },
                        onmouseenter: {
                            let period = Rc::clone(&period);
                            move |_| {
                                if let Some(period) = period.borrow().as_ref() {
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

fn get_color_and_period_for_date(date: NaiveDate, life_periods: Vec<RuntimeLifePeriod>) -> (String, Option<RuntimeLifePeriod>) {
    // Convert the vector into an iterator
    life_periods.into_iter().rev()
        .find(|period| {
            // Parse the period start date
            let period_start = NaiveDate::parse_from_str(&format!("{}-01", period.start), "%Y-%m-%d")
                .expect("Invalid start date format in life period");
            // Check if the date is greater than or equal to the period start date
            date >= period_start
        })
        .map(|period| (period.color.clone(), Some(period)))
        .unwrap_or(("#808080".to_string(), None))
}