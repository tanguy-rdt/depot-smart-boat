use eframe::{egui::{self, Separator}, epaint::Color32};
use pv_recorder::PvRecorderBuilder;

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
}

pub struct Menu {
    current_selection: MenuSelection,
    current_audio_device: usize,
    audio_device: Vec<String>,
    camera_device_1:  Vec<String>,
    camera_device_1_index: usize,
    camera_device_2:  Vec<String>,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            current_selection: MenuSelection::CONTROL,
            current_audio_device: 0,
            audio_device: vec![String::from("No device")],
            camera_device_1: vec![String::from("No device"), String::from("Camera 1")],
            camera_device_1_index: 0,
            camera_device_2: vec![String::from("No device")],
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
            ui.selectable_value(&mut self.current_selection, MenuSelection::CONTROL, "⛵ Boat Control");
            ui.selectable_value(&mut self.current_selection, MenuSelection::WEATHER, "⛅ Weather");
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
            ui.label("⚙ Settings");
            ui.separator();
            ui.label("Audio input device:");
            ui.horizontal(|ui| {    
                let combo_box_audio_input = egui::containers::ComboBox::new(0, "")
                .selected_text(&self.audio_device[self.current_audio_device])
                .show_ui(ui, |ui| {
                    for (index, device) in self.audio_device.iter().enumerate() {
                        ui.selectable_value(&mut self.current_audio_device, index, device);
                    }
                });
    
                if ui.button("R").clicked() {
                    self.audio_device = self.search_audio_devices();
                }
            });
            ui.allocate_space(egui::Vec2::new(0.0, 5.0));
            ui.label("Camera 1:");
            ui.horizontal(|ui| {    
                let combo_box_camera_device = egui::containers::ComboBox::new(1, "")
                .selected_text(&self.camera_device_1[self.camera_device_1_index])
                .show_ui(ui, |ui| {
                    for (index, device) in self.camera_device_1.iter().enumerate() {
                        ui.selectable_value(&mut self.camera_device_1_index, index, device);
                    }
                });
            });
            ui.allocate_space(egui::Vec2::new(0.0, 5.0));
            ui.label("Camera 2:");
            ui.horizontal(|ui| {    
                let combo_box_camera_device = egui::containers::ComboBox::new(2, "")
                .selected_text(&self.camera_device_2[0])
                .show_ui(ui, |ui| {
                    for (index, device) in self.camera_device_2.iter().enumerate() {
                        ui.selectable_value(&mut 0, index, device);
                    }
                });
            });
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.hyperlink_to("github", "https://github.com/tanguy-rdt/depot-smart-boat");
            });
        });
    }

    pub fn get_current(&self) -> &MenuSelection {
        &self.current_selection
    }

    pub fn get_current_cam(&self) -> &usize {
        &self.camera_device_1_index
    }

    fn search_audio_devices(&self) -> Vec<String> {
        let mut audio_device: Vec<String> = Vec::new();


        let audio_devices = PvRecorderBuilder::default().get_available_devices();
        match audio_devices {
            Ok(audio_devices) => {
                for (idx, device) in audio_devices.iter().enumerate() {
                    audio_device.push(device.clone());
                }
            }
            Err(err) => audio_device.push("No device found".to_string()),
        };

        audio_device
    }


}