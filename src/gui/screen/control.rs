use crate::gui::custom_widget::{self, circle_slider};
use crate::gui::screen::http_tools::Resource;
use crate::gui::screen::http_tools;

use eframe::egui;
use egui_extras::{Size, StripBuilder};
use std::{sync::{mpsc, Arc, Mutex}, thread::current};
use std::time::{Duration, Instant};
use poll_promise::Promise;
use eframe::egui::{Image, Rect, Vec2, Ui};

pub struct Control {
    msgq_tx: Arc<Mutex<mpsc::Sender<(String, f32)>>>,
    motor0_clockwise: bool,
    motor3_clockwise: bool,
    motor3_counterclockwise: bool,
    slider_mainsail: custom_widget::circle_slider::CircleSlider,
    slider_mainsail_value: f32,
    slider_jib: custom_widget::circle_slider::CircleSlider,
    slider_jib_value: f32,
    compass: custom_widget::compass::Compass,
    slider_mainsail_height: f32,
    last_fetch_cam: Instant,
    promise_cam: Option<Promise<ehttp::Result<Resource>>>,

}

impl Control {
    pub fn new(msgq_tx: Arc<Mutex<mpsc::Sender<(String, f32)>>>) -> Self {
        Self {
            msgq_tx: msgq_tx,
            motor0_clockwise: false,
            motor3_clockwise: false,
            motor3_counterclockwise: false,
            slider_mainsail: custom_widget::circle_slider::CircleSlider::new("Mainsail".to_string()),
            slider_mainsail_value: 0.5,
            slider_jib: custom_widget::circle_slider::CircleSlider::new("Jib".to_string()),
            slider_jib_value: 0.5,
            compass: custom_widget::compass::Compass::new(),
            slider_mainsail_height: 0.0,
            last_fetch_cam: Instant::now() - Duration::from_secs(1),
            promise_cam: Default::default(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui:  &mut egui::Ui, cam_choice: &usize){
        StripBuilder::new(ui)
        .size(Size::relative(1.0))
        .vertical(|mut strip| {
            strip.strip(|builder| {
                builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.allocate_space(egui::Vec2::new(0.0, 20.0));

                            if *cam_choice == 1 {
                                let now = Instant::now();
                                if now.duration_since(self.last_fetch_cam) > Duration::from_secs(1) {
                                    self.last_fetch_cam = now;
                                    self.promise_cam = http_tools::fetch_ressource(ctx, "http://192.168.1.46/photo.jpg".to_owned());
                                }
                                self.show_image(&self.promise_cam, ui, ctx);
                            }
                            else {
                                ui.image(egui::include_image!("img/no_img.png"));
                            }
       
                            ui.allocate_space(egui::Vec2::new(0.0, 10.0));
                            ui.image(egui::include_image!("img/no_img.png"));

                        });
                    });
                    strip.cell(|ui| {
                        self.show_cmd(ui);
                    });
                });
            });
        });
    }

    fn show_cmd(&mut self, ui:  &mut egui::Ui) {



        ui.allocate_space(egui::Vec2::new(0.0, 30.0));

        
        ui.vertical_centered(|ui| {
            ui.add(self.compass.compass());
        });
        

        egui::Grid::new("circle_slider_grid")
        .num_columns(2)
        .min_col_width(ui.available_width() / 2.0) 
        .striped(false) 
        .show(ui, |ui| {

        ui.end_row();
        ui.end_row();


        ui.vertical_centered(|ui| {
            if ui.add(self.slider_jib.curved_slider(&mut self.slider_jib_value)).changed() { 
                self.msgq_tx
                    .lock()
                    .unwrap()
                    .send(("jib_angle".to_owned(), self.slider_jib_value)).unwrap(); 
            };
        });

        ui.vertical_centered(|ui| {
            if ui.add(self.slider_mainsail.curved_slider(&mut self.slider_mainsail_value)).changed() { 
                self.msgq_tx
                    .lock()
                    .unwrap()
                    .send(("mainsail_angle".to_owned(), self.slider_mainsail_value)).unwrap(); 
            };
        });


        ui.end_row();
        ui.allocate_space(egui::Vec2::new(0.0, 30.0));
        ui.end_row();
    });

    ui.vertical_centered(|ui| {
        if ui.add(custom_widget::slider::slidebar(&mut self.slider_mainsail_height)).changed() { 
            self.msgq_tx
                .lock()
                .unwrap()
                .send(("mainsail_height".to_owned(), self.slider_mainsail_height)).unwrap(); 
        };
    });
        


    }

    pub fn set_mainsail_value(&mut self, value: f32) {
        self.slider_mainsail_value = value;
    }

    pub fn set_jib_value(&mut self, value: f32) {
        self.slider_jib_value = value;
    }

    pub fn set_mainsail_height_value(&mut self, value: f32) {
        self.slider_mainsail_height = value;
    }


    pub fn set_boat_direction_degree(&mut self, value: f32) {
        self.compass.set_boat_direction(value);    }

    pub fn set_wind_direction_degree(&mut self, value: f32) {
        self.compass.set_wind_direction(value);
    }

    fn show_image(&self, promise: &Option<Promise<Result<Resource, String>>>, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(promise) = promise {
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
                    
                            let size = Vec2::new(width, height); // Taille des images
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
    
}

