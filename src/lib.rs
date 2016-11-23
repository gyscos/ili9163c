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

extern crate bresenham;

pub mod spi;
pub mod gpio;
pub mod command;
pub mod driver;

pub fn sleep(ms: u16) {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let csx = gpio::DebugPin::new("CSX");
        let dcx = gpio::DebugPin::new("D/CX");

        let spi = spi::DebugSerial;
        // let spi = spi::BitBangingSerial::new(gpio::DebugPin::new("SCK"),
        //                                      gpio::DebugPin::new("SDA"));

        let mut driver = driver::Driver::new(spi, dcx, csx);

        driver.clear_screen(0x00);
        driver.draw_line((0, 0), (20, 10), 0x01);
    }

}
