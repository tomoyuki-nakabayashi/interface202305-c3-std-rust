fn main() {
    esp_idf_sys::link_patches();
    unsafe { esp_idf_sys::my_component::hello_from_c() }
}
