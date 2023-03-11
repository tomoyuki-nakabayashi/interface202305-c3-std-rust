fn main() -> anyhow::Result<()> {
    // リンカに正しいオプションを指定するために必要
    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")
}
