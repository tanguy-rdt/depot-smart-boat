use crate::boat_control::boat_controler_itf::BoatControlerItf;
use crate::boat_control::bme280::BME280;
use crate::boat_control::pca9685::PCA9685;


pub struct BoatControler{
    bme280: BME280,
    pca9685: PCA9685
}

impl BoatControlerItf for BoatControler {
    fn new() -> Self {
        BoatControler {
            bme280: BME280::new(),
            pca9685: PCA9685::new()
        }
    }

    fn init(&mut self){
        self.bme280.init();
        self.pca9685.init();
    }

    fn get_temperature(&mut self) -> f32{
        self.bme280.get_temperature()
    }

    fn get_pressure(&mut self) -> f32{
        self.bme280.get_pressure()
    }

    fn get_humidity(&mut self) -> f32{
        self.bme280.get_humidity()
    }

    fn start_all_motor(&mut self){
        self.pca9685.start_all_motor();
    }

    fn stop_all_motor(&mut self){
        self.pca9685.stop_all_motor();
    }

    fn positionMainSailToPort(&mut self){
        self.pca9685.rotate_servo_clockwise(0);
    }

    fn stopPositionMainSailToPort(&mut self){
        self.pca9685.stop_all_motor();
    }

    fn positionMainSailToStartBoard(&mut self){
        self.pca9685.rotate_servo_counterclockwise(0);
    }

    fn positionJibToPort(&mut self){
        self.pca9685.rotate_servo_clockwise(1);
    }

    fn positionJibToStartBoard(&mut self){
        self.pca9685.rotate_servo_counterclockwise(1);
    }


}
