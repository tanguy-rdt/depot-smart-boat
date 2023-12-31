use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;

use std::thread;
use std::time::Duration;

const PCA9685_MODE1: u8 = 0x00;
const PCA9685_MODE2: u8 = 0x01;
const PCA9685_SUBADR1: u8 = 0x02;
const PCA9685_SUBADR2: u8 = 0x03;
const PCA9685_SUBADR3: u8 = 0x04;
const PCA9685_ALLCALLADR: u8 = 0x05;
const PCA9685_LED0_ON_L: u8 = 0x06;
const PCA9685_LED0_ON_H: u8 = 0x07;
const PCA9685_LED0_OFF_L: u8 = 0x08;
const PCA9685_LED0_OFF_H: u8 = 0x09;
const PCA9685_ALLLED_ON_L: u8 = 0xFA;
const PCA9685_ALLLED_ON_H: u8 = 0xFB;
const PCA9685_ALLLED_OFF_L: u8 = 0xFC;
const PCA9685_ALLLED_OFF_H: u8 = 0xFD;
const PCA9685_PRESCALE: u8 = 0xFE;
const PCA9685_TESTMODE: u8 = 0xFF;

const MODE1_ALLCAL: u8 = 0x01;
const MODE1_SUB3: u8 = 0x02;
const MODE1_SUB2: u8 = 0x04;
const MODE1_SUB1: u8 = 0x08;
const MODE1_SLEEP: u8 = 0x10;
const MODE1_AI: u8 = 0x20;
const MODE1_EXTCLK: u8 = 0x40;
const MODE1_RESTART: u8 = 0x80;

const MODE2_OUTNE_0: u8 = 0x01;
const MODE2_OUTNE_1: u8 = 0x02;
const MODE2_OUTDRV: u8 = 0x04;
const MODE2_OCH: u8 = 0x08;
const MODE2_INVRT: u8 = 0x10;

const PCA9685_I2C_ADDRESS: u8 = 0x40;
const FREQUENCY_OSCILLATOR: u32 = 25_000_000;
const PCA9685_PRESCALE_MIN: u8 = 3;
const PCA9685_PRESCALE_MAX: u8 = 255;

// Constant for Servo
const SERVOMIN: u16 = 150; // This is the 'minimum' pulse length count (out of 4096)
const SERVOMAX: u16 = 600; // This is the 'maximum' pulse length count (out of 4096)
const USMIN: u16 = 600;    // This is the rounded 'minimum' microsecond length based on the minimum pulse of 150
const USMAX: u16 = 2400;   // This is the rounded 'maximum' microsecond length based on the maximum pulse of 600
const SERVO_FREQ: u16 = 50; // Analog servos run at ~50 Hz updates
const PWM_FREQUENCY: f32 = 50.0;

pub struct PCA9685;

impl PCA9685 {
    pub fn new() -> Self {
        Self
    }

    pub fn init(&mut self, gpio: &mut Gpio){
        gpio.i2c_set_slave_addr(PCA9685_I2C_ADDRESS);
        self.init_prescaler(gpio, PWM_FREQUENCY);
    }

    fn init_prescaler(&mut self, gpio: &mut Gpio, frequency: f32){
        // Calcule le prescaler nécessaire pour atteindre la fréquence PWM souhaitée
        let prescale_value = (((25000000.0 / (4096.0 * frequency)) + 0.5) - 1.0) as u8;
        gpio.i2c_write_byte(PCA9685_MODE1, MODE1_SLEEP); 
        gpio.i2c_write_byte(PCA9685_PRESCALE, prescale_value); // set prescaler PWM hz to 50 (0x7a)

        // Attends au moins 5 ms (délai spécifié dans la documentation du PCA9685)
        thread::sleep(Duration::from_millis(5)); 
        gpio.i2c_write_byte(PCA9685_MODE1, 0x00); 
    }

