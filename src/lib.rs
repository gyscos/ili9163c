//! Driver for the ILI9163C LCD board.
//!
//! Often sold as an inexpensive 1.44" 128x128 color TFT on red or black PCB.
//!
//! This library is largely inspired from [sumotoy]'s one.
//!
//! [sumotoy]: https://github.com/sumotoy/TFT_ILI9163C

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod spi;
pub mod gpio;
pub mod command;

use command::{Command, PixelFormat, GammaCurve};

/// Main structure to manage to LCD board.
///
/// Give this `Driver` an activated SPI device, as well as the `D/CX` and the
/// `CSX` pins, ready for output.
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

pub fn sleep(ms: u16) {}

impl<S, DCX, CSX> Driver<S, DCX, CSX>
    where S: spi::Serial,
          DCX: gpio::Pin,
          CSX: gpio::Pin
{
    pub fn new(spi: S, dcx: DCX, csx: CSX) -> Self {


        // TODO: use a RESET pin?
        //
        // SoftwareReset

        let mut driver = Driver {
            spi: spi,
            dcx: dcx,
            csx: csx,
        };

        driver.csx.low();

        driver.write_cmd(Command::SoftwareReset);
        sleep(500);

        driver.write_cmd(Command::SleepOut);
        sleep(5);

        driver.set_pixel_format(PixelFormat::Bpp16);
        driver.set_gamma_curve(GammaCurve::Curve3);
        driver.set_gamma_adjustment(true);

        driver.write_cmd(Command::NormalModeOn);

        driver.write_cmd(Command::DisplayFunctionSet5);
        driver.write_data(0b11111111);
        driver.write_data(0b00000000);

        // TODO: set gamma correction?

        // TODO: set frame rate control?

        // TODO: reset column/page address window?
        // TODO: reset scroll area?
        // TODO: clear screen?

        driver
    }

    // Low-level functions

    pub fn write_cmd(&mut self, cmd: Command) {
        self.dcx.low();

        self.csx.low();
        self.spi.write(cmd as u8);
        self.csx.high();
    }

    pub fn write_data(&mut self, data: u8) {
        self.dcx.high();

        self.csx.low();
        self.spi.write(data);
        self.csx.high();
    }

    pub fn write_data_16(&mut self, data: u16) {
        self.dcx.high();

        self.csx.low();
        self.spi.write((data >> 8) as u8);
        self.spi.write(data as u8);
        self.csx.high();
    }

    // Mid-level functions

    pub fn set_pixel_format(&mut self, format: PixelFormat) {
        self.write_cmd(Command::InterfacePixelFormat);
        self.write_data(format as u8);
    }

    pub fn set_gamma_curve(&mut self, curve: GammaCurve) {
        self.write_cmd(Command::GammaSet);
        self.write_data(curve as u8);
    }

    pub fn set_gamma_adjustment(&mut self, enable: bool) {
        self.write_cmd(Command::GamRSel);
        self.write_data(enable as u8);
    }

    // Higher-level functions now

    pub fn set_window(&mut self, (startx, starty): (u16, u16), (endx, endy): (u16, u16)) {
        self.write_cmd(Command::ColumnAddressSet);
        self.write_data_16(startx);
        self.write_data_16(endx);

        self.write_cmd(Command::PageAddressSet);
        self.write_data_16(starty);
        self.write_data_16(endy);
    }

    // TODO: handle 18-bits colors?
    pub fn clear_screen(&mut self, color: u16) {
        // TODO: hard-code 128x128? Or use some variable?

        self.set_window((0, 0), (128, 128));

        self.write_cmd(Command::MemoryWrite);
        for _ in 0..(128*128) {
            self.write_data_16(color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let csx = gpio::DebugPin::new("CSX");
        let dcx = gpio::DebugPin::new("D/CX");
        let spi = spi::DebugSerial;

        let mut driver = Driver::new(spi, dcx, csx);

        driver.clear_screen(0x00);
    }

}
