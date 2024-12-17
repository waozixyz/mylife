use dioxus::prelude::*;

#[derive(Clone)]
pub struct DayInfo {
    pub name: &'static str,
    pub symbol: &'static str,
}

pub const DAYS: [DayInfo; 7] = [
    DayInfo {
        name: "Monday",
        symbol: "🌙",
    },
    DayInfo {
        name: "Tuesday",
        symbol: "♂",
    },
    DayInfo {
        name: "Wednesday",
        symbol: "☿",
    },
    DayInfo {
        name: "Thursday",
        symbol: "♃",
    },
    DayInfo {
        name: "Friday",
        symbol: "♀",
    },
    DayInfo {
        name: "Saturday",
        symbol: "♄",
    },
    DayInfo {
        name: "Sunday",
        symbol: "☉",
    },
];

#[component]
pub fn DayTabs(active_day: String, on_day_change: EventHandler<String>) -> Element {
    rsx! {
        div {
            class: "todo-tabs",
            { DAYS.iter().map(|day| {
                let is_active = day.name == active_day;
                rsx! {
                    button {
                        key: "{day.name}",
                        class: if is_active { "active" } else { "" },
                        onclick: move |_| on_day_change.call(day.name.to_string()),
                        span {
                            class: "symbol",
                            "{day.symbol}"
                        }
                        span {
                            class: "name",
                            "{day.name}"
                        }
                    }
                }
            }) }
        }
    }
}
