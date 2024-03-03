use eframe::{egui::{self, Align2, Response, TextStyle, Vec2}, epaint::{Color32, Pos2}};

pub struct Compass {
    wind_direction: f32,
    boat_direction: f32,
}

impl Compass {
    pub fn new() -> Self {
        Self {
            wind_direction: 0.0,
            boat_direction: 0.0,
        }
    }

    pub fn compass<'a>(&'a mut self) -> impl egui::Widget + 'a {
        move |ui: &mut egui::Ui| {
            self.compass_ui(ui)
        }
    }

    pub fn set_wind_direction(&mut self, val: f32){
        self.wind_direction = val;
    }

    pub fn set_boat_direction(&mut self, val: f32){
        self.boat_direction = val;
    }
    

    fn compass_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(12.0, 12.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::focusable_noninteractive());

        self.update_ui(ui, rect, &response);
        response
    }

    fn update_ui(&mut self, ui: &mut egui::Ui, rect: egui::emath::Rect, response: &egui::Response) {
        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
    
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter()
                .rect(rect, radius, visuals.bg_stroke.color, visuals.fg_stroke);

            let circle_x = egui::lerp((rect.left())..=(rect.right()), 0.5);
            let circle_y = egui::lerp((rect.bottom())..=(rect.top()), 0.5);
            let center = egui::pos2(circle_x, circle_y);
            ui.painter()
                .circle(center, 0.9 * radius, visuals.weak_bg_fill, visuals.bg_stroke);
            ui.painter()
                .circle(center, 0.1 * radius, visuals.bg_stroke.color, visuals.bg_stroke);

            
            
            // --------------------------------------------------------------------------
            // CURSOR
            // --------------------------------------------------------------------------

            let line_start = Pos2::new(rect.center().x - 20.0, rect.center().y);
            let line_end = Pos2::new(rect.center().x + 20.0, rect.center().y);
            ui.painter().line_segment([line_start, line_end], egui::Stroke::new(1.0, egui::Color32::GRAY));
    
            let line_start = Pos2::new(rect.center().x, rect.center().y - 20.0);
            let line_end = Pos2::new(rect.center().x, rect.center().y + 20.0);
            ui.painter().line_segment([line_start, line_end], egui::Stroke::new(1.0, egui::Color32::GRAY));
      
            let line_start = Pos2::new(rect.center_top().x, rect.center_top().y - 5.0);
            let line_end = Pos2::new(rect.center_top().x, rect.center_top().y + 20.0);
            ui.painter().line_segment([line_start, line_end], egui::Stroke::new(2.0, egui::Color32::RED));
      
            let line_start = Pos2::new(rect.left_center().x - 5.0, rect.left_center().y);
            let line_end = Pos2::new(rect.left_center().x + 20.0, rect.left_center().y);
            ui.painter().line_segment([line_start, line_end], egui::Stroke::new(2.0, egui::Color32::GRAY));
      
            let line_start = Pos2::new(rect.right_center().x + 5.0, rect.right_center().y);
            let line_end = Pos2::new(rect.right_center().x - 20.0, rect.right_center().y);
            ui.painter().line_segment([line_start, line_end], egui::Stroke::new(2.0, egui::Color32::GRAY));
            
            let line_start = Pos2::new(rect.center_bottom().x, rect.center_bottom().y + 5.0);
            let line_end = Pos2::new(rect.center_bottom().x, rect.center_bottom().y - 20.0);
            ui.painter().line_segment([line_start, line_end], egui::Stroke::new(2.0, egui::Color32::GRAY));
      
            let num_graduations = 70; 
            let radius = 0.9 * radius;
            for i in 0..num_graduations {
                let angle = 2.0 * std::f32::consts::PI / num_graduations as f32 * i as f32;
                let outer_x = center.x + radius * angle.cos();
                let outer_y = center.y + radius * angle.sin();
                let inner_x = center.x + (radius - 10.0) * angle.cos(); 
                let inner_y = center.y + (radius - 10.0) * angle.sin();
    
                let line_start = Pos2::new(inner_x, inner_y);
                let line_end = Pos2::new(outer_x, outer_y);
                ui.painter().line_segment([line_start, line_end], egui::Stroke::new(1.0, egui::Color32::GRAY));
            }

            // --------------------------------------------------------------------------
            // END CURSOR
            // --------------------------------------------------------------------------



            // --------------------------------------------------------------------------
            // LABEL DIRECTION
            // --------------------------------------------------------------------------

            ui.painter().text(
                egui::pos2(rect.center().x, rect.center().y - 50.0),
                egui::Align2::CENTER_CENTER, 
                "N",
                egui::FontId::new(25.0, egui::FontFamily::Proportional),
                ui.style().visuals.text_color()
            );

            ui.painter().text(
                egui::pos2(rect.center().x, rect.center().y + 50.0),
                egui::Align2::CENTER_CENTER, 
                "S",
                egui::FontId::new(25.0, egui::FontFamily::Proportional),
                ui.style().visuals.text_color()
            );

            ui.painter().text(
                egui::pos2(rect.center().x - 50.0, rect.center().y),
                egui::Align2::CENTER_CENTER, 
                "W",
                egui::FontId::new(25.0, egui::FontFamily::Proportional),
                ui.style().visuals.text_color()
            );

            ui.painter().text(
                egui::pos2(rect.center().x + 50.0, rect.center().y),
                egui::Align2::CENTER_CENTER, 
                "E",
                egui::FontId::new(25.0, egui::FontFamily::Proportional),
                ui.style().visuals.text_color()
            );

            // --------------------------------------------------------------------------
            // END LABEL DIRECTION
            // --------------------------------------------------------------------------



            // --------------------------------------------------------------------------
            // COMPASS HAND
            // --------------------------------------------------------------------------

            let color_compass_hand = if ui.visuals().dark_mode { egui::Color32::WHITE }
            else { egui::Color32::BLACK };

            let angle_in_radians = (self.boat_direction - 90.0).to_radians(); 

            let center = egui::pos2(rect.center().x, rect.center().y);
            let radius = 0.3 * rect.height(); 

            let needle_end_x = center.x + radius * angle_in_radians.cos();
            let needle_end_y = center.y + radius * angle_in_radians.sin(); 

            let line_start = center;
            let line_end = Pos2::new(needle_end_x, needle_end_y);
            ui.painter().line_segment([line_start, line_end], egui::Stroke::new(2.0, color_compass_hand));
            
            let line_end_x = center.x + radius * angle_in_radians.cos();
            let line_end_y =  center.y + radius * angle_in_radians.sin(); 

            let boat_label_pos = match self.boat_direction {
                d if d == 0.0 || d == 360.0 => Pos2::new(line_end_x, line_end_y - 10.0),
                d if d == 90.0 => Pos2::new(line_end_x + 15.0, line_end_y - 2.0),
                d if d == 180.0 => Pos2::new(line_end_x, line_end_y + 10.0),
                d if d == 270.0 => Pos2::new(line_end_x - 15.0, line_end_y - 2.0),
                d if d > 0.0 && d < 90.0 => Pos2::new(line_end_x + 10.0, line_end_y - 10.0),
                d if d > 90.0 && d < 180.0 => Pos2::new(line_end_x + 10.0, line_end_y + 10.0),
                d if d > 180.0 && d < 270.0 => Pos2::new(line_end_x - 10.0, line_end_y + 10.0),
                _ => Pos2::new(line_end_x - 10.0, line_end_y - 10.0),
            };
            
            ui.painter().text(
                boat_label_pos,
                egui::Align2::CENTER_CENTER, 
                format!("{}°", self.boat_direction.to_string()),
                egui::FontId::new(11.0, egui::FontFamily::Proportional),
                color_compass_hand
            );
            
            // --------------------------------------------------------------------------
            // END COMPASS HAND
            // --------------------------------------------------------------------------



            // --------------------------------------------------------------------------
            // GIROUETTE HAND
            // --------------------------------------------------------------------------
            let color_girouette_hand = if ui.visuals().dark_mode { egui::Color32::LIGHT_BLUE }
            else { egui::Color32::DARK_BLUE };

            let angle_in_radians = (self.wind_direction - 90.0).to_radians(); 
    
            let outer_radius = rect.height() * 0.525; 
            let line_length = outer_radius * 0.10; 
            
            let line_end_x = center.x + (outer_radius + line_length) * angle_in_radians.cos();
            let line_end_y = center.y + (outer_radius + line_length) * angle_in_radians.sin(); 
                        
            let line_start_x = center.x + outer_radius * angle_in_radians.cos();
            let line_start_y = center.y + outer_radius * angle_in_radians.sin(); 
            
            let line_start = Pos2::new(line_start_x, line_start_y);
            let line_end = Pos2::new(line_end_x, line_end_y);
            ui.painter().line_segment([line_start, line_end], egui::Stroke::new(2.0, color_girouette_hand)); 

            let line_end_x = center.x + (outer_radius + line_length) * angle_in_radians.cos();
            let line_end_y = (center.y + (outer_radius + line_length) * angle_in_radians.sin()) - 10.0; 

            let wind_label_pos = match self.wind_direction {
                d if d == 0.0 || d == 360.0 => Pos2::new(line_end_x, line_end_y - 10.0),
                d if d == 90.0 => Pos2::new(line_end_x + 15.0, line_end_y - 2.0),
                d if d == 180.0 => Pos2::new(line_end_x, line_end_y + 10.0),
                d if d == 270.0 => Pos2::new(line_end_x - 15.0, line_end_y - 2.0),
                d if d > 0.0 && d < 90.0 => Pos2::new(line_end_x + 10.0, line_end_y - 10.0),
                d if d > 90.0 && d < 180.0 => Pos2::new(line_end_x + 10.0, line_end_y + 10.0),
                d if d > 180.0 && d < 270.0 => Pos2::new(line_end_x - 10.0, line_end_y + 10.0),
                _ => Pos2::new(line_end_x - 10.0, line_end_y - 10.0),
            };
            

            ui.painter().text(
                wind_label_pos,
                egui::Align2::CENTER_CENTER, 
                "wind",
                egui::FontId::new(11.0, egui::FontFamily::Proportional),
                color_girouette_hand
            );

            ui.painter().text(
                Pos2::new(wind_label_pos.x, wind_label_pos.y + 12.0),
                egui::Align2::CENTER_CENTER, 
                format!("{}°", self.wind_direction.to_string()),
                egui::FontId::new(11.0, egui::FontFamily::Proportional),
                color_girouette_hand
            );

            // --------------------------------------------------------------------------
            // END GIROUETTE HAND
            // --------------------------------------------------------------------------
            
        }
    }
}

