

use bresenham;
use command::{Command, PixelFormat, GammaCurve};
use gpio;

use sleep;
use spi;

type Color = u16;

pub fn parse_color(r: u8, g: u8, b: u8) -> Color {
    let r = ((r & 0b11111000) as u16) << 8;
    let g = ((g & 0b11111100) as u16) << 3;
    let b = ((b & 0b11111000) as u16) >> 3;
    r | g | b
}

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

    pub fn write_pixel(&mut self, color: Color) {
        // With 16-bit colors (5-6-5), we just write the two bytes.
        self.write_data_16(color);
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

    /// Sets the writeable memory window.
    ///
    /// Both start and end are inclusive.
    pub fn set_window(&mut self, (startx, starty): (u16, u16),
                      (endx, endy): (u16, u16)) {
        self.write_cmd(Command::ColumnAddressSet);
        self.write_data_16(startx);
        self.write_data_16(endx);

        self.write_cmd(Command::PageAddressSet);
        self.write_data_16(starty);
        self.write_data_16(endy);
    }

    // This method is enough to implement all high-level methods
    pub fn fill_rect(&mut self, (x, y): (u16, u16),
                     (width, height): (u16, u16), color: Color) {
        self.set_window((x, y), (x + width - 1, y + width - 1));
        self.write_cmd(Command::MemoryWrite);
        for _ in 0..(width * height) {
            self.write_pixel(color);
        }
    }

    // High-level interface

    // TODO: handle 18-bits colors?
    pub fn clear_screen(&mut self, color: Color) {
        // TODO: hard-code 128x128? Or use some variable?
        self.fill_rect((0, 0), (128, 128), color);
    }

    pub fn draw_hline(&mut self, (x, y): (u16, u16), length: u16,
                      color: Color) {
        self.fill_rect((x, y), (length, 1), color);
    }

    pub fn draw_vline(&mut self, (x, y): (u16, u16), length: u16,
                      color: Color) {
        self.fill_rect((x, y), (1, length), color);
    }

    pub fn draw_rect(&mut self, (x, y): (u16, u16),
                     (width, height): (u16, u16), color: Color) {

        if height > 1 {
            self.draw_hline((x, y + height - 1), width, color);
            self.draw_vline((x, y), height, color);
        }

        if width > 1 {
            self.draw_hline((x, y), width, color);
            self.draw_vline((x + width - 1, y), height, color);
        }
    }

    pub fn draw_pixel(&mut self, (x, y): (u16, u16), color: Color) {
        self.fill_rect((x, y), (1, 1), color);
    }


    // TODO: move rasterization to external crate

    pub fn draw_line(&mut self, (x1, y1): (u16, u16), (x2, y2): (u16, u16),
                     color: Color) {
        // Rasterize the line as a series of vertical/horizontal lines

        let start = (x1 as isize, y1 as isize);
        let end = (x2 as isize, y2 as isize);

        for (x, y) in bresenham::Bresenham::new(start, end) {
            self.draw_pixel((x as u16, y as u16), color);
        }
        self.draw_pixel((x2, y2), color);
    }

    pub fn draw_circle(&mut self, (x, y): (u16, u16), radius: u16,
                       color: Color) {
        // TODO: Rasterize the circle as a series of vertical/horizontal lines
        let mut dx = radius;
        let mut dy = 0;
        let mut err: i16 = 0;

        while dx >= dy {
            self.draw_pixel((x + dx, y + dy), color);
            self.draw_pixel((x + dy, y + dx), color);

            self.draw_pixel((x - dx, y + dy), color);
            self.draw_pixel((x - dy, y + dx), color);

            self.draw_pixel((x + dx, y - dy), color);
            self.draw_pixel((x + dy, y - dx), color);

            self.draw_pixel((x - dx, y - dy), color);
            self.draw_pixel((x - dy, y - dx), color);

            dy += 1;
            err += 1 + 2 * dy as i16;

            if 2 * (err - dx as i16) + 1 > 0 {
                dx -= 1;
                err += 1 - 2 * dx as i16;
            }
        }
    }

    pub fn fill_circle(&mut self, (x, y): (u16, u16), radius: u16,
                       color: Color) {
        // TODO: Rasterize the circle as a series of horizontal lines
    }

    pub fn draw_text(&mut self, (x, y): (u16, u16), size: u16, text: &str,
                     color: Color) {
        // TODO:
        // load font from external file at compilation time (`include_bytes!`)
    }
}
