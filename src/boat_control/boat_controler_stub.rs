use crate::boat_control::boat_controler_itf::BoatControlerItf;
use crate::boat_control::boat_controler::SailPosition;

use rand::distributions::{Distribution, Uniform};

pub struct BoatControlerStub;

impl BoatControlerItf for BoatControlerStub {
    fn new() -> Self {
        Self
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

    fn move_mainail_to(&mut self, position: SailPosition){}
    fn up_down_mainsail(&mut self, position: SailPosition){}
    fn move_jib_to(&mut self, position: SailPosition){}
}
