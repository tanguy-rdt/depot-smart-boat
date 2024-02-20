use crate::gui::custom_widget::{self, circle_slider};

use eframe::egui;
use egui_extras::{Size, StripBuilder};
use std::{sync::{mpsc, Arc, Mutex}, thread::current};

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
    slider: f64,
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
            slider: 0.0,
        }
    }

    pub fn show(&mut self, ui:  &mut egui::Ui){
        StripBuilder::new(ui)
        .size(Size::relative(0.5))
        .vertical(|mut strip| {
            strip.strip(|builder| {
                builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                    strip.cell(|ui| {
                        //self.show_image(ui);
                    });
                    strip.cell(|ui| {
                        self.show_cmd(ui);
                    });
                });
            });
        });
    }

    fn show_cmd(&mut self, ui:  &mut egui::Ui) {

        self.compass.set_wind_direction(200.0);
        self.compass.set_boat_direction(40.0);
        
        ui.vertical_centered(|ui| {
            ui.add(self.compass.compass());
        });
        

        egui::Grid::new("circle_slider_grid")
        .num_columns(2)
        .min_col_width(ui.available_width() / 2.0) // Définit la largeur minimale des colonnes pour occuper toute la largeur
    .striped(false) // Pour un style de grille sans alternance de couleur
    .show(ui, |ui| {

        ui.end_row();
        ui.end_row();
        ui.end_row();


        ui.vertical_centered(|ui| {
            if ui.add(self.slider_jib.curved_slider(&mut self.slider_jib_value)).changed() { 
                self.msgq_tx
                    .lock()
                    .unwrap()
                    .send(("".to_owned(), ((self.slider_jib_value as i8) as f32))).unwrap(); 
            };
        });

        ui.vertical_centered(|ui| {
            if ui.add(self.slider_mainsail.curved_slider(&mut self.slider_mainsail_value)).changed() { 
                self.msgq_tx
                    .lock()
                    .unwrap()
                    .send(("".to_owned(), ((self.slider_mainsail_value as i8) as f32))).unwrap(); 
            };
        });


        ui.end_row();
        ui.allocate_space(egui::Vec2::new(0.0, 30.0));
        ui.end_row();
    });

    ui.vertical_centered(|ui| {
        ui.add(custom_widget::slider::slidebar(&mut self.slider));
    });
        


    }

    pub fn set_mainsail_value(&mut self, value: f32) {
        self.slider_mainsail_value = value;
    }

    
}

