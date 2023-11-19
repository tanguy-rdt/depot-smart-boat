// todo: inversement des regs trig et ajustement typage

use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;


const BME280_ADDR       : u8      = 0x00;
const BME280_CONFIG_ADDR: u8      = 0xf5;
const CTRL_MEAS_ADDR    : u8      = 0xf4;

const TEMP_ADDR         : [u8; 3] = [0xfa, 0xfb, 0xfc]; // MSB, LSB, XLSB
const PRESS_ADDR        : [u8; 3] = [0xf7, 0xf8, 0xf9];
const HUM_ADDR          : [u8; 2] = [0xfd, 0xfe]; // MSB, LSB
const HUM_CTRL_ADDR     : u8     = 0xf2;

const T1_ADDR           : [u8; 2] = [0x88, 0x89]; // lsb, msb (inverese les cases) () verifier
const T2_ADDR           : [u8; 2] = [0x8a, 0x8b];
const T3_ADDR           : [u8; 2] = [0x8c, 0x8d];

const P1_ADDR           : [u8; 2] = [0x8e, 0x8f];
const P2_ADDR           : [u8; 2] = [0x90, 0x91];
const P3_ADDR           : [u8; 2] = [0x92, 0x93];
const P4_ADDR           : [u8; 2] = [0x94, 0x95];
const P5_ADDR           : [u8; 2] = [0x96, 0x97];
const P6_ADDR           : [u8; 2] = [0x98, 0x99];
const P7_ADDR           : [u8; 2] = [0x9a, 0x9b];
const P8_ADDR           : [u8; 2] = [0x9c, 0x9d];
const P9_ADDR           : [u8; 2] = [0x9e, 0x9f];

const H1_ADDR           : u8      = 0xa1;
const H2_ADDR           : [u8; 2] = [0xe1, 0xe2];
const H3_ADDR           : u8      = 0xe3;
const H4_ADDR           : [u8; 2] = [0xe4, 0xe5];
const H5_ADDR           : [u8; 2] = [0xe5, 0xe6];
const H6_ADDR           : u8      = 0xe7;

const MSB : usize = 0;
const LSB : usize = 1;
const XLSB: usize = 2;

pub struct BME280{
    gpio: Gpio
}

impl BME280 {
    pub fn new() -> Self {
        BME280 {
            gpio: Gpio::new()
        }
    }

    pub fn get_temperature(&self){

    }

    fn compensation_temperature(&self, temp_raw: i32) -> i32 {
        let mut msb = self.gpio.i2c_read_byte_from(BME280_ADDR, T1_ADDR[MSB]);
        let mut lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, T1_ADDR[LSB]);
        let dig_t1: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, T2_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, T2_ADDR[LSB]);
        let dig_t2: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, T3_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, T3_ADDR[LSB]);
        let dig_t3: u16 = ((msb as u16) << 8) | lsb as u16;

        let var1  = ((((temp_raw >> 3) - ((dig_t1 as i32) << 1))) * (dig_t2 as i32)) >> 11; 
        let var2  = ((((temp_raw >> 4) - (dig_t1 as i32)) * ((temp_raw >> 4) - ((dig_t1 as i32))) >> 12) * (dig_t3 as i32)) >> 14;
        let t_fine = var1 + var2;
        let temp = (t_fine * 5 + 128) >> 8;

        temp
    }

    pub fn get_pressure(&self){

    }

    fn compensation_pressure(&self){
        let mut msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P1_ADDR[MSB]);
        let mut lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P1_ADDR[LSB]);
        let dig_p1: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P2_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P2_ADDR[LSB]);
        let dig_p2: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P3_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P3_ADDR[LSB]);
        let dig_p3: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P4_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P4_ADDR[LSB]);
        let dig_p4: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P5_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P5_ADDR[LSB]);
        let dig_p5: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P6_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P6_ADDR[LSB]);
        let dig_p6: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P7_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P7_ADDR[LSB]);
        let dig_p7: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P8_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P8_ADDR[LSB]);
        let dig_p8: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P9_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P9_ADDR[LSB]);
        let dig_p9: u16 = ((msb as u16) << 8) | lsb as u16;
    }

    pub fn get_humidity(&self){

    }

    fn compensation_humidity(&self){
        let dig_h1: u8 = self.gpio.i2c_read_byte_from(BME280_ADDR, H1_ADDR);

        let mut msb = self.gpio.i2c_read_byte_from(BME280_ADDR, H2_ADDR[MSB]);
        let mut lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, H2_ADDR[LSB]);
        let dig_h2: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        let dig_h3: u8 = self.gpio.i2c_read_byte_from(BME280_ADDR, H3_ADDR);

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, H4_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, H4_ADDR[LSB]);
        let dig_h4: i16 = (((msb as u16) << 4) | ((lsb & 0x0f) as u16)) as i16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, H5_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, H5_ADDR[LSB]);
        let dig_h5: i16 = (((msb as u16) << 4) | (((lsb >> 4) & 0x0f) as u16)) as i16;

        let dig_h6: i8 = self.gpio.i2c_read_byte_from(BME280_ADDR, H6_ADDR) as i8;
    }


}
