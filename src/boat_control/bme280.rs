use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;


const BME280_ADDR       : u8      = 0x00;
const BME280_CONFIG_ADDR: u8      = 0xf5;
const CTRL_MEAS_ADDR    : u8      = 0xf4;

const TEMP_ADDR         : [u8; 3] = [0xfa, 0xfb, 0xfc]; // MSB, LSB, XLSB
const PRESS_ADDR        : [u8; 3] = [0xf7, 0xf8, 0xf9];
const HUM_ADDR          : [u8; 2] = [0xfd, 0xfe]; // MSB, LSB
const HUM_CTRL_ADDR     : u8     = 0xf2;

const T1_ADDR           : [u8; 2] = [0x89, 0x88]; // MSB, LSB
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
    
    pub fn init(&self){
        self.gpio.i2c_write_byte(self.bme280, HUM_CTRL_ADDR, 0x01);
        self.gpio.i2c_write_byte(self.bme280, CTRL_MEAS_ADDR, 0x27);
        self.gpio.i2c_write_byte(self.bme280, BME280_CONFIG_ADDR, 0x00);
    }

    pub fn get_temperature(&mut self) -> i32{
        let msb: u8 = self.gpio.i2c_read_byte_from(self.bme280, TEMP_ADDR[MSB]);
        let lsb: u8 = self.gpio.i2c_read_byte_from(self.bme280, TEMP_ADDR[LSB]);
        let xlsb: u8 = self.gpio.i2c_read_byte_from(self.bme280, TEMP_ADDR[XLSB]);
        let temp_raw: i32 = ((msb as i32) << 12) | ((lsb as i32) << 4) | xlsb as i32;

        let temperature = self.compensation_temperature(temp_raw);

        temperature
    }

    fn compensation_temperature(&mut self, temp_raw: i32) -> i32 {
        let mut msb = self.gpio.i2c_read_byte_from(BME280_ADDR, T1_ADDR[MSB]);
        let mut lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, T1_ADDR[LSB]);
        let dig_t1: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, T2_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, T2_ADDR[LSB]);
        let dig_t2: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, T3_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, T3_ADDR[LSB]);
        let dig_t3: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        let var1  = ((((temp_raw >> 3) - ((dig_t1 as i32) << 1))) * (dig_t2 as i32)) >> 11; 
        let var2  = ((((temp_raw >> 4) - (dig_t1 as i32)) * ((temp_raw >> 4) - ((dig_t1 as i32))) >> 12) * (dig_t3 as i32)) >> 14;
        self.t_fine = var1 + var2;
        let temp = (self.t_fine * 5 + 128) >> 8;

        temp
    }

    pub fn get_pressure(&self) -> u32{
        let msb: u8 = self.gpio.i2c_read_byte_from(self.bme280, PRESS_ADDR[MSB]);
        let lsb: u8 = self.gpio.i2c_read_byte_from(self.bme280, PRESS_ADDR[LSB]);
        let xlsb: u8 = self.gpio.i2c_read_byte_from(self.bme280, PRESS_ADDR[XLSB]);
        let press_raw: i32 = ((msb as i32) << 12) | ((lsb as i32) << 4) | xlsb as i32;

        let pressure = self.compensation_pressure(press_raw);

        pressure
    }

    fn compensation_pressure(&self, press_raw: i32) -> u32{
        let mut msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P1_ADDR[MSB]);
        let mut lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P1_ADDR[LSB]);
        let dig_p1: u16 = ((msb as u16) << 8) | lsb as u16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P2_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P2_ADDR[LSB]);
        let dig_p2: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P3_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P3_ADDR[LSB]);
        let dig_p3: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P4_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P4_ADDR[LSB]);
        let dig_p4: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P5_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P5_ADDR[LSB]);
        let dig_p5: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P6_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P6_ADDR[LSB]);
        let dig_p6: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P7_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P7_ADDR[LSB]);
        let dig_p7: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P8_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P8_ADDR[LSB]);
        let dig_p8: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        msb = self.gpio.i2c_read_byte_from(BME280_ADDR, P9_ADDR[MSB]);
        lsb = self.gpio.i2c_read_byte_from(BME280_ADDR, P9_ADDR[LSB]);
        let dig_p9: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        let mut var1 = ((self.t_fine as i32)>>1) - (64000 as i32);
        let mut var2 = (((var1>>2) * (var1>>2)) >> 11) * (dig_p6 as i32);
        var2 = var2 + ((var1*(dig_p5 as i32))<<1);
        var2 = (var2>>2)+((dig_p4 as i32)<<16);
        var1 = ((((dig_p3 as i32) * (((var1>>2)*(var1>>2)) >> 13)) >>3) + (((dig_p2 as i32) * var1)>>1))>>18;
        var1 = (((32768+var1))*(dig_p1 as i32))>>15;

        if  var1 == 0 {
            return 0;
        }
        
        let mut p: u32 = (((((1048576 as i32) - press_raw) as u32)-((var2 as u32)>>12)))*3125;

        if p<0x80000000 {
           p = (p << 1) / (var1 as u32);
        }else{
           p = (p/(var1 as u32)) * 2;
        }
        var1 = ((dig_p9 as i32) * (((((p>>3) * (p>>3))>>13) as i32)))>>12;
        var2 = (((p>>2) as i32) * (dig_p8 as i32))>>13;
        p = ((p as i32) + ((var1 + var2 + (dig_p7 as i32)) >> 4)) as u32;
        
        let pressure = p/100;

        pressure
    }

    pub fn get_humidity(&self) -> i32{
        let msb: u8 = self.gpio.i2c_read_byte_from(self.bme280, HUM_ADDR[MSB]);
        let lsb: u8 = self.gpio.i2c_read_byte_from(self.bme280, HUM_ADDR[LSB]);
        let hum_raw: i16 = (((msb as u16) << 8) | lsb as u16) as i16;

        let hum = self.compensation_humidity(hum_raw as i32);

        hum
    }

    fn compensation_humidity(&self, hum_raw: i32) -> i32{
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

        let mut v_x1: i32 = self.t_fine - (76800 as i32);
        v_x1 = ((((hum_raw << 14) -((dig_h4 as i32) << 20) - ((dig_h5 as i32) * v_x1)) + (16384 as i32)) >> 15) * (((((((v_x1 * (dig_h6 as i32)) >> 10) *
               (((v_x1 * (dig_h3 as i32)) >> 11) + (32768 as i32))) >> 10) + (2097152 as i32)) * (dig_h2 as i32) + 8192) >> 14);
        v_x1 = v_x1 - (((((v_x1 >> 15) * (v_x1 >> 15)) >> 7) * (dig_h1 as i32)) >> 4);
        v_x1 = v_x1.clamp(0, 419430400);
        let hum: i32 =  ((v_x1 >> 12)/1000) as i32;

        hum
    }
}
