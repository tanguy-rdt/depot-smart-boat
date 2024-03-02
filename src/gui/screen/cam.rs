use std::default;

use eframe::egui;
use egui_extras::{Size, StripBuilder};

#[derive(Default, Clone)]
struct CameraParam {
    ip: String,
    name: String,
    index: usize,
}

pub struct Cam{
    cam1: CameraParam,
    cam2: CameraParam,
    cam3: CameraParam,
    cam4: CameraParam,
    cam_found:  Vec<String>,
}

impl Cam {
    pub fn new() -> Self {
        let mut cam1 = CameraParam::default();
        let mut cam2 = CameraParam::default();
        let mut cam3 = CameraParam::default();
        let mut cam4 = CameraParam::default();
        let cam_found = vec![String::from("No device"), String::from("Mainsail")];
        cam1.name = cam_found[0].clone();
        cam2.name = cam_found[0].clone();
        cam3.name = cam_found[0].clone();
        cam4.name = cam_found[0].clone();

        Self { cam1, cam2, cam3, cam4, cam_found }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui:  &mut egui::Ui) {
        egui::Grid::new("circle_slider_grid")
        .num_columns(2)
        .min_col_width(ui.available_width() / 2.0) 
        .min_row_height(ui.available_height() / 2.0)
        .striped(false) 
        .show(ui, |ui| {
            self.show_camera(0, ui);
            self.show_camera(1, ui);
            ui.end_row();     
            self.show_camera(2, ui);
            self.show_camera(3, ui);

        });
    }

    fn show_camera(&mut self, cam_slot_index: usize, ui:  &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.image(egui::include_image!("img/no_img.png"));

            ui.horizontal(|ui| {  
                ui.label("Camera:");  
                let combo_box_camera_device = egui::containers::ComboBox::new(cam_slot_index, "")
                .selected_text(&self.cam_found[self.cam1.index])
                .show_ui(ui, |ui| {
                    for (index, device) in self.cam_found.iter().enumerate() {
                        ui.selectable_value(&mut self.cam1.index, index, device);
                    }
                });
            });
        });
    }
}