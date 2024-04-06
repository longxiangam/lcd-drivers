use embedded_hal::{
    blocking::{delay::*, spi::Write},
    digital::v2::*,
};

use crate::interface::DisplayInterface;
use crate::traits::InternalWiAdditions;
use crate::{color::TwoBitColor, prelude::*};
use command::Command;

//The Lookup Tables for the Display

/// Width of the display
pub const WIDTH: u32 = 128;
/// Height of the display
pub const HEIGHT: u32 = 96;
/// Default Background Color
pub const DEFAULT_BACKGROUND_COLOR: TwoBitColor = TwoBitColor::White;

mod command;
mod graphics;

///
pub mod prelude {
    pub use crate::st7571::graphics::Display2in3;

    pub use crate::traits::{WaveshareDisplay, WaveshareThreeColorDisplay};

    pub use crate::color::TwoBitColor;
    pub use crate::graphics::TwoBitColorDisplay;
}

/// Lcd2in3 driver
///
pub struct Lcd2in3<SPI, CS, DC, RST, DELAY> {
    /// Connection Interface
    interface: DisplayInterface<SPI, CS, DC, RST, DELAY>,
    /// Background Color
    color: TwoBitColor,
}

impl<SPI, CS, DC, RST, DELAY> InternalWiAdditions<SPI, CS, DC, RST, DELAY>
    for Lcd2in3<SPI, CS, DC, RST, DELAY>
where
    SPI: Write<u8>,
    CS: OutputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayMs<u8>,
{
    fn init(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
        self.interface.reset(delay, 10);

        self.command_u8(spi, 0xE2);

        delay.delay_ms(100);

        self.command_u8(spi, 0xAE);
        self.command_u8(spi, 0x38);
        self.command_u8(spi, 0xF4);
        self.command_u8(spi, 0xA0);
        self.command_u8(spi, 0xC8);

        self.command_u8(spi, 0x44);
        self.command_u8(spi, 0x00);

        self.command_u8(spi, 0x40);
        self.command_u8(spi, 0x00);
    
        self.command_u8(spi, 0xAB);
        self.command_u8(spi, 0x27);

        self.command_u8(spi, 0x81);
        self.command_u8(spi, 40);


        self.command_u8(spi, 0x57);
        self.command_u8(spi, 0x48);
        self.command_u8(spi, 0x61);


        self.command_u8(spi, 0x2C);
        delay.delay_ms(100);

        self.command_u8(spi, 0x2E);
        delay.delay_ms(100);

        self.command_u8(spi, 0x2F);
        delay.delay_ms(10);

        self.command_u8(spi, 0x7B);
        self.command_u8(spi, 0x10);
        self.command_u8(spi, 0x00);


        self.command_u8(spi, 0xa6);
        self.command_u8(spi, 0xa4);
        self.command_u8(spi, 0xaF);

        delay.delay_ms(10);
        Ok(())
    }
}

impl<SPI, CS, DC, RST, DELAY> WaveshareDisplay<SPI, CS, DC, RST, DELAY>
    for Lcd2in3<SPI, CS, DC, RST, DELAY>
