#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::{gpio::*, spi};
use common::wifi;

mod server;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("never fail");

    let _wifi = wifi::connect(peripherals.modem, CONFIG.wifi_ssid, CONFIG.wifi_psk)?;

    let spi = peripherals.spi2;
    let sclk = peripherals.pins.gpio3;
    let sdo = peripherals.pins.gpio4;
    let rst = PinDriver::output(peripherals.pins.gpio7)?;
    let dc = PinDriver::output(peripherals.pins.gpio8)?;
    let sdi_not_used: Option<Gpio0> = None;
    let cs_not_used: Option<Gpio0> = None;

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
    server::spawn_server(spi, rst, dc)?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}
