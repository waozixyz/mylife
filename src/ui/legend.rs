use dioxus::prelude::*;
use crate::models::{MyLifeApp, LegendItem};

#[component]
pub fn Legend() -> Element {
    // Fetch signals from context
    let mut app_state = use_context::<Signal<MyLifeApp>>();

    let mut open_edit_modal = move |item: LegendItem| {
        app_state.write().item_state = Some(item.clone());
        app_state.write().temp_start_date = item.start.clone();
    };

    // Directly handle rendering without `legend_items` variable
    let legend_items = {
        let mut legend_items = Vec::new();
        match app_state().view.as_str() {
            "Lifetime" => {
                let mut sorted_periods = app_state().config.life_periods.clone();
                sorted_periods.sort_by(|a, b| a.start.cmp(&b.start));

                for period in sorted_periods {
                    let item = LegendItem {
                        id: period.id,
                        name: period.name,
                        start: period.start,
                        color: period.color,
                        is_event: false,
                    };
                    legend_items.push(rsx! {
                        div {
                            key: "{item.id}",
                            class: "legend-item",
                            style: "display: flex; align-items: center; height: 20px; cursor: pointer; background-color: {item.color};",
                            onclick: move |_| open_edit_modal(item.clone()),
                            div {
                                class: "legend-item-text",
                                style: "color: black; text-align: center; width: 100%;",
                                "{item.name} ({item.start})"
                            }
                        }
                    });
                }
            },
            "EventView" => {
                if let Some(period_id) = app_state().selected_life_period {
                    if let Some(period) = app_state().config.life_periods.iter().find(|p| p.id == period_id) {
                        for event in &period.events {
                            let item = LegendItem {
                                id: event.id,
                                name: event.name.clone(),
                                start: event.start.clone(),
                                color: event.color.clone(),
                                is_event: true,
                            };
                            legend_items.push(rsx! {
                                div {
                                    key: "{item.id}",
                                    class: "legend-item",
                                    style: "display: flex; align-items: center; height: 20px; cursor: pointer; background-color: {item.color};",
                                    onclick: move |_| open_edit_modal(item.clone()),
                                    div {
                                        class: "legend-item-text",
                                        style: "color: black; text-align: center; width: 100%;",
                                        "{item.name} ({item.start})"
                                    }
                                }
                            });
                        }
                    }
                }
            },
            _ => {}
        }

        legend_items
    };

    rsx! {
        div {
            class: "legend",
            style: "display: flex; flex-direction: column; gap: 5px;",
            { legend_items.into_iter() }
        }
    }
}