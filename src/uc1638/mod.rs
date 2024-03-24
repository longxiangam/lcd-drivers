use embedded_hal::{
    blocking::{delay::*, spi::Write},
    digital::v2::*,
};


use crate::{ prelude::*};
use crate::interface::DisplayInterface;
use crate::traits::InternalWiAdditions;
use command::Command;

//The Lookup Tables for the Display


/// Width of the display
pub const WIDTH: u32 = 240;
/// Height of the display
pub const HEIGHT: u32 = 96;
/// Default Background Color
pub const DEFAULT_BACKGROUND_COLOR: Color = Color::White;
const IS_BUSY_LOW: bool = true;


mod graphics;
mod command;



pub mod prelude {

    pub use crate::traits::{
          WaveshareDisplay, WaveshareThreeColorDisplay,
    };
    
    pub use crate::color::TwoBitColor;
    pub use crate::graphics::TwoBitColorDisplay;


}



/// Lcd2in7 driver
///
pub struct Lcd2in7<SPI, CS, BUSY, DC, RST, DELAY> {
    /// Connection Interface
    interface: DisplayInterface<SPI, CS, BUSY, DC, RST, DELAY>,
    /// Background Color
    color: Color,

}

impl<SPI, CS, BUSY, DC, RST, DELAY> InternalWiAdditions<SPI, CS, BUSY, DC, RST, DELAY>
    for Lcd2in7<SPI, CS, BUSY, DC, RST, DELAY>
where
    SPI: Write<u8>,
    CS: OutputPin,
    BUSY: InputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayMs<u8>,
{
    fn init(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
      
        Ok(())
    }
}

impl<SPI, CS, BUSY, DC, RST, DELAY> WaveshareDisplay<SPI, CS, BUSY, DC, RST, DELAY>
    for Lcd2in7<SPI, CS, BUSY, DC, RST, DELAY>
where
    SPI: Write<u8>,
    CS: OutputPin,
    BUSY: InputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayMs<u8>,
{
    type DisplayColor = Color;
    fn new(
        spi: &mut SPI,
        cs: CS,
        busy: BUSY,
        dc: DC,
        rst: RST,
        delay: &mut DELAY,
    ) -> Result<Self, SPI::Error> {
        let interface = DisplayInterface::new(cs, busy, dc, rst);
        let color = DEFAULT_BACKGROUND_COLOR;

        let mut epd = Lcd2in7 {
            interface,
            color,
        };

        epd.init(spi, delay)?;

        Ok(epd)
    }

   
    fn update_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        _delay: &mut DELAY,
    ) -> Result<(), SPI::Error> {
/*        let color_value = self.color.get_byte_value();

        self.interface.cmd(spi, Command::DataStartTransmission1)?;
        self.interface
            .data_x_times(spi, color_value, WIDTH / 8 * HEIGHT)?;

*/        /* self.interface
            .cmd_with_data(spi, Command::DataStartTransmission1, buffer)?; */
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
      
        Ok(())
    }

    fn set_background_color(&mut self, color: Color) {
        self.color = color;
    }

    fn background_color(&self) -> &Color {
        &self.color
    }

    fn width(&self) -> u32 {
        WIDTH
    }

    fn height(&self) -> u32 {
        HEIGHT
    }

    
}

impl<SPI, CS, BUSY, DC, RST, DELAY> Lcd2in7<SPI, CS, BUSY, DC, RST, DELAY>
where
    SPI: Write<u8>,
    CS: OutputPin,
    BUSY: InputPin,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayMs<u8>,
{
    fn command(&mut self, spi: &mut SPI, command: Command) -> Result<(), SPI::Error> {
        self.interface.cmd(spi, command)
    }

    fn send_data(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {
        self.interface.data(spi, data)
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
