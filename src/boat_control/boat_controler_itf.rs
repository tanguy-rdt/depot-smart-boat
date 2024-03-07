pub trait BoatControlerItf {
    fn new() -> Self;
    fn init(&mut self);
    fn get_temperature(&mut self) -> f32;
    fn get_pressure(&mut self) -> f32;
    fn get_humidity(&mut self) -> f32;
    fn get_boat_direction_degree(&mut self) -> f32;
    fn get_wind_direction_degree(&mut self) -> f32;
    fn get_deep(&mut self) -> f32;
    fn move_mainail_to(&mut self, position: f32);
    fn up_down_mainsail(&mut self, position: f32);
    fn move_jib_to(&mut self, position: f32);
    fn get_geomagnetic(&mut self) -> (i16, i16, i16);
    fn automation(&mut self) -> f32;
}