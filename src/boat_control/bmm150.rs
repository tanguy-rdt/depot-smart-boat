use crate::boat_control::gpio_manager::gpio_itf::GpioItf;
use crate::boat_control::gpio_manager::Gpio;

use std::thread;
use std::time::Duration;

const BMM150_ADDR             : u8 = 0x13;
const CTRL_POWER_REGISTER     : u8 = 0x4b;
const CHIP_ID_VALUE           : u8 = 0x32;
const CHIP_ID_REGISTER        : u8 = 0x40;
const DIG_X1                  : u8 = 0x5D;
const DIG_Z4_LSB              : u8 = 0x62;
const DIG_Z2_LSB              : u8 = 0x68;
const ENABLE_POWER            : i8 = 1;
const DISABLE_POWER           : i8 = 0;
const POWERMODE_NORMAL        : u8 = 0x00;
const POWERMODE_FORCED        : u8 = 0x01;
const POWERMODE_SLEEP         : u8 = 0x03;
const MODE_RATE_REGISTER      : u8 = 0x4c;
const PRESETMODE_LOWPOWER     : u8 = 0x01;
const PRESETMODE_REGULAR      : u8 = 0x02;
const PRESETMODE_HIGHACCURACY : u8 = 0x03;
const PRESETMODE_ENHANCED     : u8 = 0x04;
const REPXY_LOWPOWER          : u8 = 0x01;
const REPXY_REGULAR           : u8 = 0x04;
const REPXY_ENHANCED          : u8 = 0x07;
const REPXY_HIGHACCURACY      : u8 = 0x17;
const REPZ_LOWPOWER           : u8 = 0x01;
const REPZ_REGULAR            : u8 = 0x07;
const REPZ_ENHANCED           : u8 = 0x0D;
const REPZ_HIGHACCURACY       : u8 = 0x29;
const REG_DATA_X_LSB          : u8 = 0x42;
const REG_AXES_ENABLE         : u8 = 0x4E;
const REG_REP_XY              : u8 = 0x51;
const REG_REP_Z               : u8 = 0x52;
const RATE_10HZ               : u8 = 0x00;
const RATE_02HZ               : u8 = 0x01;
const RATE_06HZ               : u8 = 0x02;
const RATE_08HZ               : u8 = 0x03;
const RATE_15HZ               : u8 = 0x04;
const RATE_20HZ               : u8 = 0x05;
const RATE_25HZ               : u8 = 0x06;
const RATE_30HZ               : u8 = 0x07;



struct TrimData {
    dig_x1: i8,
    dig_y1: i8,
    dig_x2: i8,
    dig_y2: i8,
    dig_z1: u16,
    dig_z2: i16,
    dig_z3: i16,
    dig_z4: i16,
    dig_xy1: u8,
    dig_xy2: i8,
    dig_xyz1: u16,
}

pub struct BMM150{
    t_fine: i32,
    bme280: u8,
    trim_data: TrimData,
    geomagnetic_x: i32,
    geomagnetic_y: i32,
    geomagnetic_z: i32,
    geomagnetic_r: i32,
}

impl BMM150 {
    pub fn new() -> Self {
        Self {
            t_fine: 0,
            bme280: 0,
            trim_data: TrimData::Default(),
            geomagnetic_x: 0,
            geomagnetic_y: 0,
            geomagnetic_z: 0,
            geomagnetic_r: 0,
        }
    }
    
    pub fn init(&mut self, gpio: &mut Gpio) -> i8{
        self.set_power_bit(ENABLE_POWER);
        thread::sleep(Duration::from_millis(3)); 
        let chip_id = self.get_chip_id();
        if chip_id == CHIP_ID_VALUE {
            self.get_trim_value();
            return 0;
        }
        else {
            return -1;
        }
    }

    fn set_power_bit(&self, state: i8) {
        gpio.i2c_set_slave_addr(BMM150_ADDR);

        let rslt = gpio.i2c_read_byte_from(CTRL_POWER_REGISTER);
        if state == DISABLE_POWER {
            gpio.i2c_write_byte(CTRL_POWER_REGISTER, rslt & 0xFE);
        }
        else {
            gpio.i2c_write_byte(CTRL_POWER_REGISTER, rslt | 0x01);
        }
    }

