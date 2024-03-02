pub trait GpioItf {
    fn new() -> Self;
    fn init(&self);
    fn i2c_set_slave_addr(&mut self, addr: u8);
    fn i2c_read_byte_from(&self, register: u8) -> u8;
    fn i2c_read_bytes_from(&self, register: u8, buffer: &mut [u8]);
    fn i2c_write_byte(&self, register: u8, value: u8);
    fn i2c_write_bytes(&self, register: u8, values: &[u8]);
    fn spi_transfer(&mut self, read_buffer: &mut[u8], write_buffer: &[u8]);
    fn set_output(&mut self, pin: u8);
    fn set_input(&mut self, pin: u8);
    fn set_high(&mut self, pin: u8);
    fn set_low(&mut self, pin: u8);
    fn is_high(&self, pin: u8) -> bool;
    fn is_low(&self, pin: u8) -> bool;
}