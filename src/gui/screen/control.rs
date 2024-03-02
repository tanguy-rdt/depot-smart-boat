use crate::gui::custom_widget;
use crate::gui::screen::http_tools::Resource;

use eframe::egui;
use egui_plot::{Legend, Line, PlotPoint, PlotPoints};
use egui_extras::{Size, StripBuilder};
use std::{sync::{mpsc, Arc, Mutex}};
use eframe::egui::{Rect, Vec2};

pub struct Control {
    msgq_tx: Arc<Mutex<mpsc::Sender<(String, f32)>>>,
    slider_mainsail: custom_widget::circle_slider::CircleSlider,
    slider_mainsail_value: f32,
    slider_jib: custom_widget::circle_slider::CircleSlider,
    slider_jib_value: f32,
    compass: custom_widget::compass::Compass,
    slider_mainsail_height: f32,
    floor_deep: Vec<f32>,
}

impl Control {
    pub fn new(msgq_tx: Arc<Mutex<mpsc::Sender<(String, f32)>>>) -> Self {
        Self {
            msgq_tx: msgq_tx,
            slider_mainsail: custom_widget::circle_slider::CircleSlider::new("Mainsail".to_string()),
            slider_mainsail_value: 0.5,
            slider_jib: custom_widget::circle_slider::CircleSlider::new("Jib".to_string()),
            slider_jib_value: 0.5,
            compass: custom_widget::compass::Compass::new(),
            slider_mainsail_height: 0.0,
            floor_deep: Vec::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui:  &mut egui::Ui){
        StripBuilder::new(ui)
        .size(Size::relative(1.0))
        .horizontal(|mut strip| {
            strip.strip(|builder| {
                builder.sizes(Size::remainder(), 2).vertical(|mut strip| {
                    strip.cell(|ui| {
                        self.show_cmd(ui);
                    });
                    strip.cell(|ui| {
                        self.show_plot(ui);
                    });
                });
            });
        });
    }


    fn show_cmd(&mut self, ui:  &mut egui::Ui) {
        StripBuilder::new(ui)
        .size(Size::relative(1.0))
        .vertical(|mut strip| {
            strip.strip(|builder| {
                builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.allocate_space(egui::Vec2::new(0.0, 15.0));
                                ui.add(self.compass.compass());
                            });
                        });
                    });
                    strip.cell(|ui| {
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
                    });
                });
            });
        });
    }

    fn show_plot(&mut self, ui:  &mut egui::Ui) {
        egui_plot::Plot::new("plot")
        .allow_zoom(false)
        .allow_drag(false)
        .show_axes(true)
        .y_axis_position(egui_plot::HPlacement::Right)
        .y_axis_width(3)
        .legend(Legend::default())
        .show(ui, |plot_ui| {
            plot_ui.hline(egui_plot::HLine::new(0.0).color(egui::Color32::LIGHT_BLUE).style(egui_plot::LineStyle::dashed_dense()));
            let point = PlotPoints::from_ys_f32(&self.floor_deep);
            plot_ui.line(Line::new(point).color(egui::Color32::WHITE).fill(-450.0));
            plot_ui.hline(egui_plot::HLine::new(-450.0).color(egui::Color32::RED).style(egui_plot::LineStyle::dashed_dense()));
            plot_ui.vline(egui_plot::VLine::new((self.floor_deep.len()-1) as f64).color(egui::Color32::RED).style(egui_plot::LineStyle::dashed_dense()));
            let text_position = PlotPoint::new((self.floor_deep.len()-1) as f64, -30.0);
            plot_ui.text(egui_plot::Text::new(text_position, format!("{:.4} m \t\t\t\t\t\t", self.floor_deep[self.floor_deep.len()-1].to_string())).color(egui::Color32::WHITE));
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

    pub fn set_deep(&mut self, value: f32) {
        if self.floor_deep.len() >= 100 {
            self.floor_deep.remove(0);
        }
        self.floor_deep.push(value);
    }    
}
