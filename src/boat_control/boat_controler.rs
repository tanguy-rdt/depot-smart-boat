use crate::boat_control::bme280::BME280;
use crate::boat_control::pca9685::PCA9685;


pub struct BoatControler{
    //bme280: BME280
    pca9685: PCA9685
}

impl BoatControler {
    pub fn new() -> Self {
        BoatControler {
            //bme280: BME280::new()
            pca9685: PCA9685::new()
        }
    }

    pub fn init(&mut self){
        //self.bme280.init();
        self.pca9685.init();
    }

    pub fn get_temperature(&mut self) -> f32{
        //self.bme280.get_temperature();
        90.89
    }

    pub fn get_pressure(&self) -> f32{
        //self.bme280.get_pressure();
        543.564
    }

    pub fn get_humidity(&self) -> f32{
        //self.bme280.get_humidity();
        5.0
    }

    pub fn start_all_motor(&mut self){
        self.pca9685.start_all_motor();
    }

    pub fn positionMainSailToPort(&mut self){
        self.pca9685.positionMainSailToPort();
    }

    pub fn positionMainSailToStartBoard(&mut self){
        self.pca9685.positionMainSailToStartBoard();
    }

    pub fn positionJibToPort(&mut self){
        self.pca9685.positionJibToPort();
    }

    pub fn positionJibToStartBoard(&mut self){
        self.pca9685.positionJibToStartBoard();
    }


}
