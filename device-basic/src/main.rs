fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    Ok(())
}
