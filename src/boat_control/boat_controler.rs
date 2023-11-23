use crate::boat_control::bme280::BME280;

pub struct BoatControler{
    bme280: BME280
}

impl BoatControler {
    pub fn new() -> Self {
        BoatControler {
            bme280: BME280::new()
        }
    }

    pub fn init(&mut self){
        self.bme280.init();
    }

    pub fn get_temperature(&mut self) -> f32{
        self.bme280.get_temperature()
    }

    pub fn get_pressure(&self) -> f32{
        self.bme280.get_pressure()
    }

    pub fn get_humidity(&self) -> f32{
        self.bme280.get_humidity()
    }
}
