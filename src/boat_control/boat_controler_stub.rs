use crate::boat_control::boat_controler_itf::BoatControlerItf;

pub struct BoatControlerStub;

impl BoatControlerItf for BoatControlerStub {
    fn new() -> Self {
        Self
    }

    fn init(&mut self){
    }

    fn get_temperature(&mut self) -> f32{
        //self.bme280.get_temperature();
        90.89
    }

    fn get_pressure(&self) -> f32{
        //self.bme280.get_pressure();
        543.564
    }

    fn get_humidity(&self) -> f32{
        //self.bme280.get_humidity();
        5.0
    }

    fn start_all_motor(&mut self){
    }

    fn stop_all_motor(&mut self){
    }

    fn positionMainSailToPort(&mut self){
    }

    fn stopPositionMainSailToPort(&mut self){
    }

    fn positionMainSailToStartBoard(&mut self){
    }

    fn positionJibToPort(&mut self){
    }

    fn positionJibToStartBoard(&mut self){
    }


}
