//! # Hello C3!
//!
//! This `libstd` program is for the ESP32-C3 boards such as M5Stamp C3U Mate or DevKitC.

fn main() {
    esp_idf_sys::link_patches();
    println!("Hello, world!");
}
