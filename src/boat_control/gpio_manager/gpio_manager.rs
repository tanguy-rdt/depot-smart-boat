use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use std::{thread, time::Duration};

#[cfg(feature = "on_target")]
use rppal::{gpio::{Gpio, Level}, i2c::I2c};

#[cfg(not(feature = "on_target"))]
pub struct GpioManager;

#[cfg(feature = "on_target")]
pub struct GpioManager{    
    i2c: rppal::i2c::I2c,
    spi: rppal::spi::Spi,
}

#[cfg(feature = "on_target")]
impl GpioItf for GpioManager {
    pub fn new(i2c_bus: u8, spi_bus: rppal::spi::Bus, spi_ss: rppal::spi::SlaveSelect) -> Self {
        let i2c = I2c::with_bus(i2c_bus).expect("Failed to initialize I2C");

        let spi = rppal::spi::Spi::new(spi_bus, spi_ss, 1000000, rppal::spi::Mode::Mode0)
            .expect("Failed to initialize SPI");

        Self { i2c, spi }
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
        if let Err(e) = self.i2c.block_read(register, buffer) {
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

    fn spi_transfer(&mut self, read_buffer: &mut[u8], write_buffer: &[u8]){
        if let Err(e) = self.spi.transfer(read_buffer, write_buffer) {
            eprintln!("Error transfer with spi protocol, erreur {e}");
        }
    }
}