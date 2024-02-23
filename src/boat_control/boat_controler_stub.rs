use crate::boat_control::boat_controler_itf::BoatControlerItf;

use rand::distributions::{Distribution, Uniform};

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

    fn get_geomagnetic(&mut self) -> (i16, i16, i16) { (0, 0, 0) }
}
