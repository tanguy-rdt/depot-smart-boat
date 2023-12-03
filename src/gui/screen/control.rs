use eframe::egui;
use egui_extras::{Size, StripBuilder};

use crate::gui::tools;

pub struct Control {
    mainsail: bool
}

impl Control {
    pub fn new() -> Self {
        Self {
            mainsail: false,
        }
    }

    fn show_image(&mut self, ui:  &mut egui::Ui){
        match self.mainsail {
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
            ui.add(tools::toggle(&mut self.mainsail));
            ui.label("mainsail");
            ui.end_row();
        });
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
}

