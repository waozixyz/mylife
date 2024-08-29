mod bottom_panel;
mod central_panel;
mod legend;
mod lifetime_view;
mod top_panel;
mod events_view;

pub use bottom_panel::draw_bottom_panel;
pub use central_panel::draw_central_panel;
pub use legend::draw_legend;
pub use lifetime_view::draw_lifetime_view;
pub use top_panel::draw_top_panel;
pub use events_view::draw_event_view;