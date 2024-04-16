//! A simple Driver for the [Waveshare](https://github.com/waveshare/e-Paper) E-Ink Displays via SPI
//!
//! - Built using [`embedded-hal`] traits.
//! - Graphics support is added through [`embedded-graphics`]
//!
//! [`embedded-graphics`]: https://docs.rs/embedded-graphics/
//! [`embedded-hal`]: https://docs.rs/embedded-hal
//!

//!
//! # Example
//!
//!```rust, no_run
//!# use embedded_hal_mock::*;
//!# fn main() -> Result<(), MockError> {
//!use embedded_graphics::{
//!    pixelcolor::BinaryColor::On as Black, prelude::*, primitives::{Line, PrimitiveStyle},
//!};
//!use epd_waveshare::{epd1in54::*, prelude::*};
//!#
//!# let expectations = [];
//!# let mut spi = spi::Mock::new(&expectations);
//!# let expectations = [];
//!# let cs_pin = pin::Mock::new(&expectations);
//!# let busy_in = pin::Mock::new(&expectations);
//!# let dc = pin::Mock::new(&expectations);
//!# let rst = pin::Mock::new(&expectations);
//!# let mut delay = delay::MockNoop::new();
//!
//!// Setup EPD
//!let mut epd = Epd1in54::new(&mut spi, cs_pin, busy_in, dc, rst, &mut delay)?;
//!
//!// Use display graphics from embedded-graphics
//!let mut display = Display1in54::default();
//!
//!// Use embedded graphics for drawing a line
//!
//!let _ = Line::new(Point::new(0, 120), Point::new(0, 295))
//!    .into_styled(PrimitiveStyle::with_stroke(Black, 1))
//!    .draw(&mut display);
//!
//!    // Display updated frame
//!epd.update_frame(&mut spi, &display.buffer(), &mut delay)?;
//!epd.display_frame(&mut spi, &mut delay)?;
//!
//!// Set the EPD to sleep
//!epd.sleep(&mut spi, &mut delay)?;
//!# Ok(())
//!# }
//!```
//!
//! # Other information and requirements
//!
//! - Buffersize: Wherever a buffer is used it always needs to be of the size: `width / 8 * length`,
//!   where width and length being either the full e-ink size or the partial update window size
//!
//! ### SPI
//!
//! MISO is not connected/available. SPI_MODE_0 is used (CPHL = 0, CPOL = 0) with 8 bits per word, MSB first.
//!
//! Maximum speed tested by myself was 8Mhz but more should be possible (Ben Krasnow used 18Mhz with his implemenation)
//!
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
    pub use crate::SPI_MODE;

    pub use crate::graphics::{Display, DisplayRotation, OctDisplay, TriDisplay};

    #[cfg(all(feature = "uc1638",feature = "async"))]
    pub use crate::uc1638::lcd_async::Lcd2in7;
    #[cfg(all(feature = "uc1638",not(feature = "async")))]
    pub use crate::uc1638::lcd_blocking::Lcd2in7;

    #[cfg(feature = "st7571")]
    pub use crate::st7571::Lcd2in3;
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

use embedded_hal::spi::{Mode, Phase, Polarity};

/// SPI mode -
/// For more infos see [Requirements: SPI](index.html#spi)
pub const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};



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

