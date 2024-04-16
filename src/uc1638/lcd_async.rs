use core::marker::Sized;
use embedded_hal_async::{delay::DelayNs, digital::Wait, spi::SpiDevice};
use embedded_hal::digital::OutputPin;
use crate::interface_async::DisplayInterface;
use crate::traits_async::{InternalWiAdditions, WaveshareDisplay};
use crate::{color::TwoBitColor, prelude::*};
use crate::uc1638::command::Command;
use crate::uc1638::{DEFAULT_BACKGROUND_COLOR, HEIGHT, WIDTH};

/// Lcd2in7 driver
///
pub struct Lcd2in7<SPI, CS, DC, RST, DELAY> {
    /// Connection Interface
    interface: DisplayInterface<SPI, CS, DC, RST, DELAY>,
    /// Background Color
    color: TwoBitColor,
}

impl<SPI, CS, DC, RST, DELAY> InternalWiAdditions<SPI, CS, DC, RST, DELAY>
for Lcd2in7<SPI, CS, DC, RST, DELAY>
    where
        SPI: SpiDevice,
        CS: OutputPin,
        DC: OutputPin,
        RST: OutputPin,
        DELAY: DelayNs,
{
   async fn init(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error> {
        self.interface.reset(delay, 10).await?;

        self.cmd_with_data_u8(spi, 0xE1, &[0xE2]);

        delay.delay_ms(10);

        self.cmd_with_data_u8(spi, 0x04, &[0x00]);

        self.command_u8(spi, 0xEB);

        self.cmd_with_data_u8(spi, 0x81, &[80]);

        self.cmd_with_data_u8(spi, 0xb8, &[0x00]);

        self.command_u8(spi, 0xa3);
        self.command_u8(spi, 0x94);
        self.command_u8(spi, 0xc4);

        self.command_u8(spi, 0x60);
        self.command_u8(spi, 0x70);

        self.cmd_with_data_u8(spi, 0xf1, &[95]);;

        self.command_u8(spi, 0xd2);
        self.command_u8(spi, 0xd5);

        self.cmd_with_data_u8(spi, 0xc9, &[0xaF]);

        delay.delay_ms(100);
        Ok(())
    }
}

impl<SPI, CS, DC, RST, DELAY> WaveshareDisplay<SPI, CS, DC, RST, DELAY>
for Lcd2in7<SPI, CS, DC, RST, DELAY>
    where
        SPI: SpiDevice,
        CS: OutputPin,
        DC: OutputPin,
        RST: OutputPin,
        DELAY: DelayNs,
{
    type DisplayColor = TwoBitColor;
    fn new(spi: &mut SPI, cs: CS, dc: DC, rst: RST, delay: &mut DELAY) -> Result<Self, SPI::Error> {
        let interface = DisplayInterface::new(cs, dc, rst);
        let color = DEFAULT_BACKGROUND_COLOR;

        let mut epd = Lcd2in7 { interface, color };

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

        self.cmd_with_data_u8(spi, 0x04, &[0x00]);
        self.command_u8(spi, 0x60);
        self.command_u8(spi, 0x70);

        for i in 0..WIDTH {
            for j in 0..(HEIGHT / 2) {

                self.cmd_with_data_u8(spi, 0x01, &[0x00]);

            }
        }

        /*   self.goto(spi, 40, 40);
          self.cmd_with_data_u8(spi, 0x01, &[0xFF,0x00,0xFF,0xFF]); */

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

impl<SPI, CS, DC, RST, DELAY> Lcd2in7<SPI, CS, DC, RST, DELAY>
    where
        SPI: SpiDevice,
        CS: OutputPin,
        DC: OutputPin,
        RST: OutputPin,
        DELAY: DelayNs,
{
    /// X对应列,值范围0-239
    /// Y对应页,值范围0-23,共24页,每页8行
    pub async fn goto(&mut self,spi:&mut SPI,X: u8, Y: u8) -> Result<(), SPI::Error>{
        let mut YY = Y;
        if(Y>23){YY=0;}	//保证页面代码最大为11

        self.cmd_with_data_u8(spi, 0x04, &[X]).await?;


        self.command_u8( spi,0x60|(YY&0x0F) ).await?;
        self.command_u8( spi,0x70|(YY>>4) ).await

        //设置PA[3:0],页地址的D3-D0位
        //由于此屏幕只有96行,每8行为一页,所以只有12页,UC1638页面设置寄存器是6位,
        //12页的话只用低4位就可以全部表达清楚,所以不用对高2位进行设置,默认高2位=00即可.
        //writei( 0x70|(Y>>4) ); 	//设置PA[5:4],页地址的D5,D4位
    }
    ///
    pub async fn put_char(&mut self,spi: &mut SPI,data:&[u8])-> Result<(), SPI::Error>{
        self.cmd_with_data_u8(spi, 0x01, data).await
    }

    async fn send_data(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {
        self.interface.data(spi, data).await
    }
    async fn command_u8(&mut self, spi: &mut SPI, command: u8) -> Result<(), SPI::Error> {
        self.interface.cmd(spi, command).await
    }
    async fn cmd_with_data_u8(
        &mut self,
        spi: &mut SPI,
        command: u8,
        data: &[u8],
    ) -> Result<(), SPI::Error> {
        self.interface.cmd_with_data(spi, command, data).await
    }

    async fn command(&mut self, spi: &mut SPI, command: Command) -> Result<(), SPI::Error> {
        self.interface.cmd(spi, command).await
    }
    async fn cmd_with_data(
        &mut self,
        spi: &mut SPI,
        command: Command,
        data: &[u8],
    ) -> Result<(), SPI::Error> {
        self.interface.cmd_with_data(spi, command, data).await
    }

    async fn send_resolution(&mut self, spi: &mut SPI) -> Result<(), SPI::Error> {
        Ok(())
    }

    /// Helper function. Sets up the display to send pixel data to a custom
    /// starting point.
    pub  async   fn shift_display(
        &mut self,
        spi: &mut SPI,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), SPI::Error> {
        self.send_data(spi, &[(x >> 8) as u8]).await?;
        let tmp = x & 0xf8;
        self.send_data(spi, &[tmp as u8]).await?; // x should be the multiple of 8, the last 3 bit will always be ignored
        let tmp = tmp + width - 1;
        self.send_data(spi, &[(tmp >> 8) as u8]).await?;
        self.send_data(spi, &[(tmp | 0x07) as u8]).await?;

        self.send_data(spi, &[(y >> 8) as u8]).await?;
        self.send_data(spi, &[y as u8]).await?;

        self.send_data(spi, &[((y + height - 1) >> 8) as u8]).await?;
        self.send_data(spi, &[(y + height - 1) as u8]).await?;

        self.send_data(spi, &[0x01]).await?; // Gates scan both inside and outside of the partial window. (default)

        Ok(())
    }
}
