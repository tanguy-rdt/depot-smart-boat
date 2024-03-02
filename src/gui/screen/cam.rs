use crate::gui::screen::http_tools;
use crate::gui::screen::http_tools::Resource;

use eframe::egui;
use egui_extras::{Size, StripBuilder};
use std::time::{Duration, Instant};
use poll_promise::Promise;
use eframe::egui::{Image, Rect, Vec2, Ui};

struct CameraParam {
    ip: String,
    name: String,
    index: usize,
    last_fetch: Instant,
    promise: Option<Promise<ehttp::Result<Resource>>>,
}

impl Default for CameraParam {
    fn default() -> Self {
        Self {
            ip: "0.0.0.0".to_string(),
            name: "No device".to_string(), 
            index: 0,
            last_fetch: Instant::now() - Duration::from_secs(1),
            promise: Default::default(),
        }
    }
}

pub struct Cam{
    cam_indice: [usize; 4],
    cam_found:  Vec<CameraParam>,
}

impl Cam {
    pub fn new() -> Self {
        let default = CameraParam::default();
        let cam = CameraParam {
                                    ip: "192.168.1.46".to_owned(),
                                    name: "rpi zero".to_owned(),
                                    index: 1,
                                    last_fetch: Instant::now() - Duration::from_secs(1),
                                    promise: Default::default(),
                                };
        let cam_found = vec![default, cam];

        Self { 
            cam_indice: [0, 0, 0, 0],
            cam_found: cam_found,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui:  &mut egui::Ui) {
        egui::Grid::new("circle_slider_grid")
        .num_columns(2)
        .min_col_width(ui.available_width() / 2.0) 
        .min_row_height(ui.available_height() / 2.0)
        .striped(false) 
        .show(ui, |ui| {
            self.show_camera(0, ctx, ui);
            self.show_camera(1, ctx, ui);
            ui.end_row();
            self.show_camera(2, ctx, ui);
            self.show_camera(3, ctx, ui);
        });
    }

    fn show_camera(&mut self, cam_slot_index: usize, ctx: &egui::Context, ui:  &mut egui::Ui) {
        ui.vertical(|ui| {

            self.show_image(cam_slot_index, ctx, ui);

            ui.horizontal(|ui| {  
                ui.label("Camera:");  
                let combo_box_camera_device = egui::containers::ComboBox::new(cam_slot_index, "")
                .selected_text(&self.cam_found[self.cam_indice[cam_slot_index]].name)
                .show_ui(ui, |ui| {
                    for (index, device) in self.cam_found.iter().enumerate() {
                        ui.selectable_value(&mut self.cam_indice[cam_slot_index], index, device.name.clone());
                    }
                });
            });
        });
    }

    fn show_image(&mut self, cam_slot_index: usize, ctx: &egui::Context, ui:  &mut egui::Ui) {
        if self.cam_found[self.cam_indice[cam_slot_index]].name != "No device" {
            let now = Instant::now();
            if now.duration_since(self.cam_found[self.cam_indice[cam_slot_index]].last_fetch) > Duration::from_secs(1) {
                self.cam_found[self.cam_indice[cam_slot_index]].last_fetch = now;
                self.cam_found[self.cam_indice[cam_slot_index]].promise = http_tools::fetch_ressource(ctx, format!("http://{}/photo.jpg", self.cam_found[self.cam_indice[cam_slot_index]].ip).to_owned());
            }
            if let Some(promise) = &self.cam_found[self.cam_indice[cam_slot_index]].promise {
                if let Some(result) = promise.ready() {
                    match result {
                        Ok(resource) => {
                            let Resource {
                                response,
                                text,
                                image,
                            } = resource;
    
                            if let Some(image) = image {
                                let image = image.clone();
                        
                                let available_rect = ctx.available_rect();
                                let width = available_rect.width()/2.0;
                                let height = available_rect.height()/2.0;
                        
                                let size = Vec2::new(width, height); 
                                let rect = Rect::from_min_size(ui.min_rect().min + Vec2::new(0.0, 0.0), size);
                                image.paint_at(ui, rect);
                        
                            } else {
                                ui.image(egui::include_image!("img/no_img.png"));   
                            }
                        }
                        Err(error) => {
                            ui.image(egui::include_image!("img/no_img.png"));   
                        }
                    }
                } else {
                    ui.spinner();
                }
            }
        } 
        else {
            ui.image(egui::include_image!("img/no_img.png"));
        }
    }
}