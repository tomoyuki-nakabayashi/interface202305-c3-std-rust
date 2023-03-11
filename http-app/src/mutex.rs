use esp_idf_sys::*;
use std::cell::UnsafeCell;

/// Mutex implementation can be also called from within an ISR.
#[derive(Debug)]
pub struct Mutex<T> {
    inner: SemaphoreHandle_t,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> {}
unsafe impl<T> Send for Mutex<T> {}

fn in_isr_context() -> bool {
    unsafe { xPortInIsrContext() != 0 }
}

impl<T> Mutex<T> {
    /// Create a new mutex
    pub fn new(data: T) -> Mutex<T> {
        let inner = unsafe { esp_idf_sys::xQueueCreateMutex(1) };
        if inner.is_null() {
            panic!("fail to create mutex");
        }
        Self {
            inner,
            data: data.into(),
        }
    }
}

impl<T> Drop for Mutex<T> {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                vQueueDelete(self.inner);
            }
        }
    }
}

impl<'a, T> mutex_trait::Mutex for &'a Mutex<T> {
    type Data = T;
    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        if in_isr_context() {
            unsafe {
                xQueueReceiveFromISR(self.inner, std::ptr::null_mut(), std::ptr::null_mut());
            }
            let result = f(unsafe { self.data.get().as_mut().unwrap() });
            unsafe {
                xQueueGiveFromISR(self.inner, std::ptr::null_mut());
            }
            result
        } else {
            unsafe {
                xQueueSemaphoreTake(self.inner, 0);
            }
            let result = f(unsafe { self.data.get().as_mut().unwrap() });
            unsafe {
                xQueueGenericSend(self.inner, std::ptr::null_mut(), 0, 0);
            }
            result
        }
    }
}
