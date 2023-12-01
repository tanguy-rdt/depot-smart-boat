use eframe::{egui, epaint::Color32};

#[derive(PartialEq)]
pub enum SidePanelSelection {
    WEATHER,
    MAP,
}

pub struct SidePanel{
    current_selection: SidePanelSelection, 
    weather_state: bool,
}

impl SidePanel {
    pub fn new() -> Self {
        Self {
            current_selection: SidePanelSelection::WEATHER,
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
                        self.current_selection = SidePanelSelection::WEATHER;
                    }
                    else if ui.button("🗺 Map").clicked() {
                        self.current_selection = SidePanelSelection::MAP;
                    }
                });
        });
    }

    pub fn get_current(&self) -> &SidePanelSelection {
        &self.current_selection
    }
}