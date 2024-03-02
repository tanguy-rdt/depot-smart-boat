use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;
use crate::boat_control::boat_controler_itf::BoatControlerItf;
use crate::boat_control::bme280::BME280;
use crate::boat_control::bmm150::BMM150;
use crate::boat_control::pca9685::PCA9685;
use crate::boat_control::girouette::Girouette;
use crate::boat_control::hcsr05::HCSRO5;

pub struct BoatControler{
    gpio: Gpio,
    bme280: BME280,
    bmm150: BMM150,
    pca9685: PCA9685,
    girouette: Girouette,
    hcsr05: HCSRO5,
    current_mainsail_angle: f32,
    current_jib_angle: f32,
    current_mainsail_height: f32,

}

impl BoatControlerItf for BoatControler {
    fn new() -> Self {
        BoatControler {
            gpio: Gpio::new(),
            bme280: BME280::new(),
            bmm150: BMM150::new(),
            pca9685: PCA9685::new(),
            girouette: Girouette::new(),
            hcsr05: HCSRO5::new(0, 1),
            current_mainsail_angle: 0.5,
            current_jib_angle: 0.5,
            current_mainsail_height: 0.0,
        }
    }

    fn init(&mut self){
        self.bme280.init(&mut self.gpio);
        self.bmm150.init(&mut self.gpio);
        self.pca9685.init(&mut self.gpio);
        self.hcsr05.init(&mut self.gpio);
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

    fn get_geomagnetic(&mut self) -> (i16, i16, i16) {
        self.bmm150.get_geomagnetic(&mut self.gpio)
    }

    fn get_deep(&mut self) -> f32 {
        self.hcsr05.get_value_m(&mut self.gpio)
    }

    fn get_boat_direction_degree(&mut self) -> f32{
        let (x, y, z) = self.bmm150.get_geomagnetic(&mut self.gpio);
        let mut compass = (x as f32).atan2(y as f32);
        if compass < 0.0 { compass += 2.0 * std::f32::consts::PI; }
        if compass > (2.0 * std::f32::consts::PI) { compass -= 2.0 * std::f32::consts::PI; }
        compass = (compass * 180.0 / std::f32::consts::PI) -70.0;
        format!("{:.2}", compass).parse().unwrap()
    }

    fn get_wind_direction_degree(&mut self) -> f32{
        let raw_value = self.girouette.get_raw_value(&mut self.gpio);
        let boat_direction = self.get_boat_direction_degree();
        self.girouette.compensate_raw_value(raw_value, boat_direction)
    }

    fn move_mainail_to(&mut self, position: f32){
        let n_turn_complete = 1.0;
        let factor = position - self.current_mainsail_angle;
        let n_turn = (n_turn_complete * factor).abs();

        match position {
            p if p > self.current_mainsail_angle => {
                self.pca9685.rotate_servo_clockwise_n_degree(&mut self.gpio, 1, n_turn)
            },
            p if p < self.current_mainsail_angle => {
                self.pca9685.rotate_servo_counterclockwise_n_degree(&mut self.gpio, 1, n_turn)
            },
            _ => {}
        };
        self.current_mainsail_angle = position;
    }

    fn up_down_mainsail(&mut self, position: f32){
        let n_turn_complete = 13.0;
        let factor = position - self.current_mainsail_height;
        let n_turn = (n_turn_complete * factor).abs();

        match position {
            p if p > self.current_mainsail_height => {
                self.pca9685.rotate_servo_clockwise_n_degree(&mut self.gpio, 3, n_turn)
            },
            p if p < self.current_mainsail_height => {
                self.pca9685.rotate_servo_counterclockwise_n_degree(&mut self.gpio, 3, n_turn)
            },
            _ => {}
        };
        self.current_mainsail_height = position;
    }

    fn move_jib_to(&mut self, position: f32){
        let n_turn_complete = 0.5;
        let factor = position - self.current_jib_angle;
        let n_turn = (n_turn_complete * factor).abs();

        match position {
            p if p > self.current_jib_angle => {
                self.pca9685.rotate_servo_clockwise_n_degree(&mut self.gpio, 0, n_turn)
            },
            p if p < self.current_jib_angle => {
                self.pca9685.rotate_servo_counterclockwise_n_degree(&mut self.gpio, 0, n_turn)
            },
            _ => {}
        };
        self.current_jib_angle = position;
    }
}
