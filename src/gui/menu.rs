use eframe::{egui, epaint::Color32};

#[derive(PartialEq)]
pub enum MenuSelection {
    WEATHER,
    MAP,
    CONTROL,
    SETTINGS
}

pub struct Menu{
    current_selection: MenuSelection, 
    weather_state: bool,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            current_selection: MenuSelection::WEATHER,
            weather_state: true,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context){
        egui::SidePanel::right("egui_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::from_rgba_unmultiplied(0, 106, 200, 155);
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
                    if ui.button("⛅ Weather").clicked() {
                        self.current_selection = MenuSelection::WEATHER;
                    }
                    else if ui.button("🗺 Map").clicked() {
                        self.current_selection = MenuSelection::MAP;
                    }
                    else if ui.button("⛵ Boat Control").clicked() {
                        self.current_selection = MenuSelection::CONTROL;
                    }
                    else if ui.button("⛭ Settings").clicked() {
                        self.current_selection = MenuSelection::SETTINGS;
                    }
                });
        });
    }

    pub fn get_current(&self) -> &MenuSelection {
        &self.current_selection
    }
}
