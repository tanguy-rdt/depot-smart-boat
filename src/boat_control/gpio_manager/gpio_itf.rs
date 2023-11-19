pub trait GpioItf {
    fn new() -> Self;
    fn init(&self);
    fn i2c_read_byte_from(&self, device_addr: u8, register: u8) -> u8;
}