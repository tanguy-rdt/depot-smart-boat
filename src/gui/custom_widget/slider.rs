use eframe::{egui::{self, InputState}, epaint::Color32};

fn slidebar_ui(ui: &mut egui::Ui, value: &mut f64) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(10.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());
    if response.dragged() {
        if let Some(pointer_position) = response.interact_pointer_pos() {
            *value = egui::remap_clamp(pointer_position.x, rect.left()..=rect.right(), 0.0..=1.0) as f64;
            response.mark_changed();
        }
    }

    response.widget_info(|| egui::WidgetInfo::slider(*value, ""));

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), *value as f32);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 1.0 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

