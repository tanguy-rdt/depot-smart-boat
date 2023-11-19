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

    fn get_humidity(&self) {

    }

    fn get_pressure(&self) {

    }

    fn get_temperature(&self) {
        
    }
}
