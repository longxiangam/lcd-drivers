use crate::Command;
use core::marker::PhantomData;
use embedded_hal_async::{delay::DelayNs, digital::Wait, spi::SpiDevice};
use embedded_hal_v2::digital::OutputPin;


/// The Connection Interface of all (?) Waveshare EPD-Devices
///
pub(crate) struct DisplayInterface<SPI,  DC, RST, DELAY> {
    /// SPI
    _spi: PhantomData<SPI>,
    /// DELAY
    _delay: PhantomData<DELAY>,


    /// Data/Command Control Pin (High for data, Low for command)
    dc: DC,
    /// Pin for Resetting
    rst: RST,
}

impl<SPI,  DC, RST, DELAY> DisplayInterface<SPI,  DC, RST, DELAY>
where
    SPI: SpiDevice,
    DC: OutputPin,
    RST: OutputPin,
    DELAY: DelayNs,
{
    pub  fn new( dc: DC, rst: RST) -> Self {
        DisplayInterface {
            _spi: PhantomData::default(),
            _delay: PhantomData::default(),
            dc,
            rst,
        }
    }

    /// Basic function for sending [Commands](Command).
    ///
    /// Enables direct interaction with the device with the help of [data()](DisplayInterface::data())
    pub(crate)  async  fn cmd<T: Command>(&mut self, spi: &mut SPI, command: T) -> Result<(), SPI::Error> {
        // low for commands
        let _ = self.dc.set_low();

        // Transfer the command over spi
        self.write(spi, &[command.address()]).await
    }

    /// Basic function for sending an array of u8-values of data over spi
    ///
    /// Enables direct interaction with the device with the help of [command()](Epd4in2::command())
    pub(crate) async fn data(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {
        // high for data
        let _ = self.dc.set_high();

        // Transfer data one u8 at a time over spi
        self.write(spi, data).await

    }
    /// Basic function for sending an array of u8-values of data over spi
    ///
    /// Enables direct interaction with the device with the help of [command()](Epd4in2::command())
    pub(crate) async fn data_all(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {
        // high for data
        let _ = self.dc.set_high();

        // Transfer data one u8 at a time over spi
        self.write_all(spi, data).await

    }
    /// Basic function for sending [Commands](Command) and the data belonging to it.
    ///
    /// TODO: directly use ::write? cs wouldn't needed to be changed twice than
    pub(crate) async fn cmd_with_data<T: Command>(
        &mut self,
        spi: &mut SPI,
        command: T,
        data: &[u8],
    ) -> Result<(), SPI::Error> {
        self.cmd(spi, command).await?;
        self.data(spi, data).await
    }

    /// Basic function for sending the same byte of data (one u8) multiple times over spi
    ///
    /// Enables direct interaction with the device with the help of [command()](ConnectionInterface::command())
    pub(crate) async fn data_x_times(
        &mut self,
        spi: &mut SPI,
        val: u8,
        repetitions: u32,
    ) -> Result<(), SPI::Error> {
        // high for data
        let _ = self.dc.set_high();
        // Transfer data (u8) over spi
        for _ in 0..repetitions {
            self.write(spi, &[val]).await?;
        }
        Ok(())
    }

    // spi write helper/abstraction function
    async fn write(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {


        // transfer spi data
        // Be careful!! Linux has a default limit of 4096 bytes per spi transfer
        // see https://raspberrypi.stackexchange.com/questions/65595/spi-transfer-fails-with-buffer-size-greater-than-4096
        if cfg!(target_os = "linux") {
            for data_chunk in data.chunks(4096) {
                spi.write(data_chunk).await?;
            }
        } else {
            for datum in data {
                spi.write(&[*datum]).await?;
            }

        }



        Ok(())
    }

    async fn write_all(&mut self, spi: &mut SPI, data: &[u8]) -> Result<(), SPI::Error> {

        spi.write(data).await?;

        Ok(())
    }

    /// Resets the device.
    ///
    /// Often used to awake the module from deep sleep. See [Epd4in2::sleep()](Epd4in2::sleep())
    ///
    /// The timing of keeping the reset pin low seems to be important and different per device.
    /// Most displays seem to require keeping it low for 10ms, but the 7in5_v2 only seems to reset
    /// properly with 2ms
    pub(crate) async fn reset(&mut self,  delay:&mut DELAY, duration: u8) -> Result<(), SPI::Error>{
        let _ = self.rst.set_high();
        delay.delay_ms(10).await;

        let _ = self.rst.set_low();
        delay.delay_ms(duration as u32).await;
        let _ = self.rst.set_high();
        //TODO: the upstream libraries always sleep for 200ms here
        // 10ms works fine with just for the 7in5_v2 but this needs to be validated for other devices
        delay.delay_ms(250).await;
        Ok(())
    }
}
