#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::gui::screen::Screen;
use crate::gui::{Menu, MenuSelection};

use std::sync::{mpsc, Arc, Mutex};
use eframe::{egui::{self, Button, panel::Side}, epaint::Color32};

pub struct Gui{
    weather: bool,
    map: bool,
    temperature: f32,
    humidity: f32,
    pressure: f32,
    msgq_rx: Arc<Mutex<mpsc::Receiver<(String, f32)>>>,
    screen: Screen,
    menu: Menu
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
            screen: Screen::new(egui_ctx),
            menu: Menu::new()
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
            self.screen.show_current(self.menu.get_current(), ui);
        });

        self.menu.show(ctx);

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {

            });
        });

    }
}