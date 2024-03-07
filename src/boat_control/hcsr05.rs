use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;
use std::{thread, time::Duration};

pub struct HCSRO5 {
    trigger_pin: u8,
    echo_pin: u8,
}

impl HCSRO5 {
    pub fn new(trigger_pin: u8, echo_pin: u8) -> Self {
        Self { 
            trigger_pin, 
            echo_pin 
        }
    }

    pub fn init(&self, gpio: &mut impl GpioItf) {
        gpio.set_output(self.trigger_pin);
        gpio.set_input(self.echo_pin);
        gpio.set_low(self.trigger_pin);
        thread::sleep(Duration::from_millis(2));
    }

    pub fn get_value_m(&self, gpio: &mut Gpio) -> f32 {
        self.read_value(gpio) as f32 / 100.0 
    }

    pub fn get_value_cm(&self, gpio: &mut Gpio) -> f32 {
        self.read_value(gpio) as f32
    }

    fn read_value(&self, gpio: &mut Gpio) -> f64 {
        gpio.set_high(self.trigger_pin);
        thread::sleep(Duration::from_micros(10));
        gpio.set_low(self.trigger_pin);

        while gpio.is_low(self.echo_pin) {}
        let start = std::time::Instant::now();

        while gpio.is_high(self.echo_pin) {}
        let duration = start.elapsed();

        duration.as_secs_f64() * 340.0 / 2.0 * 100.0
    }
}