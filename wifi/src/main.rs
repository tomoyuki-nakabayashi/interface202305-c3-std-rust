use esp_idf_hal::peripherals::Peripherals;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

mod wifi;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("never fail");

    let _wifi = wifi::connect(peripherals.modem, CONFIG.wifi_ssid, CONFIG.wifi_psk)?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}
