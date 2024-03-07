use crate::boat_control::boat_controler_itf::BoatControlerItf;
use rand::distributions::{Distribution, Uniform};

const HEADWIND: f32 = 0.5;
const CLOSEWIND_BABORD: f32 = 0.375;
const CROSSWIND_BABORD: f32 = 0.250;
const BEAMWIND_BABORD: f32 = 0.125;
const DOWNWIND: f32 = 0.0;
const BEAMWIND_TRIBORD: f32 = 0.875;
const CROSSWIND_TRIBORD: f32 = 0.750;
const CLOSEWIND_TRIBORD: f32 = 0.625;

pub struct BoatControlerStub {
    current_mainsail_angle: f32,
    current_jib_angle: f32,
    current_mainsail_height: f32,
}

impl BoatControlerItf for BoatControlerStub {
    fn new() -> Self {
        Self {
            current_mainsail_angle: 0.5,
            current_jib_angle: 0.5,
            current_mainsail_height: 0.0,

        }    
    }

    fn init(&mut self){
    }

    fn get_temperature(&mut self) -> f32{
        let mut rng = rand::thread_rng();
        let termp_range = Uniform::new_inclusive(20.0, 21.0);
        termp_range.sample(&mut rng)
    }

    fn get_pressure(&mut self) -> f32{
        let mut rng = rand::thread_rng();
        let pressure_range = Uniform::new_inclusive(900.0, 1000.0);
        pressure_range.sample(&mut rng)
    }

    fn get_humidity(&mut self) -> f32{
        let mut rng = rand::thread_rng();
        let hum_range = Uniform::new_inclusive(65.0, 66.0);
        hum_range.sample(&mut rng)
    }

    fn get_boat_direction_degree(&mut self) -> f32{
        let mut rng = rand::thread_rng();
        let boat_direction = Uniform::new_inclusive(0.0, 360.0);
        boat_direction.sample(&mut rng)
    }

    fn get_wind_direction_degree(&mut self) -> f32{
        let mut rng = rand::thread_rng();
        let wind_direction = Uniform::new_inclusive(0.0, 360.0);
        wind_direction.sample(&mut rng)
    }

    fn get_deep(&mut self) -> f32 {
        let mut rng = rand::thread_rng();
        let deep = Uniform::new_inclusive(-350.0, -300.0);
        deep.sample(&mut rng)
    }

    fn move_mainail_to(&mut self, position: f32) { 
        let n_turn_complete = 1.0;
        let factor = position - self.current_mainsail_angle;
        let n_turn = (n_turn_complete * factor).abs();
        self.current_mainsail_angle = position;

        println!("move_mainail_to {position}, real movement {factor}, with {n_turn} turn");
    }

    fn up_down_mainsail(&mut self, position: f32) { 
        let n_turn_complete = 13.0;
        let factor = position - self.current_mainsail_height;
        let n_turn = (n_turn_complete * factor).abs();
        self.current_mainsail_height = position;

        println!("up_down_mainsail {position}, real movement {factor}, with {n_turn} turn");
    }

    fn move_jib_to(&mut self, position: f32) {
        let n_turn_complete = 1.5;
        let factor = position - self.current_jib_angle;
        let n_turn = (n_turn_complete * factor).abs();
        self.current_jib_angle = position;

        println!("move_jib_to {position}, real movement {factor}, with {n_turn} turn");
    }

    fn automation(&mut self) { 

        let wind_direction_degree = self.get_wind_direction_degree();

        match wind_direction_degree {
            w if w <= 22.5 => {
                self.move_mainail_to(HEADWIND);
                self.move_jib_to(HEADWIND);
            },
            w if w <= 67.5 => {
                self.move_mainail_to(CLOSEWIND_BABORD);
                self.move_jib_to(CLOSEWIND_BABORD);
            },
            w if w <= 112.5 => {
                self.move_mainail_to(CROSSWIND_BABORD);
                self.move_jib_to(CROSSWIND_BABORD);
            },
            w if w <= 157.5 => {
                self.move_mainail_to(BEAMWIND_BABORD);
                self.move_jib_to(BEAMWIND_BABORD);
            },
            w if w <= 202.5 => {
                self.move_mainail_to(DOWNWIND);
                self.move_jib_to(DOWNWIND);
            },
            w if w <= 247.5 => {
                self.move_mainail_to(BEAMWIND_TRIBORD);
                self.move_jib_to(BEAMWIND_TRIBORD);
            },
            w if w <= 292.5 => {
                self.move_mainail_to(CROSSWIND_TRIBORD);
                self.move_jib_to(CROSSWIND_TRIBORD);
            },
            w if w <= 337.5 => {
                self.move_mainail_to(CLOSEWIND_TRIBORD);
                self.move_jib_to(CLOSEWIND_TRIBORD);
            },
            w if w <= 360.0 => {
                self.move_mainail_to(HEADWIND);
                self.move_jib_to(HEADWIND);
            },
            _ => {}
        };
    }

    fn get_geomagnetic(&mut self) -> (i16, i16, i16) { (0, 0, 0) }
}
