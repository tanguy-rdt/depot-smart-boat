use crate::gpio_manager::gpio_itf::GpioItf;
use crate::gpio_manager::Gpio;

pub struct Model{
    gpio: Gpio,
    mainsail_angle: i8,
    foque_angle: i8
}

impl Model {
    pub fn new() -> Self {
        Model {
            gpio: Gpio::new(),
            mainsail_angle: 0,
            foque_angle: 0,
        }
    }

    pub fn init_model(&self) {
        &self.gpio.init();
    }

    pub fn get_mainsail_angle(&self) -> i8 {
        self.mainsail_angle
    }

    fn set_mainsail_angle(&mut self, angle: i8) {
        self.mainsail_angle = angle;

        println!("*********************************");
        println!("         Move motor");
        println!("*********************************");
        println!("Move mainsail to {}", angle);
    }

    pub fn get_foque_angle(&self) -> i8 {
        self.foque_angle
    }

    fn set_foque_angle(&mut self, angle: i8) {
        self.foque_angle = angle;
        println!("*********************************");
        println!("         Move motor");
        println!("*********************************");
        println!("Move foque to {}", angle);
    }

    fn direction_tribord(&mut self){
        self.set_mainsail_angle(1);
        self.set_foque_angle(1);
    }

    fn direction_babord(&mut self){
        self.set_mainsail_angle(0);
        self.set_foque_angle(0);
    }

    pub fn treat_action(&mut self, action: &str){
        match action {
            "direction_tribord" => self.direction_tribord(),
            "direction_babord" => self.direction_babord(),
            _ => (),
        };

    }
}