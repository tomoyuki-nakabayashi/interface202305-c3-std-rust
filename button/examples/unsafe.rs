use esp_idf_hal::{gpio::*, interrupt::task, prelude::*};
use esp_idf_sys::TaskHandle_t;
use log::*;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    const BUTTON_NOTIFICATION: u32 = 1;

    static mut TASK_HANDLE: TaskHandle_t = std::ptr::null_mut();
    unsafe {
        TASK_HANDLE = task::current().expect("always can get valid current task handle.");
    }

    let peripherals = Peripherals::take().expect("never fail");
    let button_pin = peripherals.pins.gpio9;

    let mut button = Box::new(PinDriver::input(button_pin)?);
    button.set_pull(Pull::Down)?;
    button.set_interrupt_type(InterruptType::PosEdge)?;

    unsafe {
        button.subscribe(move || {
            task::notify(TASK_HANDLE, BUTTON_NOTIFICATION);
        })?;
    }

    loop {
        let res = task::wait_notification(Some(Duration::from_secs(1)));
        if let Some(BUTTON_NOTIFICATION) = res {
            info!("button pressed");
        }
    }
}
