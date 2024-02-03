use crate::gui::tools;

use eframe::egui;
use egui_extras::{Size, StripBuilder};
use std::sync::{mpsc, Arc, Mutex};

pub struct Control {
    msgq_tx: Arc<Mutex<mpsc::Sender<(String, f32)>>>,
    motor0_clockwise: bool,
    motor1_clockwise: bool,
    motor3_clockwise: bool,
    motor0_counterclockwise: bool,
    motor1_counterclockwise: bool,
    motor3_counterclockwise: bool,
    sail: bool,
}

impl Control {
    pub fn new(msgq_tx: Arc<Mutex<mpsc::Sender<(String, f32)>>>) -> Self {
        Self {
            msgq_tx: msgq_tx,
            motor0_clockwise: false,
            motor1_clockwise: false,
            motor3_clockwise: false,
            motor0_counterclockwise: false,
            motor1_counterclockwise: false,
            motor3_counterclockwise: false,
            sail: false,
        }
    }

    pub fn show(&mut self, ui:  &mut egui::Ui){
        StripBuilder::new(ui)
        .size(Size::relative(1.0)) // Diviser l'espace en deux colonnes, chaque colonne ayant la moitié de la largeur disponible
        .vertical(|mut strip| {
            strip.strip(|builder| {
                builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                    strip.cell(|ui| {
                        self.show_image(ui);
                    });
                    strip.cell(|ui| {
                        self.show_cmd(ui);
                    });
                });
            });
        });
    }

    fn show_image(&mut self, ui:  &mut egui::Ui){
        match self.sail {
            true => {
                if ui.visuals().dark_mode {
                    ui.image(egui::include_image!(
                        "./img/dark_theme/boat_actif.png"
                    ));
                }
                else {
                    ui.image(egui::include_image!(
                        "./img/light_theme/boat_actif.png"
                    ));
                }

            },
            false => {
                if ui.visuals().dark_mode {
                    ui.image(egui::include_image!(
                        "./img/dark_theme/boat_inactif.png"
                    ));
                }
                else {
                    ui.image(egui::include_image!(
                        "./img/light_theme/boat_inactif.png"
                    ));
                }
            },
            _ => (),
        };
    }

    fn show_cmd(&mut self, ui:  &mut egui::Ui) {
        egui::Grid::new("TextLayoutDemo")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Jib Starboard: ");
            if ui.add(tools::toggle(&mut self.motor0_clockwise)).changed() { self.msgq_tx.lock().unwrap().send(("jib_starboard".to_owned(), ((self.motor0_clockwise as i8) as f32))).unwrap(); };
            ui.end_row();

            ui.label("Jib to port: ");
            if ui.add(tools::toggle(&mut self.motor0_counterclockwise)).changed() { self.msgq_tx.lock().unwrap().send(("jib_to_port".to_owned(), ((self.motor0_counterclockwise as i8) as f32))).unwrap(); };
            ui.end_row();

            ui.label("Mainsail Starboard: ");
            if ui.add(tools::toggle(&mut self.motor1_clockwise)).changed() { self.msgq_tx.lock().unwrap().send(("mainsail_starboard".to_owned(), ((self.motor1_clockwise as i8) as f32))).unwrap(); };
            ui.end_row();

            ui.label("Mainsail to port: ");
            if ui.add(tools::toggle(&mut self.motor1_counterclockwise)).changed() { self.msgq_tx.lock().unwrap().send(("mainsail_to_port".to_owned(), ((self.motor1_counterclockwise as i8) as f32))).unwrap(); };
            ui.end_row();

            ui.label("Mainsail up: ");
            if ui.add(tools::toggle(&mut self.motor3_clockwise)).changed() { self.msgq_tx.lock().unwrap().send(("mainsail_up".to_owned(), ((self.motor3_clockwise as i8) as f32))).unwrap(); };
            ui.end_row();

            ui.label("Mainsail down: ");
            if ui.add(tools::toggle(&mut self.motor3_counterclockwise)).changed() { self.msgq_tx.lock().unwrap().send(("mainsail_down".to_owned(), ((self.motor3_counterclockwise as i8) as f32))).unwrap(); };
            ui.end_row();

            ui.label("Sail :");
            ui.add(tools::toggle(&mut self.sail));
            ui.end_row();

            ui.label("Safran :");
            ui.end_row();

            ui.label("Mainsail :");
            ui.end_row();

        });
    }

    pub fn set_motor(&mut self, value: bool) {
        self.motor0_clockwise = value;
    }

    
}

