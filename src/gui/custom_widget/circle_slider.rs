use eframe::{egui::{self, Align2, TextStyle}, epaint::Color32};

pub struct CircleSlider {
    position_x: f32,
    position_y: f32,
    value: i32,
}

impl CircleSlider {
    pub fn new() -> Self {
        Self {
            position_x: 0.5,
            position_y: 0.0,
            value: 0,
        }
    }

    pub fn curved_slidebar<'a>(&'a mut self, value: &'a mut f64) -> impl egui::Widget + 'a {
        move |ui: &mut egui::Ui| {
            self.curved_slidebar_ui(ui, value)
        }
    }

    fn curved_slidebar_ui(&mut self, ui: &mut egui::Ui, value: &mut f64) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(10.0, 10.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());
        response.widget_info(|| egui::WidgetInfo::slider(*value, ""));

        if response.dragged() {
            if let Some(pointer_position) = response.interact_pointer_pos() {
                let pointer_position_x = egui::remap_clamp(pointer_position.x, rect.left()..=rect.right(), 0.0..=1.0);
                let pointer_position_y = egui::remap_clamp(pointer_position.y, rect.bottom()..=rect.top(), 0.0..=1.0);
    
                let relative_x = pointer_position_x - 0.5;
                let relative_y = pointer_position_y - 0.5;
                let length = (relative_x * relative_x + relative_y * relative_y).sqrt();
                self.position_x = 0.5 + relative_x / length * 0.5;
                self.position_y = 0.5 + relative_y / length * 0.5;

                if self.position_y > 0.5 {
                    self.position_y = 0.5;
                    if (self.position_x < 0.5) { self.position_x = 0.0; }
                    else { self.position_x = 1.0; }
                }

                self.value = (self.position_x * 100.0) as i32;
            }
        }
    
        if response.drag_released() {
            *value = self.position_x as f64;
        }
    
        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
    
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter()
                .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
    
            let circle_x = egui::lerp((rect.left())..=(rect.right()), self.position_x);
            let circle_y = egui::lerp((rect.bottom())..=(rect.top()), self.position_y);
            let center = egui::pos2(circle_x, circle_y);
            ui.painter()
                .circle(center, 0.1 * radius, visuals.bg_fill, visuals.fg_stroke);

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER, 
                format!("{} %", self.value.to_string()),
                egui::FontId::new(30.0, egui::FontFamily::Proportional),
                ui.style().visuals.text_color()
            );

        }
        
        response
    }
}

