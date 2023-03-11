use log::*;
use std::io::prelude::*;
use std::net::TcpStream;
use esp_idf_hal::peripherals::Peripherals;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
    #[default("")]
    http_server: &'static str,
}

mod wifi;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("never fail");

    let _wifi = wifi::connect(peripherals.modem, CONFIG.wifi_ssid, CONFIG.wifi_psk)?;

    // HTTP サーバー に TCP 接続する
    let mut stream = TcpStream::connect(CONFIG.http_server)?;
    // HTTP リクエストを送信する
    let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", CONFIG.http_server);
    stream.write(request.as_bytes())?;

    // HTTP レスポンスを受信する
    let mut buf: [u8; 1024] = [0; 1024];
    stream.read(&mut buf)?;

    info!("response:");
    print!("{}", std::str::from_utf8(&buf)?);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}
