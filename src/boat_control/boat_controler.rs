use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;
use crate::boat_control::boat_controler_itf::BoatControlerItf;
use crate::boat_control::bme280::BME280;
use crate::boat_control::pca9685::PCA9685;

pub enum SailPosition {
    Starboard, //tribord, droite
    ToPort, //babord, gauche
    Up, 
    Down,
}


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

    fn move_mainail_to(&mut self, position: SailPosition){
        match position {
            SailPosition::Starboard => self.pca9685.rotate_servo_clockwise_n_degree(&mut self.gpio, 1, 1.0),
            SailPosition::ToPort => self.pca9685.rotate_servo_counterclockwise_n_degree(&mut self.gpio, 1, 1.0),
            _ => {}
        }
    }

    fn up_down_mainsail(&mut self, position: SailPosition){
        match position {
            SailPosition::Up => self.pca9685.rotate_servo_clockwise_n_degree(&mut self.gpio, 3, 1.0),
            SailPosition::Down => self.pca9685.rotate_servo_counterclockwise_n_degree(&mut self.gpio, 3, 1.0),
            _ => {}
        }    
    }

    fn move_jib_to(&mut self, position: SailPosition){
        match position {
            SailPosition::Starboard => self.pca9685.rotate_servo_clockwise_n_degree(&mut self.gpio, 0, 1.5),
            SailPosition::ToPort => self.pca9685.rotate_servo_counterclockwise_n_degree(&mut self.gpio, 0, 1.5),
            _ => {}
        }    
    }
}
