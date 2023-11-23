use crate::boat_control::gpio_manager::gpio_itf::GpioItf;

#[cfg(feature = "on_target")]
use rppal::{gpio::{Gpio, Level}, i2c::I2c};


#[cfg(not(feature = "on_target"))]
pub struct GpioManager;

#[cfg(feature = "on_target")]
pub struct GpioManager{    
    i2c: rppal::i2c::I2c
}

#[cfg(feature = "on_target")]
impl GpioItf for GpioManager {
    fn new() -> Self {
        let i2c_result = I2c::new();

        match i2c_result {
            Ok(i2c) => {
                println!("I2C initialized successfully!");
                GpioManager { i2c }
            }
            Err(err) => {
                eprintln!("Error initializing I2C: {}", err);
                panic!("Error initializing I2C");
            }
        }
    }

    fn init(&self){
        println!("Im the init in rpi mod");
    }

    fn i2c_set_slave_addr(&mut self, addr: u8){
        self.i2c.set_slave_address(addr as u16).unwrap();;
    }

    fn i2c_read_byte_from(&self, register: u8) -> u8{
        let mut buf = [0u8; 1];
        self.i2c.block_read(register, &mut buf).unwrap();
        buf[0]
    }

    fn i2c_write_byte(&self, register: u8, value: u8){
        self.i2c.block_write(register as u8, &[value]).unwrap();
    }
}