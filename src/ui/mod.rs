mod bottom_panel;
mod central_panel;
mod edit_legend_item;
mod events_view;
mod legend;
mod lifetime_view;
mod top_panel;

pub use bottom_panel::BottomPanel;
pub use central_panel::CentralPanel;
pub use edit_legend_item::EditLegendItem;
pub use events_view::EventView;
pub use legend::Legend;
#[cfg(not(target_arch = "wasm32"))]
pub use lifetime_view::get_svg_content;
pub use lifetime_view::LifetimeView;
pub use top_panel::TopPanel;