where
    SPI: Write<u8>,
    CS: OutputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayMs<u8>,
{
    type DisplayColor = TwoBitColor;
    fn new(spi: &mut SPI, cs: CS, dc: DC, rst: RST, delay: &mut DELAY) -> Result<Self, SPI::Error> {
        let interface = DisplayInterface::new(cs, dc, rst);
        let color = DEFAULT_BACKGROUND_COLOR;

        let mut epd = Lcd2in3 { interface, color };

        epd.init(spi, delay)?;

        Ok(epd)
    }

    fn update_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        _delay: &mut DELAY,
    ) -> Result<(), SPI::Error> {
        self.cmd_with_data_u8(spi, 0x04, &[0x00]);
        self.cmd_with_data_u8(spi, 0x60, &[0x70]);
        self.cmd_with_data_u8(spi, 0x01, buffer)?;
        Ok(())
    }

    fn update_partial_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), SPI::Error> {
        Ok(())
    }

    fn display_frame(&mut self, spi: &mut SPI, _delay: &mut DELAY) -> Result<(), SPI::Error> {
        Ok(())
    }

    fn update_and_display_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        delay: &mut DELAY,
    ) -> Result<(), SPI::Error> {
        Ok(())
    }

    fn clear_frame(&mut self, spi: &mut SPI, _delay: &mut DELAY) -> Result<(), SPI::Error> {
        let color_value = self.color.get_byte_value();




        let j = 0;
        //y
        self.command_u8(spi, (0b10110000 | (j & 0b00001111) as u8) );
        //x

        self.command_u8(spi, (0b00010000 | ((0 >> 4) & 0b00001111)) as u8);
        self.command_u8(spi,  (0  & 0b00001111) as u8);

        for j in 0..HEIGHT {
            for i in 0..WIDTH/2 {

                self.interface.data(spi, &[0x00]);

            }
        }

    

        Ok(())
    }

    fn set_background_color(&mut self, color: TwoBitColor) {
        self.color = color;
    }

    fn background_color(&self) -> &TwoBitColor {
        &self.color
    }

    fn width(&self) -> u32 {
        WIDTH
    }

    fn height(&self) -> u32 {
        HEIGHT
    }
}

impl<SPI, CS, DC, RST, DELAY> Lcd2in3<SPI, CS, DC, RST, DELAY>
where
    SPI: Write<u8>,
    CS: OutputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayMs<u8>,
{


    /// X对应列,值范围0-239
    /// Y对应页,值范围0-23,共24页,每页8行
    pub fn goto(&mut self,spi:&mut SPI,X: u8, Y: u8) {
     
        //y
        self.command_u8(spi, (0b10110000 | (Y & 0b00001111) as u8) );
        //x

        self.command_u8(spi, (0b00010000 | ((X >> 4) & 0b00001111)) as u8);
        self.command_u8(spi,  (X  & 0b00001111) as u8);
        

    }
    ///
    pub fn put_char(&mut self,spi: &mut SPI,data:&[u8]){
        self.interface.data(spi, data);
    }

    fn send_data(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {
        self.interface.data(spi, data)
    }
    fn command_u8(&mut self, spi: &mut SPI, command: u8) -> Result<(), SPI::Error> {
        self.interface.cmd(spi, command)
    }
    fn cmd_with_data_u8(
        &mut self,
        spi: &mut SPI,
        command: u8,
        data: &[u8],
    ) -> Result<(), SPI::Error> {
        self.interface.cmd_with_data(spi, command, data)
    }

    fn command(&mut self, spi: &mut SPI, command: Command) -> Result<(), SPI::Error> {
        self.interface.cmd(spi, command)
    }
    fn cmd_with_data(
        &mut self,
        spi: &mut SPI,
        command: Command,
        data: &[u8],
    ) -> Result<(), SPI::Error> {
        self.interface.cmd_with_data(spi, command, data)
    }

    fn send_resolution(&mut self, spi: &mut SPI) -> Result<(), SPI::Error> {
        Ok(())
    }

    /// Helper function. Sets up the display to send pixel data to a custom
    /// starting point.
    pub fn shift_display(
        &mut self,
        spi: &mut SPI,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), SPI::Error> {
        self.send_data(spi, &[(x >> 8) as u8])?;
        let tmp = x & 0xf8;
        self.send_data(spi, &[tmp as u8])?; // x should be the multiple of 8, the last 3 bit will always be ignored
        let tmp = tmp + width - 1;
        self.send_data(spi, &[(tmp >> 8) as u8])?;
        self.send_data(spi, &[(tmp | 0x07) as u8])?;

        self.send_data(spi, &[(y >> 8) as u8])?;
        self.send_data(spi, &[y as u8])?;

        self.send_data(spi, &[((y + height - 1) >> 8) as u8])?;
        self.send_data(spi, &[(y + height - 1) as u8])?;

        self.send_data(spi, &[0x01])?; // Gates scan both inside and outside of the partial window. (default)

        Ok(())
    }
}
