use core::marker::PhantomData;
use embedded_graphics_core::pixelcolor::BinaryColor;
use embedded_hal::{
    blocking::{delay::*, spi::Write},
    digital::v2::*,
};

use crate::interface::DisplayInterface;
use crate::traits::InternalWiAdditions;
use crate::{color::Color, prelude::*};

//The Lookup Tables for the Display

/// Width of the display
pub const WIDTH: u32 = 144;
/// Height of the display
pub const HEIGHT: u32 = 168;
/// Default Background Color
pub const DEFAULT_BACKGROUND_COLOR: BinaryColor = BinaryColor::Off;
const DISP_MODE:u8 = 0b00000010; // L display mode;
const UPDATE_MODE:u8 = 0b00000011;  // H memory mode;
mod graphics;

///
pub mod prelude {
    pub use crate::sharp1in26::graphics::Display1in26;

    pub use crate::graphics::Display;
}

/// Lcd1in26 driver
///
pub struct Lcd1in26<SPI, CS,  DELAY> {
    /// SPI
    _spi: PhantomData<SPI>,
    /// DELAY
    _delay: PhantomData<DELAY>,
    /// CS for SPI
    cs: CS,
    /// Background Color
    color: BinaryColor,
}




impl<SPI, CS,  DELAY> Lcd1in26<SPI, CS, DELAY>
where
    SPI: Write<u8>,
    CS: OutputPin,
    DELAY: DelayMs<u8>,
{
    pub fn new(spi: &mut SPI, cs: CS, delay: &mut DELAY) -> Result<Self, SPI::Error> {
        let color = DEFAULT_BACKGROUND_COLOR;

        let mut lcd = crate::sharp1in26::Lcd1in26 {
            _spi:PhantomData::default(),
            _delay:PhantomData::default(),
            cs,
            color
        };

        Ok(lcd)
    }
    pub  fn set_background_color(&mut self, color: BinaryColor) {
        self.color = color;
    }

    pub  fn background_color(&self) -> &BinaryColor {
        &self.color
    }

    pub  fn width(&self) -> u32 {
        WIDTH
    }

    pub  fn height(&self) -> u32 {
        HEIGHT
    }

    pub fn clear(&mut self, spi: &mut SPI){
        self.cs.set_high();

       // spi.write( &[0b00000110, 0b00000000]);

        spi.write( &[0b00000110]);
        spi.write( &[0b00000000]);
        self.cs.set_low();
    }
    pub  fn update_frame(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {

        self.cs.set_high();
        spi.write(&[UPDATE_MODE]);

        let row_bytes:u32 = self.width()  / 8;
        for i in 0..self.height()   {

            spi.write(&[(i+1) as u8]);

            for j in 0.. row_bytes{
                let index = i*18+j;
                let mut temp = data[index as usize];
                temp = !temp;

                spi.write(&[temp]);
            }

            spi.write(&[0x00]);
        }

        spi.write(&[0x00]);
        spi.write(&[0x00]);

        self.cs.set_low();

        Ok(())
    }


}
