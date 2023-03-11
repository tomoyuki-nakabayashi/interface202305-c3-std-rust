use bme280::i2c::BME280;
use embedded_hal::i2c::I2c;
use esp_idf_hal::{delay::FreeRtos, gpio::*};
use esp_idf_svc::timer::*;

use anyhow::anyhow;
use mutex_trait::Mutex;
use log::*;
use std::{sync::mpsc, sync::Arc, thread, ops::Deref};
use common::{
    sensor_value::SensorValue,
    mutex,
    no_std_anyhow,
};

/// Sensor type that BME280 can measure.
/// Used to select a type to be posted.
#[derive(PartialEq, Clone, Copy)]
pub(crate) enum SensorType {
    Temperature,
    Humidity,
    Pressure,
}

impl SensorType {
    pub fn next(&self) -> Self {
        match *self {
            Self::Temperature => Self::Humidity,
            Self::Humidity => Self::Pressure,
            Self::Pressure => Self::Temperature,
        }
    }
}

pub fn spawn_client<I2C>(i2c: I2C, button_pin: Gpio9, http_server: &'static str) -> anyhow::Result<()>
where
    I2C: 'static + I2c + Send,
{
    let mut button = Box::new(PinDriver::input(button_pin)?);
    button.set_pull(Pull::Down)?;
    button.set_interrupt_type(InterruptType::PosEdge)?;

    let state = Arc::new(mutex::Mutex::new(SensorType::Temperature));
    let s1 = state.clone();

    unsafe {
        button.subscribe(move || {
            s1.deref().lock(|s| {
                *s = s.next();
            });
        })?;
    }
    Box::leak(button);

    // http
    thread::Builder::new()
        .stack_size(8192)
        .spawn(move || -> anyhow::Result<()> {
            let (tx, rx) = mpsc::channel();
            let mut delay = FreeRtos;

            // initialize the BME280 using the primary I2C address 0x76
            let mut bme280 = BME280::new_primary(i2c);
            no_std_anyhow!(bme280.init(&mut delay))?;

            let timer = EspTaskTimerService::new()?;
            let timer = timer.timer(move || {
                let measurements = no_std_anyhow!(bme280.measure(&mut delay)).unwrap();
                let value = match state.deref().lock(|s| *s) {
                    SensorType::Temperature => SensorValue::Temperature(measurements.temperature),
                    SensorType::Humidity => SensorValue::Humidity(measurements.humidity),
                    SensorType::Pressure => SensorValue::Pressure(measurements.pressure),
                };
                match tx.send(value) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("channel operation failed {}", e)
                    }
                }
            })?;

            timer.every(std::time::Duration::from_secs(5))?;

            loop {
                let v = rx.recv()?;
                info!("{:?}", v);

                let resp = attohttpc::post("http_server")
                    .json(&v)?
                    .send()?;
                info!("Status: {:?}", resp.status());
            }
        })?;

    Ok(())
}
