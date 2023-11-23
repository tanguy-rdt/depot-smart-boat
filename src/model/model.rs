use crate::boat_control::BoatControler;

pub struct Model{
    boat_controler: BoatControler,
    mainsail_angle: i8,
    foque_angle: i8,
    temperature: f32,
    pressure: f32,
    humidity: f32
}

impl Model {
    pub fn new() -> Self {
        Model {
            boat_controler: BoatControler::new(),
            mainsail_angle: 0,
            foque_angle: 0,
            temperature: 0.0,
            pressure: 0.0,
            humidity: 0.0
        }
    }

    pub fn init_model(&mut self) {
        self.boat_controler.init();
    }

    pub fn get_temperature(&mut self) -> f32 {
        self.temperature = self.boat_controler.get_temperature();
        self.temperature
    }

    pub fn get_pressure(&mut self) -> f32 {
        self.pressure = self.boat_controler.get_pressure();
        self.pressure
    }

    pub fn get_humidity(&mut self) -> f32 {
        self.humidity = self.boat_controler.get_humidity();
        self.humidity
    }

    pub fn get_mainsail_angle(&self) -> i8 {
        self.mainsail_angle
    }

    fn set_mainsail_angle(&mut self, angle: i8) {
        self.mainsail_angle = angle;
    }

    pub fn get_foque_angle(&self) -> i8 {
        self.foque_angle
    }

    fn set_foque_angle(&mut self, angle: i8) {
        self.foque_angle = angle;
    }

    fn direction_tribord(&mut self){
        self.set_mainsail_angle(1);
        self.set_foque_angle(1);
    }

    fn direction_babord(&mut self){
        self.set_mainsail_angle(0);
        self.set_foque_angle(0);
    }

    pub fn treat_action(&mut self, action: &str){
        match action {
            "direction_tribord" => self.direction_tribord(),
            "direction_babord" => self.direction_babord(),
            _ => (),
        };

    }
}