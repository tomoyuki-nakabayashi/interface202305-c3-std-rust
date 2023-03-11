use anyhow::anyhow;
use esp_idf_hal::{delay::FreeRtos, gpio::*, prelude::*, spi};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use ssd1331::{DisplayRotation, Ssd1331};

macro_rules! no_std_anyhow {
    ($e:expr) => {
        ($e).map_err(|e| anyhow!("{:?}, {}:{}", e, std::file!(), std::line!()))
    };
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("never fail");
    let spi = peripherals.spi2;

    let sclk = peripherals.pins.gpio3;
    let sdo = peripherals.pins.gpio4;
    let mut rst = PinDriver::output(peripherals.pins.gpio7)?;
    let dc = PinDriver::output(peripherals.pins.gpio8)?;
    let sdi_not_used: Option<Gpio0> = None;
    let cs_not_used: Option<Gpio0> = None;

    let mut delay = FreeRtos;

    let config = spi::SpiConfig::new().baudrate(4.MHz().into());
    let spi = spi::SpiDeviceDriver::new_single(
        spi,
        sclk,
        sdo,
        sdi_not_used,
        spi::Dma::Disabled,
        cs_not_used,
        &config,
    )?;

    let mut disp = Ssd1331::new(spi, dc, DisplayRotation::Rotate0);
    no_std_anyhow!(disp.reset(&mut rst, &mut delay))?;
    no_std_anyhow!(disp.init())?;
    disp.clear();
    no_std_anyhow!(disp.flush())?;

    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    Text::new("hello C3!", Point::new(20, 30), style).draw(&mut disp)?;
    no_std_anyhow!(disp.flush())?;

    Ok(())
}
