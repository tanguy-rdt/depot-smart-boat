use crate::boat_control::boat_controler::SailPosition;

pub trait BoatControlerItf {
    fn new() -> Self;
    fn init(&mut self);
    fn get_temperature(&mut self) -> f32;
    fn get_pressure(&mut self) -> f32;
    fn get_humidity(&mut self) -> f32;
    fn move_mainail_to(&mut self, position: SailPosition);
    fn up_down_mainsail(&mut self, position: SailPosition);
    fn move_jib_to(&mut self, position: SailPosition);
}