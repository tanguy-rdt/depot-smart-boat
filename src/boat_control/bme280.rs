use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;


const BME280_ADDR       : u8      = 0x76;
const BME280_CONFIG_ADDR: u8      = 0xf5;
const CTRL_MEAS_ADDR    : u8      = 0xf4;

const TEMP_ADDR         : [u8; 3] = [0xfa, 0xfb, 0xfc];
const PRESS_ADDR        : [u8; 3] = [0xf7, 0xf8, 0xf9];
const HUM_ADDR          : [u8; 2] = [0xfd, 0xfe];
const HUM_CTRL_ADDR     : u8     = 0xf2;

const T1_ADDR           : [u8; 2] = [0x89, 0x88];
const T2_ADDR           : [u8; 2] = [0x8b, 0x8a];
const T3_ADDR           : [u8; 2] = [0x8d, 0x8c];

const P1_ADDR           : [u8; 2] = [0x8f, 0x8e];
const P2_ADDR           : [u8; 2] = [0x91, 0x90];
const P3_ADDR           : [u8; 2] = [0x93, 0x92];
const P4_ADDR           : [u8; 2] = [0x95, 0x94];
const P5_ADDR           : [u8; 2] = [0x97, 0x96];
const P6_ADDR           : [u8; 2] = [0x99, 0x98];
const P7_ADDR           : [u8; 2] = [0x9b, 0x9a];
const P8_ADDR           : [u8; 2] = [0x9d, 0x9c];
const P9_ADDR           : [u8; 2] = [0x9f, 0x9e];

const H1_ADDR           : u8      = 0xa1;
const H2_ADDR           : [u8; 2] = [0xe2, 0xe1];
const H3_ADDR           : u8      = 0xe3;
const H4_ADDR           : [u8; 2] = [0xe4, 0xe5];
const H5_ADDR           : [u8; 2] = [0xe6, 0xe5];
const H6_ADDR           : u8      = 0xe7;

const MSB : usize = 0;
const LSB : usize = 1;
const XLSB: usize = 2;

pub struct BME280{
    gpio: Gpio,
    t_fine: i32,
    bme280: u8
}

impl BME280 {
    pub fn new() -> Self {
        BME280 {
            gpio: Gpio::new(),
            t_fine: 0,
            bme280: 0
        }
    }
    
    pub fn init(&mut self){
        self.gpio.i2c_set_slave_addr(BME280_ADDR);
        self.gpio.i2c_write_byte(HUM_CTRL_ADDR, 0x01);
        self.gpio.i2c_write_byte(CTRL_MEAS_ADDR, 0x27);
        self.gpio.i2c_write_byte(BME280_CONFIG_ADDR, 0x00);
    }

    pub fn get_temperature(&mut self) -> f32{
        let msb: u8 = self.gpio.i2c_read_byte_from(TEMP_ADDR[MSB]);
        let lsb: u8 = self.gpio.i2c_read_byte_from(TEMP_ADDR[LSB]);
        let xlsb: u8 = self.gpio.i2c_read_byte_from(TEMP_ADDR[XLSB]);
        let temp_raw: i32 = ((msb as i32) << 12) | ((lsb as i32) << 4) | xlsb as i32;

        let temperature_in_C: f32 = (self.compensation_temperature(temp_raw)) as f32 / 100.0;

        temperature_in_C
    }

