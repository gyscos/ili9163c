#![no_std]

pub mod spi;
pub mod gpio;
pub mod command;

pub struct Driver<S, DCX, CSX>
    where S: spi::Serial,
          DCX: gpio::Pin,
          CSX: gpio::Pin
{
    /// Main data bus
    ///
    /// Connects to SCK (clock) and SDA (MOSI).
    ///
    /// It seems MISO is also connected to SDA, so during read instructions,
    /// we'll read the same wire.
    spi: S,

    /// D/CX: Data/Command Selection pin.
    ///
    /// Connects to CS on the board
    /// Low for command, High for memory write
    dcx: DCX,

    /// Chip Enable pin. (Basically SPI's SS)
    ///
    /// Connect to A0 on the board
    csx: CSX,
}

impl <S, DCX, CSX> Driver<S, DCX, CSX>
    where S: spi::Serial,
          DCX: gpio::Pin,
          CSX: gpio::Pin
{
    pub fn new(spi: S, dcx: DCX, csx: CSX) -> Self {
        Driver {
            spi: spi,
            dcx: dcx,
            csx: csx,
        }
    }

    pub fn write_cmd(&mut self, cmd: command::Command) {
        self.dcx.low();
        self.spi.write_u8(cmd as u8);
    }

    pub fn write_data(&mut self, data: u8) {
        self.dcx.high();
        self.spi.write_u8(data);
    }
}
