use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;

use std::thread;
use std::time::Duration;

const PCA9685_I2C_ADDRESS: u8 = 0x40;
const FREQUENCY_OSCILLATOR: f32 = 25000000.0;
const PCA9685_PRESCALE_MIN: u8 = 3;
const PCA9685_PRESCALE_MAX: u8 = 255;

const PCA9685_MODE1: u8 = 0x00;
const PCA9685_LED0_ON_L: u8 = 0x06;
const PCA9685_PRESCALE: u8 = 0xFE;

const MODE1_SLEEP: u8 = 0x10;
const MODE1_AI: u8 = 0x20;
const MODE1_RESTART: u8 = 0x80;

// Constant for Servo
const SERVOMIN: u16 = 150; // This is the 'minimum' pulse length count (out of 4096)
const SERVOMAX: u16 = 600; // This is the 'maximum' pulse length count (out of 4096)
const SERVO_FREQ: f32 = 50.0;
const PRESCALE_MIN: u8 = 3;  
const PRESCALE_MAX: u8 = 255;

pub struct PCA9685;

impl PCA9685 {
    pub fn new() -> Self {
        Self
    }

    pub fn init(&mut self, gpio: &mut Gpio){
        gpio.i2c_set_slave_addr(PCA9685_I2C_ADDRESS);
        self.init_prescaler(gpio, SERVO_FREQ);
    }

    fn init_prescaler(&mut self, gpio: &mut Gpio, frequency: f32){
        // Calcule le prescaler nécessaire pour atteindre la fréquence PWM souhaitée
        let mut prescale_value = (((FREQUENCY_OSCILLATOR / (4096.0 * frequency)) + 0.5) - 1.0) as u8;
        
        if prescale_value < PRESCALE_MIN {
            prescale_value = PRESCALE_MIN;
        }
        else if prescale_value > PRESCALE_MAX {
            prescale_value = PRESCALE_MAX;
        }

        gpio.i2c_write_byte(PCA9685_MODE1, MODE1_SLEEP); 
        let _ = self.read_mode1(gpio);
        gpio.i2c_write_byte(PCA9685_PRESCALE, prescale_value); // set prescaler PWM hz to 50 (0x7a)
        thread::sleep(Duration::from_millis(5)); 

        gpio.i2c_write_byte(PCA9685_MODE1, MODE1_RESTART | MODE1_AI); 
        let _ = self.read_mode1(gpio); 
    }

    pub fn rotate_servo_clockwise(&mut self, gpio: &mut Gpio, channel: i32){
        gpio.i2c_set_slave_addr(PCA9685_I2C_ADDRESS);
        self.set_led(gpio, channel, 0, 184);
        let _ = self.read_mode1(gpio);
    }

    pub fn rotate_servo_clockwise_n_degree(&mut self, gpio: &mut Gpio, channel: i32, n_turn: f32) {
        let degree: f32 = 360.0 * n_turn;
        let time_for_full_rotation_ms = 800; 
    
        let sleep_time_ms = (degree / 360.0) * time_for_full_rotation_ms as f32;
    
        self.rotate_servo_clockwise(gpio, channel);
        thread::sleep(Duration::from_millis(sleep_time_ms as u64));
        self.stop_motor(gpio, channel);
    }

    pub fn rotate_servo_counterclockwise(&mut self, gpio: &mut Gpio, channel: i32){
        gpio.i2c_set_slave_addr(PCA9685_I2C_ADDRESS);
        self.set_led(gpio, channel, 0, 430);
    }

    pub fn rotate_servo_counterclockwise_n_degree(&mut self, gpio: &mut Gpio, channel: i32, n_turn: f32) {
        let degree: f32 = 360.0 * n_turn;
        let time_for_full_rotation_ms = 800; 
    
        let sleep_time_ms = (degree / 360.0) * time_for_full_rotation_ms as f32;
    
        self.rotate_servo_counterclockwise(gpio, channel);
        thread::sleep(Duration::from_millis(sleep_time_ms as u64));
        self.stop_motor(gpio, channel);
    }

    pub fn stop_motor(&mut self, gpio: &mut Gpio, channel: i32){
        gpio.i2c_set_slave_addr(PCA9685_I2C_ADDRESS);
        self.set_led(gpio, channel, 0, 307);
    }

    fn set_led(&mut self, gpio: &mut Gpio, channel: i32, on: u16, off: u16){
        let on_register = PCA9685_LED0_ON_L + (4 * channel) as u8;

        println!("on_register: {}", on_register);

        let buffer = [(on & 0xFF) as u8, 
                      ((on >> 8) & 0xFF) as u8, 
                      (off & 0xFF) as u8, 
                      ((off >> 8) & 0xFF) as u8];
        gpio.i2c_write_bytes(on_register, &buffer);
    }

    fn read_mode1(&mut self, gpio: &mut Gpio) -> u8 {
        let mode1 = gpio.i2c_read_byte_from(PCA9685_MODE1);

        #[cfg(debug_assertions)]{
            match mode1 & 0x0F {
                0x00 => print!(""),
                0x01 => print!("ALLCALL "),
                0x02 => print!("SUB 3 "),
                0x03 => print!("ALLCALL and SUB 3 "),
                0x04 => print!("SUB 2 "),
                0x05 => print!("ALLCALL and SUB 2 "),
                0x06 => print!("SUB 3 and SUB 2 "),
                0x07 => print!("ALLCALL and SUB 3 and SUB 2 "),
                0x08 => print!("SUB 1 "),
                0x09 => print!("ALLCALL and SUB 1 "),
                0x0A => print!("SUB 3 and SUB 1 "),
                0x0B => print!("ALLCALL and SUB 3 and SUB 1 "),
                0x0C => print!("SUB 2 and SUB 1 "),
                0x0D => print!("ALLCALL and SUB 2 and SUB 1 "),
                0x0E => print!("SUB 3 and SUB 2 and SUB 1 "),
                0x0F => print!("ALLCALL and SUB 3 and SUB 2 and SUB 1 "),
                _ => print!("Unknown MODE1 value: 0x{:02X}", mode1),
            }
            
            if mode1 & 0x0F != 0x00 { print!("and "); }
            
            match mode1 & 0xF0 {
                0x00 => print!("NO MODE"),
                0x10 => print!("SLEEP MODE"),
                0x20 => print!("AI MODE"),
                0x30 => print!("SLEEP and AI MODE"),
                0x40 => print!("EXTCLK MODE"),
                0x50 => print!("SLEEP and EXTCLK MODE"),
                0x60 => print!("AI and EXTCLK MODE"),
                0x70 => print!("SLEEP and AI MODE and EXTCLK MODE"),
                0x80 => print!("RESTART MODE"),
                0x90 => print!("SLEEP and RESTART MODE"),
                0xA0 => print!("AI and RESTART MODE"),
                0xB0 => print!("SLEEP and AI and RESTART MODE"),
                0xC0 => print!("EXTCLK and RESTART MODE"),
                0xD0 => print!("SLEEP and EXTCLK and RESTART MODE"),
                0xE0 => print!("AI and EXTCLK and RESTART MODE"),
                0xF0 => print!("SLEEP and AI MODE and EXTCLK and RESTART MODE"),
                _ => print!("Unknown MODE1 value: 0x{:02X}", mode1),
            }

            println!(" 0x{:02X}", mode1);
        }
        mode1
    }
}


   
