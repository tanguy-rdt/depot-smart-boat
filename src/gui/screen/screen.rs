use crate::gui::screen::map::Osm;
use crate::gui::screen::control::Control;
use crate::gui::menu::MenuSelection;

use eframe::egui::{Context, Ui, widgets};
use eframe::egui;
pub struct Screen{
    map: Osm,
    control: Control
}

impl Screen {
    pub fn new(egui_ctx: Context) -> Self {
        Self {
            map: Osm::new(egui_ctx),
            control: Control::new(),
        }
    }

    pub fn show_current(&mut self, menu_choice: &MenuSelection, ui: &mut Ui){
        match menu_choice {
            MenuSelection::WEATHER => self.show_weather_screen(ui),
            MenuSelection::MAP => self.show_map_screen(ui),
            MenuSelection::CONTROL => self.show_control_screen(ui),
            MenuSelection::SETTINGS => self.show_settings_screen(ui),
            _ => (),
        };
    }

    fn show_weather_screen(&mut self, ui: &mut Ui){
        ui.label(format!("Temperature: {:.2} C", 0));
        ui.label(format!("Humidity: {:.2} %", 0));
        ui.label(format!("Pressure: {:.2} Pa", 0)); 
    }

    fn show_map_screen(&mut self, ui: &mut Ui){
        ui.add(self.map.get_map());
        self.map.zoom(ui);
        self.map.position(ui);
    }

    fn show_control_screen(&mut self, ui: &mut Ui){
        self.control.show(ui)
    }

    fn show_settings_screen(&mut self, ui: &mut Ui){
        widgets::global_dark_light_mode_buttons(ui);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.hyperlink_to("github", "https://github.com/tanguy-rdt/depot-smart-boat");
        });
    }
}