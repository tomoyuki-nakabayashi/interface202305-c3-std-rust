use esp_idf_hal::{gpio::*, prelude::*, task};
use esp_idf_sys::{tskTaskControlBlock, TaskHandle_t};
use log::*;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    const BUTTON_NOTIFICATION: u32 = 1;

    let task_handle: AtomicPtr<tskTaskControlBlock> = AtomicPtr::new(std::ptr::null_mut());
    let ptr: TaskHandle_t = task::current().expect("never fail.");
    task_handle.store(ptr, Ordering::Relaxed);

    let peripherals = Peripherals::take().expect("never fail");
    let button_pin = peripherals.pins.gpio9;

    let mut button = Box::new(PinDriver::input(button_pin)?);
    button.set_pull(Pull::Down)?;
    button.set_interrupt_type(InterruptType::PosEdge)?;

    unsafe {
        button.subscribe(move || {
            task::notify(task_handle.load(Ordering::Relaxed), BUTTON_NOTIFICATION);
        })?;
    }

    loop {
        let res = task::wait_notification(Some(Duration::from_secs(1)));
        if let Some(BUTTON_NOTIFICATION) = res {
            info!("button pressed");
        }
    }
}
