use log::*;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");
    debug!("this is a debug message");
    error!("oops! {}", 1);

    Ok(())
}
