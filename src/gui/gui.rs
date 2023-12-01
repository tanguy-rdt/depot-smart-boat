#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::gui::map::Osm;
use crate::gui::{SidePanel, SidePanelSelection};

use std::sync::{mpsc, Arc, Mutex};
use eframe::{egui::{self, Button, panel::Side}, epaint::Color32};

pub struct Gui{
    weather: bool,
    map: bool,
    temperature: f32,
    humidity: f32,
    pressure: f32,
    msgq_rx: Arc<Mutex<mpsc::Receiver<(String, f32)>>>,
    osm: Osm,
    side_panel: SidePanel
}

impl Gui {
    pub fn new(msgq_rx: Arc<Mutex<mpsc::Receiver<(String, f32)>>>, egui_ctx: egui::Context) -> Self {
        Self {
            weather: true,
            map: false,
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0.0,
            msgq_rx: msgq_rx,
            osm: Osm::new(egui_ctx),
            side_panel: SidePanel::new()
        }
    }

    fn get_current_value(&mut self) {
        loop {
            match self.msgq_rx.lock().unwrap().try_recv() {
                Ok((var, value)) => {
                    match var.as_str() {
                        "temperature" => self.temperature = value,
                        "humidity" => self.humidity = value,
                        "pressure" => self.pressure = value,
                        _ => (),
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.get_current_value();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Smart boat - control panel");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if *self.side_panel.get_current() == SidePanelSelection::WEATHER {
                ui.label(format!("Temperature: {:.2} C", self.temperature));
                ui.label(format!("Humidity: {:.2} %", self.humidity));
                ui.label(format!("Pressure: {:.2} Pa", self.pressure)); 
            }
            else if *self.side_panel.get_current() == SidePanelSelection::MAP {
                ui.add(self.osm.get_map());
                self.osm.zoom(ui);
                self.osm.position(ui);
    
            }
        });

        self.side_panel.show(ctx);

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.hyperlink_to("github", "https://github.com/tanguy-rdt/depot-smart-boat");
                });
            });
        });

    }
}