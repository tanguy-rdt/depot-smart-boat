#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::gui::screen::Screen;
use crate::gui::{Menu, MenuSelection};

use std::sync::{mpsc, Arc, Mutex};
use eframe::{egui::{self, Button, panel::Side}, epaint::Color32};

pub struct Gui{
    screen: Screen,
    menu: Menu
}

impl Gui {
    pub fn new(msgq_rx: Arc<Mutex<mpsc::Receiver<(String, f32)>>>, msgq_tx: Arc<Mutex<mpsc::Sender<(String, f32)>>>, egui_ctx: egui::Context) -> Self {
        Self {
            screen: Screen::new(msgq_rx, msgq_tx, egui_ctx),
            menu: Menu::new()
        }
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Smart boat - control panel");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.screen.show_current(self.menu.get_current(), ui, ctx);
        });


        egui::SidePanel::right("egui_panel")
        .resizable(false)
        .default_width(150.0)
        .show(ctx, |ui| {
            self.menu.show(ctx, ui);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {

            });
        });

    }
}