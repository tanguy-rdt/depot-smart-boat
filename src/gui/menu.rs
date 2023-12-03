use eframe::{egui, epaint::Color32};

#[derive(PartialEq)]
pub enum MenuSelection {
    WEATHER,
    MAP_CLASSIC,
    MAP_CLOUDS,
    MAP_PRECIPITATION,
    MAP_SEA_LEVEL_PRESSURE,
    MAP_WIND_SPEED,
    MAP_TEMPERATURE,
    CONTROL,
    SETTINGS
}

pub struct Menu {
    current_selection: MenuSelection
}

impl Menu {
    pub fn new() -> Self {
        Self {
            current_selection: MenuSelection::WEATHER
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
            ui.selectable_value(&mut self.current_selection, MenuSelection::WEATHER, "⛅ Weather");
            ui.selectable_value(&mut self.current_selection, MenuSelection::CONTROL, "⛵ Boat Control");
            ui.separator();
            ui.label("🗺 Map");
            ui.separator();
            ui.selectable_value(&mut self.current_selection, MenuSelection::MAP_CLASSIC, "Classic");
            ui.selectable_value(&mut self.current_selection, MenuSelection::MAP_CLOUDS, "Clouds");
            ui.selectable_value(&mut self.current_selection, MenuSelection::MAP_PRECIPITATION, "Precipitation");
            ui.selectable_value(&mut self.current_selection, MenuSelection::MAP_SEA_LEVEL_PRESSURE, "Sea level pressure");
            ui.selectable_value(&mut self.current_selection, MenuSelection::MAP_WIND_SPEED, "Wind speed");
            ui.selectable_value(&mut self.current_selection, MenuSelection::MAP_TEMPERATURE, "Temperature");
            ui.separator();
            ui.selectable_value(&mut self.current_selection, MenuSelection::SETTINGS, "⛭ Settings");
        });
    }

    pub fn get_current(&self) -> &MenuSelection {
        &self.current_selection
    }
}