use crate::gpio_manager::gpio_itf::GpioItf;

pub struct GpioManagerStub;

impl GpioItf for GpioManagerStub {
    fn new() -> Self {
        GpioManagerStub
    }

    fn init_gpio(&self){
        println!("Im the init in stub mod");
    }
}