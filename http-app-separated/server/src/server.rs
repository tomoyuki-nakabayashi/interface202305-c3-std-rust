use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use embedded_hal_0_2::{blocking::spi, digital::v2::OutputPin};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_sys::EspError;
use ssd1331::{DisplayRotation, Ssd1331};

use log::*;
use std::io::Read;
use std::marker::Send;
use std::sync::mpsc;
use std::thread;

use anyhow::anyhow;
use embedded_svc::{http::server::*, io::Write};
use esp_idf_svc::http::server::*;
use common::{
    no_std_anyhow,
    sensor_value::*,
};

pub fn spawn_server<SPI, RST, DC>(spi: SPI, mut rst: RST, dc: DC) -> anyhow::Result<()>
where
    SPI: 'static + spi::Write<u8> + Send,
    <SPI as spi::Write<u8>>::Error: std::fmt::Debug,
    DC: 'static + OutputPin<Error = EspError> + Send,
    RST: 'static + OutputPin<Error = EspError> + Send,
{
    let (tx, rx) = mpsc::channel();

    let server_config = Configuration::default();
    let mut server = Box::new(EspHttpServer::new(&server_config)?);
    server
        .fn_handler("/", Method::Get, |req| {
            let html = index_html();
            req.into_ok_response()?.write_all(&html.as_bytes())?;

            Ok(())
        })?
        .fn_handler("/sensor", Method::Post, move |req| {
            use embedded_svc::io::adapters::ToStd;
            let mut body = String::new();
            // req.read_to_string(&mut body)?;
            ToStd::new(req).read_to_string(&mut body)?;

            let v: SensorValue = serde_json::from_str(&body)?;

            info!("posted value: {:?}", v);
            tx.send(v)?;

            Ok(())
        })?;
    Box::leak(server);

    thread::Builder::new()
        .stack_size(24000)
        .spawn(move || -> anyhow::Result<()> {
            let mut delay = FreeRtos;
            let mut disp = Ssd1331::new(spi, dc, DisplayRotation::Rotate0);
            no_std_anyhow!(disp.reset(&mut rst, &mut delay))?;
            no_std_anyhow!(disp.init())?;

            disp.clear();
            no_std_anyhow!(disp.flush())?;

            loop {
                let v = rx.recv()?;
                disp.clear();
                no_std_anyhow!(disp.flush())?;
                let text = v.to_string();
                let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
                no_std_anyhow!(Text::new(&text, Point::new(0, 30), style).draw(&mut disp))?;
                no_std_anyhow!(disp.flush())?;
            }
        })?;

    Ok(())
}

fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}

fn index_html() -> String {
    templated("Please post sensor value to /sensor")
}
