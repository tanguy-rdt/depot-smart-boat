use crate::gui::screen::map::Map;
use crate::gui::screen::control::Control;
use crate::gui::menu::MenuSelection;

use eframe::egui::{Context, Ui, widgets};
use eframe::egui;
pub struct Screen{
    map: Map,
    control: Control
}

impl Screen {
    pub fn new(egui_ctx: Context) -> Self {
        Self {
            map: Map::new(egui_ctx),
            control: Control::new(),
        }
    }

    pub fn show_current(&mut self, menu_choice: &MenuSelection, ui: &mut Ui, ctx: &egui::Context){
        match menu_choice {
            MenuSelection::WEATHER => self.show_weather_screen(ui),
            MenuSelection::MAP_CLASSIC => self.show_map_screen(ctx, ui, menu_choice),
            MenuSelection::MAP_CLOUDS => self.show_map_screen(ctx, ui, menu_choice),
            MenuSelection::MAP_PRECIPITATION => self.show_map_screen(ctx, ui, menu_choice),
            MenuSelection::MAP_SEA_LEVEL_PRESSURE => self.show_map_screen(ctx, ui, menu_choice),
            MenuSelection::MAP_WIND_SPEED => self.show_map_screen(ctx, ui, menu_choice),
            MenuSelection::MAP_TEMPERATURE => self.show_map_screen(ctx, ui, menu_choice),
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

    fn show_map_screen(&mut self, ctx: &egui::Context, ui: &mut Ui, menu_choice: &MenuSelection){
        self.map.show(ctx, ui, menu_choice);
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