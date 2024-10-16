
#![no_std]
//#![deny(missing_docs)]


pub mod graphics;

#[cfg(not(feature = "async"))]
mod traits;

#[cfg(feature = "async")]
mod traits_async;
pub mod color;

#[cfg(not(feature = "async"))]
/// Interface for the physical connection between display and the controlling device
mod interface;
#[cfg(feature = "async")]
mod interface_async;

#[cfg(feature = "uc1638")]
///
pub mod uc1638;
#[cfg(feature = "st7571")]
///
pub mod st7571;

#[cfg(feature = "sharp1in26")]
pub mod sharp1in26;

/// Includes everything important besides the chosen Display
pub mod prelude {
    pub use crate::color::{Color, OctColor, TriColor};
    #[cfg(not(feature = "async"))]
    pub use crate::traits::{
          WaveshareDisplay,
    };
    #[cfg(feature = "async")]
    pub use crate::traits_async::{
        WaveshareDisplay,
    };


    pub use crate::graphics::{Display, DisplayRotation, OctDisplay, TriDisplay};

    #[cfg(all(feature = "uc1638",feature = "async"))]
    pub use crate::uc1638::lcd_async::Lcd2in7;
    #[cfg(all(feature = "uc1638",not(feature = "async")))]
    pub use crate::uc1638::lcd_blocking::Lcd2in7;

    #[cfg(feature = "st7571")]
    pub use crate::st7571::Lcd2in3;

    #[cfg(feature = "sharp1in26")]
    pub use crate::sharp1in26::Lcd1in26;
}

/// Computes the needed buffer length. Takes care of rounding up in case width
/// is not divisible by 8.
///
///  unused
///  bits        width
/// <----><------------------------>
/// \[XXXXX210\]\[76543210\]...\[76543210\] ^
/// \[XXXXX210\]\[76543210\]...\[76543210\] | height
/// \[XXXXX210\]\[76543210\]...\[76543210\] v
pub const fn buffer_len(width: usize, height: usize) -> usize {
    (width + 7) / 8 * height
}
#[cfg(feature = "async")]
use embedded_hal_v2::spi::{Mode, Phase, Polarity};
#[cfg(feature = "blocking")]
use embedded_hal::spi::{Mode, Phase, Polarity};



/// All commands need to have this trait which gives the address of the command
/// which needs to be send via SPI with activated CommandsPin (Data/Command Pin in CommandMode)
pub(crate) trait Command {
    fn address(self) -> u8;
}

///支持直接写命令码
impl Command for u8 {
    fn address(self) -> u8 {
        return self
    }
}

