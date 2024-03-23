use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Size;
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::Pixel;
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::pixelcolor::{Gray2, GrayColor};
use embedded_graphics::pixelcolor::raw::RawU2;
use embedded_graphics::prelude::{PixelColor, Point};
use crate::uc1638::{DEFAULT_BACKGROUND_COLOR, HEIGHT, WIDTH};
use crate::graphics::{Display, DisplayRotation};
/// Full size buffer for use with the 4in2 EPD
///
/// Can also be manuall constructed:
/// `buffer: [DEFAULT_BACKGROUND_COLOR.get_byte_value(); WIDTH / 8 * HEIGHT]`
pub struct Display3in27 {
    buffer: [u8; WIDTH as usize * HEIGHT as usize / 4],
    rotation: DisplayRotation,
}

impl Default for Display3in27 {
    fn default() -> Self {
        Display3in27 {
            buffer: [DEFAULT_BACKGROUND_COLOR.get_byte_value();
                WIDTH as usize * HEIGHT as usize / 4],
            rotation: DisplayRotation::default(),
        }
    }
}


impl DrawTarget for Display3in27 {
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


impl OriginDimensions for Display3in27 {
    fn size(&self) -> Size {
        Size::new(WIDTH, HEIGHT)
    }
}


impl TwoBitColorDisplay for Display3in27 {
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

/***
定义颜色
 */
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TwoBitColor {
    /// Inactive pixel.
    Black,

    /// Active pixel.
    White,
    Gray1,
    Gray2,
}

impl Default for TwoBitColor {
    fn default() -> Self {
        Self::Black
    }
}

impl TwoBitColor {

    #[inline]
    pub fn invert(self) -> Self {
        match self {
            TwoBitColor::White => TwoBitColor::Black,
            TwoBitColor::Black => TwoBitColor::White,
            TwoBitColor::Gray1 => TwoBitColor::Gray2,
            TwoBitColor::Gray2 => TwoBitColor::Gray1,
        }
    }


    #[inline]
    pub fn is_on(self) -> bool {
        self == TwoBitColor::White
    }


    #[inline]
    pub fn is_off(self) -> bool {
        self == TwoBitColor::Black
    }


}

impl PixelColor for TwoBitColor {
    type Raw = RawU2;
}

//定义一个 trait 继承自DrawTarget
// 实现此trait 后可以被用于 实现了 Drawable 类型的 draw 方法中
pub trait TwoBitColorDisplay: DrawTarget<Color = TwoBitColor> {
    /// Clears the buffer of the display with the chosen background color
/*    fn clear_buffer(&mut self, background_color: TwoBitColor) {
        for elem in self.get_mut_buffer().iter_mut() {
            *elem = TwoBitColor::colors_byte(background_color, background_color);
        }
    }*/

    /// Returns the buffer
    fn buffer(&self) -> &[u8];

    /// Returns a mutable buffer
    fn get_mut_buffer(&mut self) -> &mut [u8];

    /// Sets the rotation of the display
    fn set_rotation(&mut self, rotation: DisplayRotation);

    /// Get the current rotation of the display
    fn rotation(&self) -> DisplayRotation;

    /// Helperfunction for the Embedded Graphics draw trait
    ///
    /// Becomes uneccesary when const_generics become stablised
    fn draw_helper(
        &mut self,
        width: u32,
        height: u32,
        pixel: Pixel<TwoBitColor>,
    ) -> Result<(), Self::Error> {
        let rotation = self.rotation();
        let buffer = self.get_mut_buffer();

        let Pixel(point, color) = pixel;
        if outside_display(point, width, height, rotation) {
            return Ok(());
        }

        // Give us index inside the buffer and the bit-position in that u8 which needs to be changed

        let (index, bit) =
            find_gray2_position(point.x as u32, point.y as u32, width, height, rotation);
        let index = index as usize;

        // "Draw" the Pixel on that bit
        match color {
            TwoBitColor::Black => {
                // clear bit in bw-buffer -> black
                buffer[index] &= !bit;
            }
            TwoBitColor::White => {
                // set bit in bw-buffer -> white
                buffer[index] |= bit;
            },
            TwoBitColor::Gray1 => {
                let code:u8 =0x55  & bit;
                buffer[index] &= !bit;//先置为零
                buffer[index] |= code;
            },
            TwoBitColor::Gray2 =>{
                let code:u8 = 0xAA & bit;
                buffer[index] &= !bit;//先置为零
                buffer[index] |= code;
            }
        }

        Ok(())
    }
}

//是否超出显示范围
fn outside_display(p: Point, width: u32, height: u32, rotation: DisplayRotation) -> bool {
    if p.x < 0 || p.y < 0 {
        return true;
    }
    let (x, y) = (p.x as u32, p.y as u32);
    match rotation {
        DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
            if x >= width || y >= height {
                return true;
            }
        }
        DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
            if y >= width || x >= height {
                return true;
            }
        }
    }
    false
}

//旋转x y
fn find_rotation(x: u32, y: u32, width: u32, height: u32, rotation: DisplayRotation) -> (u32, u32) {
    let nx;
    let ny;
    match rotation {
        DisplayRotation::Rotate0 => {
            nx = x;
            ny = y;
        }
        DisplayRotation::Rotate90 => {
            nx = width - 1 - y;
            ny = x;
        }
        DisplayRotation::Rotate180 => {
            nx = width - 1 - x;
            ny = height - 1 - y;
        }
        DisplayRotation::Rotate270 => {
            nx = y;
            ny = height - 1 - x;
        }
    }
    (nx, ny)
}

#[rustfmt::skip]
fn find_gray2_position(x: u32, y: u32, width: u32, height: u32, rotation: DisplayRotation) -> (u32, u8) {
    let (nx, ny) = find_rotation(x, y, width, height, rotation);
    //每个字节描述4个像素，每两个bit
    (
        nx / 4 + (width  / 4) * ny,
        0xc0 >> (nx % 4)*2 ,
    )
}