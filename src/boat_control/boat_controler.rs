use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;
use crate::boat_control::boat_controler_itf::BoatControlerItf;
use crate::boat_control::bme280::BME280;
use crate::boat_control::pca9685::PCA9685;


pub struct BoatControler{
    gpio: Gpio,
    bme280: BME280,
    pca9685: PCA9685
}

impl BoatControlerItf for BoatControler {
    fn new() -> Self {
        BoatControler {
            gpio: Gpio::new(),
            bme280: BME280::new(),
            pca9685: PCA9685::new()
        }
    }

    fn init(&mut self){
        self.bme280.init(&mut self.gpio);
        self.pca9685.init(&mut self.gpio);
    }

    fn get_temperature(&mut self) -> f32{
        self.bme280.get_temperature(&mut self.gpio)
    }

    fn get_pressure(&mut self) -> f32{
        self.bme280.get_pressure(&mut self.gpio)
    }

    fn get_humidity(&mut self) -> f32{
        self.bme280.get_humidity(&mut self.gpio)
    }

    fn start_all_motor(&mut self){
        self.pca9685.start_all_motor(&mut self.gpio);
    }

    fn stop_all_motor(&mut self){
        self.pca9685.stop_all_motor(&mut self.gpio);
    }

    fn positionMainSailToPort(&mut self){
        self.pca9685.rotate_servo_clockwise(&mut self.gpio, 0);
    }

    fn stopPositionMainSailToPort(&mut self){
        self.pca9685.stop_all_motor(&mut self.gpio);
    }

    fn positionMainSailToStartBoard(&mut self){
        self.pca9685.rotate_servo_counterclockwise(&mut self.gpio, 0);
    }

    fn positionJibToPort(&mut self){
        self.pca9685.rotate_servo_clockwise(&mut self.gpio, 1);
    }

    fn positionJibToStartBoard(&mut self){
        self.pca9685.rotate_servo_counterclockwise(&mut self.gpio, 1);
    }


}
