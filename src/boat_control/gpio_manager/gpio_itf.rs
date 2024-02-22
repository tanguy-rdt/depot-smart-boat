pub trait GpioItf {
    fn new() -> Self;
    fn init(&self);
    fn i2c_set_slave_addr(&mut self, addr: u8);
    fn i2c_read_byte_from(&self, register: u8) -> u8;
    fn i2c_read_bytes_from(&self, register: u8, buffer: &mut [u8]);
    fn i2c_write_byte(&self, register: u8, value: u8);
    fn i2c_write_bytes(&self, register: u8, values: &[u8]);
}