    pub fn rotate_servo_clockwise(&mut self, gpio: &mut Gpio, channel: i32){
        gpio.i2c_set_slave_addr(PCA9685_I2C_ADDRESS);
        self.set_led(gpio, channel, 0x199, 0x4CC);
    }

    pub fn rotate_servo_counterclockwise(&mut self, gpio: &mut Gpio, channel: i32){
        gpio.i2c_set_slave_addr(PCA9685_I2C_ADDRESS);
        self.set_led(gpio, channel, 0x199, 0x4CC);
    }

    pub fn set_sleep_mode(&mut self, gpio: &mut Gpio){
        let mut mode1 = self.read_mode1(gpio);

        // Activer le mode sommeil (bit 4 à 1)
        mode1 |= MODE1_SLEEP;
        gpio.i2c_write_byte(PCA9685_MODE1, mode1);    // Control register set to Mode 1 Sleep to set Prescaler

        // Attends au moins 5 ms (délai spécifié dans la documentation du PCA9685)
        thread::sleep(Duration::from_millis(5)); 
        self.read_mode1(gpio);
    }

    pub fn start_all_motor(&mut self, gpio: &mut Gpio){
        self.restart_pwm_channels(gpio);
        self.set_all_led_mode(gpio);
        self.set_all_led(gpio, 0x199, 0x4CC);
    }

    pub fn stop_all_motor(&mut self, gpio: &mut Gpio){
        gpio.i2c_set_slave_addr(PCA9685_I2C_ADDRESS);
        self.set_sleep_mode(gpio);
    }

    fn set_led(&mut self, gpio: &mut Gpio, channel: i32, on: u16, off: u16){
        let on_register = PCA9685_LED0_ON_L + (4 * channel) as u8;
        let off_register = PCA9685_LED0_OFF_L + (4 * channel) as u8;
  
        gpio.i2c_write_byte(on_register, (on & 0xFF) as u8);
        gpio.i2c_write_byte(on_register+1, ((on >> 8) & 0xFF) as u8);
        gpio.i2c_write_byte(off_register, (off & 0xFF) as u8);
        gpio.i2c_write_byte(off_register+1, ((off >> 8) & 0xFF) as u8);  
    }

    fn restart_pwm_channels(&mut self, gpio: &mut Gpio) {
        let mode = self.read_mode1(gpio);
        
        // Vérifier que le bit 7 (RESTART) est à 1 logique
        if (mode & MODE1_RESTART) != 0 {
            self.reset_sleep_mode1(gpio); // Si c'est le cas, effacer le bit 4 (SLEEP)
            gpio.i2c_write_byte(PCA9685_MODE1, MODE1_RESTART); // Écrire la logique 1 dans le bit 7 de MODE1 pour redémarrer tous les canaux PWM
        }
    }

    fn set_all_led_mode(&mut self, gpio: &mut Gpio){
        gpio.i2c_write_byte(PCA9685_MODE1, MODE1_ALLCAL);
        thread::sleep(Duration::from_millis(5)); 
    }

    fn set_all_led(&mut self, gpio: &mut Gpio, on: u16, off: u16){
        gpio.i2c_write_byte(PCA9685_ALLLED_ON_L, (on & 0xFF) as u8);
        gpio.i2c_write_byte(PCA9685_ALLLED_ON_H, ((on >> 8) & 0xFF) as u8);
        gpio.i2c_write_byte(PCA9685_ALLLED_OFF_L, (off & 0xFF) as u8);
        gpio.i2c_write_byte(PCA9685_ALLLED_OFF_H, ((off >> 8) & 0xFF) as u8);   
    }  

    fn reset_sleep_mode1(&mut self, gpio: &mut Gpio){
        let mut mode1 = self.read_mode1(gpio);
        mode1 &= !MODE1_SLEEP;
        gpio.i2c_write_byte(PCA9685_MODE1, mode1);

        // Attends au moins 5 ms (délai spécifié dans la documentation du PCA9685)
        thread::sleep(Duration::from_millis(5)); 
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


   
