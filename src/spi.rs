use gpio;

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

pub struct BitBangingSerial<SCK, MOSI>
    where SCK: gpio::Pin,
          MOSI: gpio::Pin
{
    sck: SCK,
    mosi: MOSI,
}

impl<SCK, MOSI> BitBangingSerial<SCK, MOSI>
    where SCK: gpio::Pin,
          MOSI: gpio::Pin
{
    pub fn new(sck: SCK, mosi: MOSI) -> Self {
        BitBangingSerial {
            sck: sck,
            mosi: mosi,
        }
    }

    fn write_bit(&mut self, bit: bool) {
        self.sck.low();
        if bit {
            self.mosi.high();
        } else {
            self.mosi.low();
        }
        self.sck.high();
    }
}

impl<SCK, MOSI> Serial for BitBangingSerial<SCK, MOSI>
    where SCK: gpio::Pin,
          MOSI: gpio::Pin
{
    fn write(&mut self, data: u8) {
        self.write_bit((data & 0b10000000) != 0);
        self.write_bit((data & 0b01000000) != 0);
        self.write_bit((data & 0b00100000) != 0);
        self.write_bit((data & 0b00010000) != 0);
        self.write_bit((data & 0b00001000) != 0);
        self.write_bit((data & 0b00000100) != 0);
        self.write_bit((data & 0b00000010) != 0);
        self.write_bit((data & 0b00000001) != 0);
    }
}
