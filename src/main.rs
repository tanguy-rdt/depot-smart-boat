mod code;

use crate::code::PCA9685;

use std::thread;
use std::time::Duration;

// Fréquence PWM souhaitée (Hz)
const PWM_FREQUENCY: f32 = 200.0;

// Constant for Servo
const SERVOMIN: u16 = 150; // This is the 'minimum' pulse length count (out of 4096)
const SERVOMAX: u16 = 600; // This is the 'maximum' pulse length count (out of 4096)

fn main() {

    let mut my_pca9685 = PCA9685::new();

    my_pca9685.init_pca_addr();

    my_pca9685.init_prescaler(PWM_FREQUENCY);

    my_pca9685.set_all_led_mode();

    my_pca9685.set_all_led(0x199, 0x4CC);

    thread::sleep(Duration::from_millis(5000));

    my_pca9685.set_sleep_mode();

    my_pca9685.set_all_led(0x00, 0x73A);

    //my_pca9685.reset_sleep_mode();

    my_pca9685.read_mode1();

    
}