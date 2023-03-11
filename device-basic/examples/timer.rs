use anyhow::anyhow;
use log::*;

use bme280::i2c::BME280;
use esp_idf_hal::{delay::FreeRtos, i2c, prelude::*};
use esp_idf_svc::timer::EspTaskTimerService;
use std::{thread, time::Duration};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("never fail");

    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio0;
    let scl = peripherals.pins.gpio1;

    let config = i2c::I2cConfig::new().baudrate(100.kHz().into());
    let i2c = i2c::I2cDriver::new(i2c, sda, scl, &config)?;

    let mut delay = FreeRtos;
    // initialize the BME280 using the primary I2C address 0x76
    let mut bme280 = BME280::new_primary(i2c);
    bme280
        .init(&mut delay)
        .map_err(|e| anyhow!("{:?}, {}:{}", e, std::file!(), std::line!()))?;

    let timer_svc = EspTaskTimerService::new()?;
    let timer = Box::new(timer_svc.timer(move || {
        let m = bme280.measure(&mut delay).unwrap();
        info!(
            "temperature: {}, humidity: {}, pressure: {}",
            m.temperature, m.humidity, m.pressure
        );
    })?);

    timer.every(Duration::from_secs(1))?;
    Box::leak(timer);

    thread::sleep(Duration::from_secs(10));

    Ok(())
}
