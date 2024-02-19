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
        }
    }

    pub fn show(&mut self, ui:  &mut egui::Ui){
        StripBuilder::new(ui)
        .size(Size::relative(1.0))
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
        egui::Grid::new("TextLayoutDemo")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Mainsail up: ");
            if ui.add(custom_widget::toggle::toggle(&mut self.motor3_clockwise)).changed() { self.msgq_tx.lock().unwrap().send(("mainsail_up".to_owned(), ((self.motor3_clockwise as i8) as f32))).unwrap(); };
            ui.end_row();

            ui.label("Mainsail down: ");
            if ui.add(custom_widget::toggle::toggle(&mut self.motor3_counterclockwise)).changed() { self.msgq_tx.lock().unwrap().send(("mainsail_down".to_owned(), ((self.motor3_counterclockwise as i8) as f32))).unwrap(); };

            ui.end_row();
            ui.allocate_space(egui::Vec2::new(0.0, 30.0));
            ui.end_row();


            ui.end_row();

            if ui.add(self.slider_mainsail.curved_slider(&mut self.slider_mainsail_value)).changed() { 
                self.msgq_tx
                    .lock()
                    .unwrap()
                    .send(("".to_owned(), ((self.slider_mainsail_value as i8) as f32))).unwrap(); 
            };

            ui.add_space(20.0);

            if ui.add(self.slider_jib.curved_slider(&mut self.slider_jib_value)).changed() { 
                self.msgq_tx
                    .lock()
                    .unwrap()
                    .send(("".to_owned(), ((self.slider_jib_value as i8) as f32))).unwrap(); 
            };
            ui.end_row();
        });
    }

    pub fn set_mainsail_value(&mut self, value: f32) {
        self.slider_mainsail_value = value;
    }

    
}