    fn get_chip_id(&self) -> u8 {
        gpio.i2c_set_slave_addr(BMM150_ADDR);
        gpio.i2c_read_byte_from(CHIP_ID_REGISTER)
    }

    fn get_trim_value(&mut self) {
        gpio.i2c_set_slave_addr(BMM150_ADDR);

        let mut trim_x1_y1 = [0u8; 2];
        gpio.i2c_read_bytes_from(DIG_X1, &mut trim_x1_y1);

        let mut trim_xyz_data = [0u8; 4];
        gpio.i2c_read_bytes_from(DIG_Z4_LSB, &mut trim_xyz_data);

        let mut trim_xy1_xy2 = [0u8; 10];
        gpio.i2c_read_bytes_from(DIG_Z2_LSB, &mut trim_xy1_xy2);

        self.trim_data.dig_x1 = trim_x1_y1[0] as i8;
        self.trim_data.dig_y1 = trim_x1_y1[1] as i8;
        self.trim_data.dig_x2 = trim_xyz_data[2] as i8;
        self.trim_data.dig_y2 = trim_xyz_data[3] as i8;
        let temp_msb = (trim_xy1_xy2[3] << 8) as i16;
        self.trim_data.dig_z1 = (temp_msb | trim_xy1_xy2[2]) as u16;
        let temp_msb = (trim_xy1_xy2[1] << 8) as i16;
        self.trim_data.dig_z2 = (temp_msb | trim_xy1_xy2[0]) as i16;
        let temp_msb = (trim_xy1_xy2[7] << 8) as i16;
        self.trim_data.dig_z3 = (temp_msb | trim_xy1_xy2[6]) as i16;
        let temp_msb = (trim_xyz_data[1] << 8) as i16;
        self.trim_data.dig_z4 = (temp_msb | trim_xyz_data[0]) as i16;
        self.trim_data.dig_xy1 = trim_xy1_xy2[9];
        self.trim_data.dig_xy2 = trim_xy1_xy2[8] as i8;
        let temp_msb = ((trim_xy1_xy2[5] & 0x7F) << 8) as i16;
        self.trim_data.dig_xyz1 = (temp_msb | trim_xy1_xy2[4]) as u16;
    }

    fn set_operation_mode(&self, mode: u8){
        gpio.i2c_set_slave_addr(BMM150_ADDR);

        let rslt = gpio.i2c_read_byte_from(MODE_RATE_REGISTER);
        match mode {
            POWERMODE_NORMAL => {
                self.set_power_bit(ENABLE_POWER);
                gpio.i2c_write_byte(MODE_RATE_REGISTER, rslt & 0xf9);
            },
            POWERMODE_FORCED => {
                self.set_power_bit(ENABLE_POWER);
                gpio.i2c_write_byte(MODE_RATE_REGISTER, (rslt & 0xf9) | 0x02);
            },
            POWERMODE_SLEEP => {
                self.set_power_bit(ENABLE_POWER);
                gpio.i2c_write_byte(MODE_RATE_REGISTER, (rslt & 0xf9) | 0x04);

            },
            _ => self.set_power_bit(DISABLE_POWER),
        }
    }

    fn set_preset_mode(&self, mode: u8){
        gpio.i2c_set_slave_addr(BMM150_ADDR);

        match mode {
            PRESETMODE_LOWPOWER => {
                self.set_xy_rep(REPXY_LOWPOWER);
                self.set_z_rep(REPZ_LOWPOWER);
            },
            PRESETMODE_REGULAR => {
                self.set_xy_rep(REPXY_REGULAR);
                self.set_z_rep(REPZ_REGULAR);
            },
            PRESETMODE_HIGHACCURACY => {
                self.set_xy_rep(REPXY_HIGHACCURACY);
                self.set_z_rep(REPZ_HIGHACCURACY);
            },
            PRESETMODE_ENHANCED => {
                self.set_xy_rep(REPXY_ENHANCED);
                self.set_z_rep(REPZ_ENHANCED);
            },
            _ => {
                self.set_xy_rep(REPXY_LOWPOWER);
                self.set_z_rep(REPZ_LOWPOWER);
            },
        }
    }

