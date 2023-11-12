pub trait GpioItf {
    fn new() -> Self;
    fn init(&self);
}