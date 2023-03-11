#[macro_export]
macro_rules! no_std_anyhow {
    ($e:expr) => {
        ($e).map_err(|e| anyhow!("{:?}, {}:{}", e, std::file!(), std::line!()))
    };
}
