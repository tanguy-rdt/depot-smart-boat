use eframe::egui;

pub struct Weather{
    temperature: f32,
    humidity: f32,
    pressure: f32,
}

impl Weather{
    pub fn new() -> Self {
        Self {
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0.0
        }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        ui.label(format!("Temperature: {:.2} C", self.temperature));
        ui.label(format!("Humidity: {:.2} %", self.humidity));
        ui.label(format!("Pressure: {:.2} Pa", self.pressure)); 
    }

    pub fn set_temperature(&mut self, value: f32){
        self.temperature = value;
    }

    pub fn set_humidity(&mut self, value: f32){
        self.humidity = value;
    }

    pub fn set_pressure(&mut self, value: f32){
        self.pressure = value;
    }
}