    fn set_xy_rep(&self, mode: u8){
        gpio.i2c_set_slave_addr(BMM150_ADDR);

        match mode {
            REPXY_LOWPOWER     => gpio.i2c_write_byte(REG_REP_XY, mode),
            REPXY_REGULAR      => gpio.i2c_write_byte(REG_REP_XY, mode),
            REPXY_ENHANCED     => gpio.i2c_write_byte(REG_REP_XY, mode),
            REPXY_HIGHACCURACY => gpio.i2c_write_byte(REG_REP_XY, mode),
            _                  => gpio.i2c_write_byte(REG_REP_XY, REPXY_LOWPOWER),
        }
    }

    fn set_z_rep(&self, mode: u8){
        gpio.i2c_set_slave_addr(BMM150_ADDR);

        match mode {
            REPZ_LOWPOWER     => gpio.i2c_write_byte(REG_REP_Z, mode),
            REPZ_REGULAR      => gpio.i2c_write_byte(REG_REP_Z, mode),
            REPZ_ENHANCED     => gpio.i2c_write_byte(REG_REP_Z, mode),
            REPZ_HIGHACCURACY => gpio.i2c_write_byte(REG_REP_Z, mode),
            _                 => gpio.i2c_write_byte(REG_REP_Z, REPZ_LOWPOWER),
        }
    }

    fn set_rate(&self, rate: u8){
        gpio.i2c_set_slave_addr(BMM150_ADDR);
        
        let rslt = gpio.i2c_read_byte_from(MODE_RATE_REGISTER);
        match rate {
            RATE_02HZ => gpio.i2c_write_byte(MODE_RATE_REGISTER, (rslt&0xc7) | 0x08),
            RATE_06HZ => gpio.i2c_write_byte(MODE_RATE_REGISTER, (rslt&0xc7) | 0x10),
            RATE_08HZ => gpio.i2c_write_byte(MODE_RATE_REGISTER, (rslt&0xc7) | 0x18),
            RATE_10HZ => gpio.i2c_write_byte(MODE_RATE_REGISTER, rslt&0xc7),
            RATE_15HZ => gpio.i2c_write_byte(MODE_RATE_REGISTER, (rslt&0xc7) | 0x20),
            RATE_20HZ => gpio.i2c_write_byte(MODE_RATE_REGISTER, (rslt&0xc7) | 0x28),
            RATE_25HZ => gpio.i2c_write_byte(MODE_RATE_REGISTER, (rslt&0xc7) | 0x30),
            RATE_30HZ => gpio.i2c_write_byte(MODE_RATE_REGISTER, (rslt&0xc7) | 0x38),
            _         => gpio.i2c_write_byte(MODE_RATE_REGISTER, rslt&0xc7),
        }
    }

    fn set_measurement_xyz(&self, x: bool, y: bool, z: bool) {
        gpio.i2c_set_slave_addr(BMM150_ADDR);
        let rslt = gpio.i2c_read_byte_from(REG_AXES_ENABLE);
        
        let mut buf: u8 = 0x00;
        
        if x { buf = rslt & 0xF7; }
        else { buf = rslt | 0x08; }

        if y { buf = buf & 0xEF; }
        else { buf = buf | 0x10; }

        if z { buf = buf & 0xDF; }
        else { buf = buf | 0x20; }

        gpio.i2c_write_byte(REG_AXES_ENABLE, buf);
    }

    fn get_geomagnetic(&self) -> (i32, i32, i32){
        gpio.i2c_set_slave_addr(BMM150_ADDR);

        let mut rslt = [0u8; 8];
        gpio.i2c_read_bytes_from(REG_DATA_X_LSB, &mut rslt);

        rslt[1] = rslt[1] as i8;
        rslt[3] = rslt[3] as i8;
        rslt[5] = rslt[5] as i8;

        let geomagnetic_raw_x = ((rslt[0]&0xF8) >> 3)  | (rslt[1]*32) as i32;
        let geomagnetic_raw_y = ((rslt[2]&0xF8) >> 3)  | (rslt[3]*32) as i32;
        let geomagnetic_raw_z = ((rslt[4]&0xFE) >> 1)  | (rslt[5]*128) as i32;
        let geomagnetic_raw_r = ((rslt[6]&0xFC) >> 2)  | (rslt[7]*64) as i32;

        self.geomagnetic_x = self.compensate_x(geomagnetic_raw_x, geomagnetic_raw_r);
        self.geomagnetic_y = self.compensate_y(geomagnetic_raw_y, geomagnetic_raw_r);
        self.geomagnetic_z = self.compensate_z(geomagnetic_raw_z, geomagnetic_raw_r);

        (self.geomagnetic_x, self.geomagnetic_y, self.geomagnetic_z)
    }

