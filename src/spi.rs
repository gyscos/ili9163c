// TODO: move it to separate crate

// TODO: handle baudrate configuration?
pub trait Serial {
    fn write(&mut self, data: u8);
}

pub struct DummySerial;

impl Serial for DummySerial {
    fn write(&mut self, data: u8) {}
}


#[cfg(test)]
pub struct DebugSerial;

#[cfg(test)]
impl Serial for DebugSerial {
    fn write(&mut self, data: u8) {
        println!("SPI: {:08b}", data);
    }
}
