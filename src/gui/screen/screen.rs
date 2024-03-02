use crate::gui::screen::map::Map;
use crate::gui::screen::weather::Weather;
use crate::gui::screen::control::Control;
use crate::gui::menu::MenuSelection;

use std::sync::{mpsc, Arc, Mutex};
use eframe::egui::{Context, Ui, widgets};
use eframe::egui;

pub struct Screen{
    msgq_rx: Arc<Mutex<mpsc::Receiver<(String, f32)>>>,
    control: Control,
    weather: Weather,
    map: Map,
}

impl Screen {
    pub fn new(msgq_rx: Arc<Mutex<mpsc::Receiver<(String, f32)>>>, msgq_tx: Arc<Mutex<mpsc::Sender<(String, f32)>>>, egui_ctx: Context) -> Self {
        Self {
            msgq_rx: msgq_rx,
            map: Map::new(egui_ctx),
            weather: Weather::new(),
            control: Control::new(msgq_tx),
        }
    }

    pub fn show_current(&mut self, menu_choice: &MenuSelection, ui: &mut Ui, ctx: &egui::Context){
        self.check_msgq_rx();
        match menu_choice {
            MenuSelection::WEATHER => self.show_weather_screen(ctx, ui),
            MenuSelection::MAP_CLASSIC => self.show_map_screen(ctx, ui, menu_choice),
            MenuSelection::MAP_CLOUDS => self.show_map_screen(ctx, ui, menu_choice),
            MenuSelection::MAP_PRECIPITATION => self.show_map_screen(ctx, ui, menu_choice),
            MenuSelection::MAP_SEA_LEVEL_PRESSURE => self.show_map_screen(ctx, ui, menu_choice),
            MenuSelection::MAP_WIND_SPEED => self.show_map_screen(ctx, ui, menu_choice),
            MenuSelection::MAP_TEMPERATURE => self.show_map_screen(ctx, ui, menu_choice),
            MenuSelection::CONTROL => self.show_control_screen(ctx, ui),
            _ => (),
        };
    }

    fn show_weather_screen(&mut self, ctx: &egui::Context, ui: &mut Ui){
        self.weather.show(ctx, ui);
    }

    fn show_map_screen(&mut self, ctx: &egui::Context, ui: &mut Ui, menu_choice: &MenuSelection){
        self.map.show(ctx, ui, menu_choice);
    }

    fn show_control_screen(&mut self, ctx: &egui::Context, ui: &mut Ui){
        self.control.show(ctx, ui);
    }

    fn check_msgq_rx(&mut self){
        loop {
            match self.msgq_rx.lock().unwrap().try_recv() {
                Ok((var, value)) => {
                    match var.as_str() {
                        "temperature" => self.weather.set_temperature(value),
                        "humidity" => self.weather.set_humidity(value),
                        "pressure" => self.weather.set_pressure(value),
                        "mainsail_angle" => self.control.set_mainsail_value(value),
                        "jib_angle" => self.control.set_jib_value(value),
                        "mainsail_height" => self.control.set_mainsail_height_value(value),
                        "boat_direction" => self.control.set_boat_direction_degree(value),
                        "wind_direction" => self.control.set_wind_direction_degree(value),
                        "deep" => self.control.set_deep(value),
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