use crate::boat_control::gpio_manager::gpio_itf::GpioItf;

pub struct GpioManagerStub;

impl GpioItf for GpioManagerStub {
    fn new() -> Self {
        GpioManagerStub
    }

    fn init(&self){
        println!("Im the init in stub mod");
    }

    fn i2c_read_byte_from(&self, device_addr: u8, register: u8) -> u8{
        println!("Im the init in stub mod");

        0
    }
}