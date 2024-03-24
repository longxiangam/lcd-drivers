use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Size;
use embedded_graphics::Pixel;
use embedded_graphics::geometry::OriginDimensions;

use crate::color::TwoBitColor;
use crate::graphics::TwoBitColorDisplay;
use crate::uc1638::{DEFAULT_BACKGROUND_COLOR, HEIGHT, WIDTH};
use crate::graphics::{Display, DisplayRotation};

/// 
pub struct Display2in7 {
    buffer: [u8; WIDTH as usize * HEIGHT as usize / 4],
    rotation: DisplayRotation,
}

impl Default for Display2in7 {
    fn default() -> Self {
        Display2in7 {
            buffer: [DEFAULT_BACKGROUND_COLOR.get_byte_value();
                WIDTH as usize * HEIGHT as usize / 4],
            rotation: DisplayRotation::default(),
        }
    }
}


impl DrawTarget for Display2in7 {
    type Color = TwoBitColor;
    type Error = core::convert::Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where
            I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.draw_helper(WIDTH, HEIGHT, pixel)?;
        }
        Ok(())
    }
}


impl OriginDimensions for Display2in7 {
    fn size(&self) -> Size {
        Size::new(WIDTH, HEIGHT)
    }
}


impl TwoBitColorDisplay for Display2in7 {
    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn get_mut_buffer(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    fn set_rotation(&mut self, rotation: DisplayRotation) {
        self.rotation = rotation;
    }

    fn rotation(&self) -> DisplayRotation {
        self.rotation
    }
}
