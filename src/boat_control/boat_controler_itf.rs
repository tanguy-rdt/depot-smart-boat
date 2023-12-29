pub trait BoatControlerItf {
    fn new() -> Self;
    fn init(&mut self);
    fn get_temperature(&mut self) -> f32;
    fn get_pressure(&self) -> f32;
    fn get_humidity(&self) -> f32;
    fn start_all_motor(&mut self);
    fn stop_all_motor(&mut self);
    fn positionMainSailToPort(&mut self);
    fn stopPositionMainSailToPort(&mut self);
    fn positionMainSailToStartBoard(&mut self);
    fn positionJibToPort(&mut self);
    fn positionJibToStartBoard(&mut self);
}