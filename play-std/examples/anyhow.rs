use log::*;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let num = u8::from_str_radix("A5", 10)?;
    let mut buf: [u8; 64] = [0u8; 64];
    {
        let mut buffer = std::io::BufWriter::new(&mut buf[..]);
        buffer.write(&[num])?;
    }
    info!("buf[0] = {}", buf[0]);

    Ok(())
}