    fn compensation_temperature(&mut self, temp_raw: i32) -> i32 {
        let mut msb = self.gpio.i2c_read_byte_from(T1_ADDR[MSB]);
        let mut lsb = self.gpio.i2c_read_byte_from(T1_ADDR[LSB]);
        let dig_t1: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(T2_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(T2_ADDR[LSB]);
        let dig_t2: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(T3_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(T3_ADDR[LSB]);
        let dig_t3: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        let var1  = ((((temp_raw >> 3) - ((dig_t1 as i32) << 1))) * (dig_t2 as i32)) >> 11; 
        let var2  = ((((temp_raw >> 4) - (dig_t1 as i32)) * ((temp_raw >> 4) - ((dig_t1 as i32))) >> 12) * (dig_t3 as i32)) >> 14;
        self.t_fine = var1 + var2;
        let temp = ((self.t_fine * 5 + 128) >> 8);

        temp
    }

    pub fn get_pressure(&self) -> f32{
        let msb: u8 = self.gpio.i2c_read_byte_from(PRESS_ADDR[MSB]);
        let lsb: u8 = self.gpio.i2c_read_byte_from(PRESS_ADDR[LSB]);
        let xlsb: u8 = self.gpio.i2c_read_byte_from(PRESS_ADDR[XLSB]);
        let press_raw: i32 = ((msb as i32) << 12) | ((lsb as i32) << 4) | xlsb as i32;

        let pressure_in_pa = (self.compensation_pressure(press_raw) as f32)/256.0;

        pressure_in_pa
    }

    fn compensation_pressure(&self, press_raw: i32) -> u32{
        let mut msb = self.gpio.i2c_read_byte_from(P1_ADDR[MSB]);
        let mut lsb = self.gpio.i2c_read_byte_from(P1_ADDR[LSB]);
        let dig_p1: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(P2_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(P2_ADDR[LSB]);
        let dig_p2: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(P3_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(P3_ADDR[LSB]);
        let dig_p3: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(P4_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(P4_ADDR[LSB]);
        let dig_p4: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(P5_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(P5_ADDR[LSB]);
        let dig_p5: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(P6_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(P6_ADDR[LSB]);
        let dig_p6: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(P7_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(P7_ADDR[LSB]);
        let dig_p7: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(P8_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(P8_ADDR[LSB]);
        let dig_p8: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(P9_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(P9_ADDR[LSB]);
        let dig_p9: i16 = (((msb as u16) << 8) | lsb as u16) as i16;



        let mut var1: i64 = (self.t_fine as i64) - 128000;
        let mut var2: i64 = var1 * var1 * (dig_p6 as i64);
        var2 = var2 + ((var1 * (dig_p5 as i64)) << 17);
        var2 = var2 + ((dig_p4 as i64) << 35);
        var1 = ((var1 * var1 * (dig_p3 as i64)) >> 8) + ((var1 * (dig_p2 as i64)) << 12);
        var1 = ((((1 as i64) << 47) + var1)) * (dig_p1 as i64) >> 33;

        if var1 == 0 {
            return 0;
        }

        let mut p: i64 = 1048576 - (press_raw as i64);
        p = (((p << 31) - var2)*3125)/var1;
        var1 = ((dig_p9 as i64) * (p>>13) * (p>>13)) >> 25;
        var2 = ((dig_p8 as i64) * p) >> 19;
        p = ((p + var1 + var2) >> 8) + ((dig_p7 as i64)<<4);

        let pressure: u32 = (p as u32)/100;

        pressure
    }

    pub fn get_humidity(&self) -> f32{
        let msb: u8 = self.gpio.i2c_read_byte_from(HUM_ADDR[MSB]);
        let lsb: u8 = self.gpio.i2c_read_byte_from(HUM_ADDR[LSB]);
        let hum_raw: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        let hum_in_percent = (self.compensation_humidity(hum_raw as i32) as f32)/1024.0;

        hum_in_percent
    }

    fn compensation_humidity(&self, hum_raw: i32) -> i32{
        let dig_h1: u8 = self.gpio.i2c_read_byte_from(H1_ADDR);

        let mut msb = self.gpio.i2c_read_byte_from(H2_ADDR[MSB]);
        let mut lsb = self.gpio.i2c_read_byte_from(H2_ADDR[LSB]);
        let dig_h2: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        let dig_h3: u8 = self.gpio.i2c_read_byte_from(H3_ADDR);

        msb = self.gpio.i2c_read_byte_from(H4_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(H4_ADDR[LSB]);
        let dig_h4: i16 = (((msb as u16) << 4) | ((lsb & 0x0f) as u16)) as i16;

        msb = self.gpio.i2c_read_byte_from(H5_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(H5_ADDR[LSB]);
        let dig_h5: i16 = (((msb as u16) << 4) | (((lsb >> 4) & 0x0f) as u16)) as i16;

        let dig_h6: i8 = self.gpio.i2c_read_byte_from(H6_ADDR) as i8;

        let mut v_x1: i32 = self.t_fine - (76800 as i32);
        v_x1 = ((((hum_raw << 14) -((dig_h4 as i32) << 20) - ((dig_h5 as i32) * v_x1)) + (16384 as i32)) >> 15) * (((((((v_x1 * (dig_h6 as i32)) >> 10) *
               (((v_x1 * (dig_h3 as i32)) >> 11) + (32768 as i32))) >> 10) + (2097152 as i32)) * (dig_h2 as i32) + 8192) >> 14);
        v_x1 = v_x1 - (((((v_x1 >> 15) * (v_x1 >> 15)) >> 7) * (dig_h1 as i32)) >> 4);
        v_x1 = v_x1.clamp(0, 419430400);
        let hum: i32 =  (((v_x1 >> 12)) as i32);

        hum
    }
}
