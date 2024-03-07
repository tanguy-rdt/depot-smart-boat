use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;
use crate::boat_control::boat_controler_itf::BoatControlerItf;
use crate::boat_control::bme280::BME280;
use crate::boat_control::bmm150::BMM150;
use crate::boat_control::pca9685::PCA9685;
use crate::boat_control::girouette::Girouette;
use crate::boat_control::hcsr05::HCSRO5;

const HEADWIND: f32 = 0.5;
const CLOSEWIND_BABORD: f32 = 0.375;
const CROSSWIND_BABORD: f32 = 0.250;
const BEAMWIND_BABORD: f32 = 0.125;
const DOWNWIND: f32 = 0.0;
const BEAMWIND_TRIBORD: f32 = 0.875;
const CROSSWIND_TRIBORD: f32 = 0.750;
const CLOSEWIND_TRIBORD: f32 = 0.625;

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
            hcsr05: HCSRO5::new(23, 24),
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
        - self.hcsr05.get_value_cm(&mut self.gpio) * 10.0
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

    fn automation(&mut self) { 

        let wind_direction_degree = self.get_wind_direction_degree();

        match wind_direction_degree {
            w if w <= 22.5 => {
                self.move_mainail_to(HEADWIND);
                self.move_jib_to(HEADWIND);
            },
            w if w <= 67.5 => {
                self.move_mainail_to(CLOSEWIND_BABORD);
                self.move_jib_to(CLOSEWIND_BABORD);
            },
            w if w <= 112.5 => {
                self.move_mainail_to(CROSSWIND_BABORD);
                self.move_jib_to(CROSSWIND_BABORD);
            },
            w if w <= 157.5 => {
                self.move_mainail_to(BEAMWIND_BABORD);
                self.move_jib_to(BEAMWIND_BABORD);
            },
            w if w <= 202.5 => {
                self.move_mainail_to(DOWNWIND);
                self.move_jib_to(DOWNWIND);
            },
            w if w <= 247.5 => {
                self.move_mainail_to(BEAMWIND_TRIBORD);
                self.move_jib_to(BEAMWIND_TRIBORD);
            },
            w if w <= 292.5 => {
                self.move_mainail_to(CROSSWIND_TRIBORD);
                self.move_jib_to(CROSSWIND_TRIBORD);
            },
            w if w <= 337.5 => {
                self.move_mainail_to(CLOSEWIND_TRIBORD);
                self.move_jib_to(CLOSEWIND_TRIBORD);
            },
            w if w <= 360.0 => {
                self.move_mainail_to(HEADWIND);
                self.move_jib_to(HEADWIND);
            },
            _ => {}
        };
    }
}
