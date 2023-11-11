pub trait GpioItf {
    fn new() -> Self;
    fn init_gpio(&self);
}