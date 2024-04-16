
use crate::{color::TwoBitColor, prelude::*};
use command::Command;

//The Lookup Tables for the Display

/// Width of the display
pub const WIDTH: u32 = 240;
/// Height of the display
pub const HEIGHT: u32 = 96;
/// Default Background Color
pub const DEFAULT_BACKGROUND_COLOR: TwoBitColor = TwoBitColor::White;

mod command;
mod graphics;
#[cfg(feature = "async")]
pub mod lcd_async;


#[cfg(not(feature = "async"))]
pub mod lcd_blocking;




///
pub mod prelude {
    pub use crate::uc1638::graphics::Display2in7;

    #[cfg(not(feature = "async"))]
    pub use crate::traits::{WaveshareDisplay };
    #[cfg(feature = "async")]
    pub use crate::traits_async::{WaveshareDisplay };
    pub use crate::color::TwoBitColor;
    pub use crate::graphics::TwoBitColorDisplay;
}
