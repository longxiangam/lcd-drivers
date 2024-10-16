use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Size;
use embedded_graphics::Pixel;
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics_core::pixelcolor::BinaryColor;
use crate::color::Color;
use crate::sharp1in26::{DEFAULT_BACKGROUND_COLOR, HEIGHT, WIDTH};
use crate::graphics::{Display, DisplayRotation};

/// 
pub struct Display1in26 {
    buffer: [u8; (WIDTH  * HEIGHT /8 ) as usize],
    rotation: DisplayRotation,
}

impl Default for Display1in26 {
    fn default() -> Self {
        Display1in26 {
            buffer: [0x00;
                WIDTH as usize * HEIGHT as usize / 8],
            rotation: DisplayRotation::default(),
        }
    }
}


impl DrawTarget for Display1in26 {
    type Color = BinaryColor;
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


impl OriginDimensions for Display1in26 {
    fn size(&self) -> Size {
        Size::new(WIDTH, HEIGHT)
    }
}


impl Display for Display1in26 {
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


    fn draw_helper(
        &mut self,
        width: u32,
        height: u32,
        pixel: Pixel<BinaryColor>,
    ) -> Result<(), Self::Error> {
        let rotation = self.rotation();
        let buffer = self.get_mut_buffer();

        let Pixel(point, color) = pixel;
        if crate::graphics::outside_display(point, width, height, rotation) {
            return Ok(());
        }

        // Give us index inside the buffer and the bit-position in that u8 which needs to be changed

        let (index, bit) =
           find_position(point.x as u32, point.y as u32, width, height, rotation);
        let index = index as usize;

        // "Draw" the Pixel on that bit
        match color {
            BinaryColor::On => {
                // clear bit in bw-buffer -> black
                buffer[index] |= bit;
            }
            BinaryColor::Off => {
                // set bit in bw-buffer -> white
                buffer[index] &= !bit;
            },
        }

        Ok(())
    }
}
#[rustfmt::skip]
fn find_position(x: u32, y: u32, width: u32, height: u32, rotation: DisplayRotation) -> (u32, u8) {
    let (nx, ny) = crate::graphics::find_rotation(x, y, width, height, rotation);
    //+防止宽度不是8的整陪数
    (
        (width+7)/8 * ny + nx / 8,
        0x01 <<(nx % 8)
    )
}