use eframe::{egui::{self, Align2, Response, TextStyle, Vec2}, epaint::{Color32, Pos2}};

pub struct CircleSlider {
    position_x: f32,
    position_y: f32,
    value: f32,
    value_percent: i32,
    drag_done: bool,
    label: String,
}

impl CircleSlider {
    pub fn new(label: String) -> Self {
        Self {
            position_x: 0.5,
            position_y: 0.0,
            value: 0.0,
            value_percent: 0,
            drag_done: true,
            label: label,
        }
    }

    pub fn curved_slider<'a>(&'a mut self, value: &'a mut f32) -> impl egui::Widget + 'a {
        move |ui: &mut egui::Ui| {
            self.curved_slider_ui(ui, value)
        }
    }

    fn curved_slider_ui(&mut self, ui: &mut egui::Ui, value: &mut f32) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(7.0, 7.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::drag());
        response.widget_info(|| egui::WidgetInfo::slider(*value as f64, ""));

        if response.dragged() {
            if let Some(pointer_position) = response.interact_pointer_pos() {
                self.drag_done = false;
                let pointer_position_x = egui::remap_clamp(pointer_position.x, rect.left()..=rect.right(), 0.0..=1.0);
                let pointer_position_y = egui::remap_clamp(pointer_position.y, rect.bottom()..=rect.top(), 0.0..=1.0);
                self.calc_position_from_pointer(pointer_position_x, pointer_position_y);
            }
        }
        else if response.drag_released() {
            self.drag_done = true;
            *value = self.value;
        }
        else if self.drag_done {
            self.calc_position_from_value(&value);
        }

        self.update_ui(ui, rect, &response);
        response
    }

    fn calc_position_from_pointer(&mut self, pointer_position_x: f32, pointer_position_y: f32) {
        let radius = 0.5; 
        let center_x = 0.5; 
        let center_y = 0.5; 
        let dx = pointer_position_x - center_x;
        let dy = pointer_position_y - center_y;
        let mut angle = dy.atan2(dx) + 2.0 * std::f32::consts::PI;

        if angle > 2.5 * std::f32::consts::PI {
            angle = std::f32::consts::PI;
        } else if angle > 2.0 * std::f32::consts::PI && angle < 2.5 * std::f32::consts::PI {
            angle = 2.0 * std::f32::consts::PI;
        }
        
        self.position_x = center_x + radius * angle.cos();
        self.position_y = center_y + radius * angle.sin();

        self.value = format!("{:.2}", (angle - std::f32::consts::PI)/std::f32::consts::PI).parse().unwrap();
        self.value_percent = ((self.value * 100.0) - 50.0) as i32;
    }

    fn calc_position_from_value(&mut self, value: &f32) {
        let radius = 0.5; 
        let center_x = 0.5; 
        let center_y = 0.5; 
        let angle = std::f32::consts::PI * *value + std::f32::consts::PI;
        
        self.position_x = center_x + radius * angle.cos();
        self.position_y = center_y + radius * angle.sin();

        self.value = format!("{:.2}", (angle - std::f32::consts::PI)/std::f32::consts::PI).parse().unwrap();
        self.value_percent = ((self.value * 100.0) - 50.0) as i32;
    }


    fn update_ui(&mut self, ui: &mut egui::Ui, rect: egui::emath::Rect, response: &egui::Response) {
        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
    
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter()
                .rect(rect, radius, egui::Color32::TRANSPARENT, visuals.fg_stroke);

            let left_line_start = Pos2::new(rect.left_center().x - 10.0, rect.left_center().y);
            let left_line_end = Pos2::new(rect.left_center().x + 10.0, rect.left_center().y);
            ui.painter().line_segment([left_line_start, left_line_end], egui::Stroke::new(2.0, egui::Color32::WHITE));


            let right_line_start = Pos2::new(rect.right_center().x - 10.0, rect.right_center().y);
            let right_line_end = Pos2::new(rect.right_center().x + 10.0, rect.right_center().y);
            ui.painter().line_segment([right_line_start, right_line_end], egui::Stroke::new(2.0, egui::Color32::WHITE));
    
            let circle_x = egui::lerp((rect.left())..=(rect.right()), self.position_x);
            let circle_y = egui::lerp((rect.bottom())..=(rect.top()), self.position_y);
            let center = egui::pos2(circle_x, circle_y);
            ui.painter()
                .circle(center, 0.1 * radius, visuals.bg_fill, visuals.fg_stroke);

            ui.painter().text(
                egui::pos2(rect.center().x, rect.center().y - 15.0),
                egui::Align2::CENTER_CENTER, 
                self.label.to_string(),
                egui::FontId::new(25.0, egui::FontFamily::Proportional),
                ui.style().visuals.text_color()
            );

            ui.painter().text(
                egui::pos2(rect.center().x, rect.center().y + 15.0),
                egui::Align2::CENTER_CENTER, 
                format!("{} %", self.value_percent.to_string()),
                egui::FontId::new(20.0, egui::FontFamily::Proportional),
                ui.style().visuals.text_color()
            );
        }
    }
}

