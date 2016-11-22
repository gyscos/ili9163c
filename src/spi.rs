// TODO: move it to separate crate

// TODO: handle baudrate configuration?
pub trait Serial {
    fn write_u8(&mut self, data: u8);
    fn write_u16(&mut self, data: u16);
}
