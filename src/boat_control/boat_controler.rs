use crate::boat_control::boat_controler_itf::BoatControlerItf;
use crate::boat_control::bme280::BME280;
use crate::boat_control::pca9685::PCA9685;
use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;


pub struct BoatControler{
    gpio: Gpio,
    bme280: BME280,
    pca9685: PCA9685,
}

impl BoatControlerItf for BoatControler {
    fn new() -> Self {
        BoatControler {
            gpio: Gpio::new(),
            bme280: BME280::new(),
            pca9685: PCA9685::new(),
        }
    }

    fn init(&mut self){
        self.bme280.init(&mut self.gpio);
        self.pca9685.init(&mut self.gpio);
    }

    fn get_temperature(&mut self) -> f32{
        self.bme280.get_temperature(&self.gpio)
    }

    fn get_pressure(&self) -> f32{
        self.bme280.get_pressure(&self.gpio)
    }

    fn get_humidity(&self) -> f32{
        self.bme280.get_humidity(&self.gpio)
    }

    fn start_all_motor(&mut self){
        self.pca9685.start_all_motor(&mut self.gpio);
    }

    fn stop_all_motor(&mut self){
        self.pca9685.stop_all_motor(&mut self.gpio);
    }

    fn positionMainSailToPort(&mut self){
        self.pca9685.positionMainSailToPort(&mut self.gpio);
    }

    fn stopPositionMainSailToPort(&mut self){
        self.pca9685.stopPositionMainSailToPort(&mut self.gpio);
    }

    fn positionMainSailToStartBoard(&mut self){
        self.pca9685.positionMainSailToStartBoard(&mut self.gpio);
    }

    fn positionJibToPort(&mut self){
        self.pca9685.positionJibToPort(&mut self.gpio);
    }

    fn positionJibToStartBoard(&mut self){
        self.pca9685.positionJibToStartBoard(&mut self.gpio);
    }


}
