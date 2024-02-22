use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use std::{thread, time::Duration};

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
        let i2c_result = I2c::with_bus(1);

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

    fn i2c_set_slave_addr(&mut self, addr: u8) {
        if let Err(e) = self.i2c.set_slave_address(addr as u16) {
            eprintln!("Error defining I2C slave address 0x{:x}: {:?}", addr, e);
        }
    }    

    fn i2c_read_byte_from(&self, register: u8) -> u8 {
        let mut buf = [0u8; 1];
        if let Err(e) = self.i2c.block_read(register, &mut buf) {
            eprintln!("Error reading the I2C register 0x{:x}: {:?}", register, e);
            return 0; 
        }
        buf[0]
    }

    fn i2c_read_bytes_from(&self, register: u8, buffer: &mut [u8]) {
        if let Err(e) = self.i2c.block_read(register, &mut buffer) {
            eprintln!("Error reading the I2C register 0x{:x}: {:?}", register, e);
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
        }
    }

    fn i2c_write_byte(&self, register: u8, value: u8) {
        if let Err(e) = self.i2c.block_write(register as u8, &[value]) {
            eprintln!("Error writing to the I2C register 0x{:x}: {:?}", register, e);
        }
    }    

    fn i2c_write_bytes(&self, register: u8, values: &[u8]) {
        if let Err(e) = self.i2c.block_write(register as u8, values) {
            eprintln!("Error writing to the I2C register 0x{:x}: {:?}", register, e);
        }
    }
}