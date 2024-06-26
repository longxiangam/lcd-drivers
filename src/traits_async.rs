use core::marker::Sized;
use embedded_hal_async::{delay::DelayNs, digital::Wait, spi::SpiDevice};
use embedded_hal_v2::digital::OutputPin;

pub(crate) trait InternalWiAdditions<SPI, DC, RST, DELAY>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    /// This initialises the EPD and powers it up
    ///
    /// This function is already called from
    ///  - [new()](WaveshareDisplay::new())
    ///  - [`wake_up`]
    ///
    ///
    /// This function calls [reset](WaveshareDisplay::reset),
    /// so you don't need to call reset your self when trying to wake your device up
    /// after setting it to sleep.
    async fn init(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error>;
}



pub trait WaveshareDisplay<SPI,   DC, RST, DELAY>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    /// The Color Type used by the Display
    type DisplayColor;
    /// Creates a new driver from a SPI peripheral, Busy InputPin, DC
    ///
    /// This already initialises the device.
    async fn new(
        spi: &mut SPI,
        dc: DC,
        rst: RST,
        delay: &mut DELAY,
    ) -> Result<Self, SPI::Error>
    where
        Self: Sized;

    /// Sets the backgroundcolor for various commands like [clear_frame](WaveshareDisplay::clear_frame)
    fn set_background_color(&mut self, color: Self::DisplayColor);

    /// Get current background color
    fn background_color(&self) -> &Self::DisplayColor;

    /// Get the width of the display
    fn width(&self) -> u32;

    /// Get the height of the display
    fn height(&self) -> u32;

    /// Transmit a full frame to the SRAM of the EPD
    async fn update_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        delay: &mut DELAY
    ) -> Result<(), SPI::Error>;

    /// Transmits partial data to the SRAM of the EPD
    ///
    /// (x,y) is the top left corner
    ///
    /// BUFFER needs to be of size: width / 8 * height !
    async  fn update_partial_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), SPI::Error>;

    /// Displays the frame data from SRAM
    ///
    /// This function waits until the device isn`t busy anymore
    async  fn display_frame(&mut self, spi: &mut SPI,delay: &mut DELAY) -> Result<(), SPI::Error>;

    /// Provide a combined update&display and save some time (skipping a busy check in between)
    async  fn update_and_display_frame(
        &mut self,
        spi: &mut SPI,
        buffer: &[u8],
        delay: &mut DELAY
    ) -> Result<(), SPI::Error>;

    /// Clears the frame buffer on the EPD with the declared background color
    ///
    /// The background color can be changed with [`WaveshareDisplay::set_background_color`]
    async  fn clear_frame(&mut self, spi: &mut SPI, delay: &mut DELAY) -> Result<(), SPI::Error>;

  
   
}
