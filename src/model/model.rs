use crate::boat_control::BoatControler;
use std::sync::{mpsc, Arc, Mutex};
use rand::distributions::{Distribution, Uniform};

pub struct Model{
    boat_controler: BoatControler,
    mainsail_angle: i8,
    foque_angle: i8,
    temperature: f32,
    pressure: f32,
    humidity: f32,
    tx_gui: Arc<Mutex<mpsc::Sender<(String, f32)>>>,
}

impl Model {
    pub fn new(tx_gui: Arc<Mutex<mpsc::Sender<(String, f32)>>>) -> Self {
        Model {
            boat_controler: BoatControler::new(),
            mainsail_angle: 0,
            foque_angle: 0,
            temperature: 0.0,
            pressure: 0.0,
            humidity: 0.0, 
            tx_gui: tx_gui
        }
    }

    pub fn init_model(&mut self) {
        self.boat_controler.init();
    }

    pub fn get_temperature(&mut self) -> f32 {
        self.temperature = self.boat_controler.get_temperature();
        let mut rng = rand::thread_rng();
        let temperature_range = Uniform::new_inclusive(19.0, 30.0);
        self.temperature = temperature_range.sample(&mut rng);
        self.tx_gui
        .lock()
        .unwrap()
        .send(("temperature".to_string(), self.temperature))
        .unwrap();
        self.temperature
    }

    pub fn get_pressure(&mut self) -> f32 {
        self.pressure = self.boat_controler.get_pressure();
        let mut rng = rand::thread_rng();
        let pressure_range = Uniform::new_inclusive(900.0, 1000.0);
        self.pressure = pressure_range.sample(&mut rng);
        self.tx_gui
        .lock()
        .unwrap()
        .send(("pressure".to_string(), self.pressure))
        .unwrap();
        self.pressure
    }

    pub fn get_humidity(&mut self) -> f32 {
        self.humidity = self.boat_controler.get_humidity();
        let mut rng = rand::thread_rng();
        let humidity_range = Uniform::new_inclusive(50.0, 100.0);
        self.humidity = humidity_range.sample(&mut rng);
        self.tx_gui
        .lock()
        .unwrap()
        .send(("humidity".to_string(), self.humidity))
        .unwrap();
        self.humidity
    }

    fn set_mainsail_angle(&mut self, angle: i8) {
        self.mainsail_angle = angle;
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

    fn motor(&mut self, val: bool){
        //println!("motor status {}", val);
        self.boat_controler.start_all_motor();
    }

    pub fn treat_action(&mut self, var: &str, val: f32){
        match var {
            "direction_tribord" => self.direction_tribord(),
            "direction_babord" => self.direction_babord(),
            "motor" => self.motor(val != 0.0),
            _ => (),
        };
    }
}