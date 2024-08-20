use eframe::egui;

pub fn calculate_centered_rect(available: egui::Rect, desired_size: egui::Vec2) -> egui::Rect {
    let size = egui::Vec2::new(
        desired_size.x.min(available.width()),
        desired_size.y.min(available.height()),
    );
    let pos = available.center() - (size / 2.0);
    egui::Rect::from_min_size(pos, size)
}
