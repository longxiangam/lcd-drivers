use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Size;
use embedded_graphics::Pixel;
use embedded_graphics::geometry::OriginDimensions;

use crate::color::TwoBitColor;
use crate::graphics::TwoBitColorDisplay;
use crate::st7571::{DEFAULT_BACKGROUND_COLOR, HEIGHT, WIDTH};
use crate::graphics::{Display, DisplayRotation};

/// 
pub struct Display2in3 {
    buffer: [u8; (WIDTH  * HEIGHT /4 ) as usize],
    rotation: DisplayRotation,
}

impl Default for Display2in3 {
    fn default() -> Self {
        Display2in3 {
            buffer: [DEFAULT_BACKGROUND_COLOR.get_byte_value();
                WIDTH as usize * HEIGHT as usize / 4],
            rotation: DisplayRotation::default(),
        }
    }
}


impl DrawTarget for Display2in3 {
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


impl OriginDimensions for Display2in3 {
    fn size(&self) -> Size {
        Size::new(WIDTH, HEIGHT)
    }
}


impl TwoBitColorDisplay for Display2in3 {
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
        pixel: Pixel<TwoBitColor>,
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
            TwoBitColor::Black => {
                // clear bit in bw-buffer -> black

                buffer[index] |= bit;
                buffer[index+1] |= bit;
            }
            TwoBitColor::White => {
                // set bit in bw-buffer -> white
                buffer[index] &= !bit;
                buffer[index+1] &= !bit;
            },
            TwoBitColor::Gray1 => {
                buffer[index] &= !bit;
                buffer[index+1] |= bit;
            },
            TwoBitColor::Gray2 =>{
                buffer[index] |= bit;
                buffer[index+1] &= !bit;
            }
        }

        Ok(())
    }
}
#[rustfmt::skip]
fn find_position(x: u32, y: u32, width: u32, height: u32, rotation: DisplayRotation) -> (u32, u8) {
    let (nx, ny) = crate::graphics::find_rotation(x, y, width, height, rotation);
    //每列的八个像素放在两个字节中，底位在前
    let row = ny / 8;
    let col = nx;

    (
        (width * 2) * row + (col * 2),
        0x01 << (ny %8)
    )
}