    fn compensate_x(self, x: i32, r: i32) -> i32{
        if x != -4096{
            let mut process_comp_x0: i32 = if r != 0 { r }
            else if self.trim_data.dig_xyz1 != 0 { self.trim_data.dig_xyz1 }
            else { 0 };

            if process_comp_x0 != 0 {
                let process_comp_x1 = (self.trim_data.dig_xyz1*16384/process_comp_x0) as i32;
                let process_comp_x2 = (process_comp_x1 - 0x4000) as i32;
                let process_comp_x3 = process_comp_x2 * process_comp_x2;
                let process_comp_x4 = self.trim_data.dig_xy2 * (process_comp_x3/128);
                let process_comp_x5 = self.trim_data.dig_xy1 * 128;
                let process_comp_x6 = process_comp_x2 * process_comp_x5;
                let process_comp_x7 = (process_comp_x4 + process_comp_x6)/512 + 0x100000;
                let process_comp_x8 = self.trim_data.dig_x2 + 0xA0;
                let process_comp_x9 = (process_comp_x8 * process_comp_x7)/4096;
                let process_comp_x10 = x * process_comp_x9;
                ((process_comp_x10/8192) + self.trim_data.dig_x1*8)/16
            }
            else {
                -32768
            }
        }
        else {
            -32768
        }
    }

    fn compensate_y(self, y: i32, r: i32) -> i32{
        if y != -4096{
            let mut process_comp_y0: i32 = if r != 0 { r }
            else if self.trim_data.dig_xyz1 != 0 { self.trim_data.dig_xyz1 }
            else { 0 };

            if process_comp_y0 != 0 {
                let process_comp_y1 = (self.trim_data.dig_xyz1*16384/process_comp_y0) as i32;
                let process_comp_y2 = int(process_comp_y1 - 0x4000) as i32;
                let process_comp_y3 = process_comp_y2 * process_comp_y2;
                let process_comp_y4 = self.trim_data.dig_xy2 * (process_comp_y3/128);
                let process_comp_y5 = self.trim_data.dig_xy1 * 128;
                let process_comp_y6 = (process_comp_y4+process_comp_y5*process_comp_y2)/512;
                let process_comp_y7 = self.trim_data.dig_y2 + 0xA0;
                let process_comp_y8 = ((process_comp_y6 + 0x100000)*process_comp_y7)/4096;
                let process_comp_y9 = y*process_comp_y8;
                ((process_comp_y9/8192) + self.trim_data.dig_y1*8)/16
            }
            else {
                -32768
            }
        }
        else {
            -32768
        }
    }

    fn compensate_z(&self, z: i32, r: i32) -> i32{
        if z != -16384{
            if self.trim_data.dig_z2 != 0 && self.trim_data.dig_z1 != 0 && self.trim_data.dig_xyz1 != 0 && r != 0 {
                let process_comp_z0 = r - self.trim_data.dig_xyz1;
                let process_comp_z1 = (self.trim_data.dig_z3*process_comp_z0)/4;
                let process_comp_z2 = (z - self.trim_data.dig_z4)*32768;
                let process_comp_z3 = self.trim_data.dig_z1 * r*2;
                let process_comp_z4 = (process_comp_z3+32768)/65536;
                let mut res = (process_comp_z2 - process_comp_z1)/(self.trim_data.dig_z2+process_comp_z4);
                res = res.clamp(-32767, 32767);
                res/16
            }
            else {
                -32768
            }
        }
        else {
            -32768
        }
    }

    fn get_compass_degree(&self) -> f32 {
        let (x, y, z) = self.get_geomagnetic();
        let mut compass = (x as f32).atan2(y as f32);
        if compass < 0 { compass += 2.0 * std::consts::f32::PI; }
        if compass > (2.0 * std::consts::f32::PI) { compass -= 2.0 * std::consts::f32::PI; }
        compass * 180.0 / std::consts::f32::PI
    }
}
