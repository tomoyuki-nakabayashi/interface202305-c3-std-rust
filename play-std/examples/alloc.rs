use log::*;
use std::collections::BTreeMap;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mut xs = Vec::new();
    for i in 0..5 {
        xs.push(i);
    }
    info!("{:?}", xs);

    let s = String::from("Hello!");
    info!("{}", s);

    let mut m = BTreeMap::new();
    m.insert("one", 1);
    m.insert("two", 2);
    m.insert("three", 3);
    info!("{:?}", m);
    info!("{:?}", m.get("one"));

    Ok(())
}
