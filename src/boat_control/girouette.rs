use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;
use std::{thread, time::Duration};

pub struct Girouette;

impl Girouette {
    pub fn new() -> Self {
        Self
    }

    pub fn get_raw_value(&self, gpio: &mut Gpio) -> u16 {
        let send_buf = [0b00000001, 0b10000000, 0b00000000];
        let mut recv_buf = [0u8; 3];
        gpio.spi_transfer(&mut recv_buf, &send_buf);
        ((recv_buf[1] as u16) << 8 | (recv_buf[2] as u16)) & 0x3FF
    }

    pub fn compensate_raw_value(&self, raw_value: u16, degrees_from_north: f32) -> f32 {
         let value_in_degrees = ((raw_value as f32) * 360.0)/1023.0;
         value_in_degrees + degrees_from_north
    }
}