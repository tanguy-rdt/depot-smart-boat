use eframe::{egui::{self, InputState}, epaint::Color32};

pub fn slidebar(value: &mut f32) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| slidebar_ui(ui, value)
}  

fn slidebar_ui(ui: &mut egui::Ui, value: &mut f32) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(7.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());
    if response.dragged() {
        if let Some(pointer_position) = response.interact_pointer_pos() {
            let raw_value = egui::remap_clamp(pointer_position.x, rect.left()..=rect.right(), 0.0..=1.0);
            *value = format!("{:.2}", raw_value).parse().unwrap();
        }
    }

    if response.drag_released() {
        response.mark_changed();
    }

    response.widget_info(|| egui::WidgetInfo::slider(*value as f64, ""));

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), *value);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 1.0 * radius, visuals.bg_fill, visuals.fg_stroke);

        ui.painter().text(
            egui::Pos2::new(rect.center_top().x, rect.center_top().y - 10.0),
            egui::Align2::CENTER_CENTER, 
            format!("Mainsail height: {} %", (*value * 100.0) as u32),
            egui::FontId::new(13.0, egui::FontFamily::Proportional),
            ui.style().visuals.text_color()
        );
    }

    response
}

