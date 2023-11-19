use crate::boat_control::gpio_manager::gpio_itf::GpioItf;

#[cfg(feature = "on_target")]
use rppal::gpio::{Gpio, Level};

const GAUCHE: u8 = 4;
const DROITE: u8 = 14;

#[cfg(not(feature = "on_target"))]
pub struct GpioManager;

#[cfg(feature = "on_target")]
pub struct GpioManager{    
    pin_gauche: rppal::gpio::OutputPin,
    pin_droite: rppal::gpio::OutputPin,
}

#[cfg(feature = "on_target")]
impl GpioItf for GpioManager {
    fn new() -> Self {
        let gpio = Gpio::new().unwrap();

        let mut pin_gauche = gpio.get(GAUCHE).unwrap().into_output();
        let mut pin_droite = gpio.get(DROITE).unwrap().into_output();
    
        pin_gauche.set_low();
        pin_droite.set_low();
    
        GpioManager { pin_gauche, pin_droite }
    }

    fn init(&self){
        println!("Im the init in rpi mod");
    }

    fn i2c_read_byte_from(&self, device_addr: u8, register: u8) -> u8{
        println!("Im the init in rpi mod");

        0
    }


    fn i2c_write_byte(&self, device_addr: u8, register: u8, value: u8){
        println!("Im the init in rpi mod");

    }
}