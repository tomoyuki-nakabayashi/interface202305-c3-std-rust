use log::*;
use std::{thread, time::Duration};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    spawn_threads();
    threads_return_result();
    thread_builder()?;
    change_thread_priority();
    thread_can_borrow_static_lifetime();

    loop {}
}

fn print_freertos_tasks() {
    let mut buf = [0u8; 1024];
    unsafe { esp_idf_sys::vTaskList(buf.as_mut_ptr() as *mut i8) };
    // stack hwm: stack high water mark
    // The minimum amount of stack space that has remained for the task since
    // the task was created.  The closer this value is to zero the closer the task
    // has come to overflowing its stack.
    info!("tasks:\n
name          state  priority stack hwm id\n
{}",
        String::from_utf8_lossy(&buf)
    );
}

fn print_heap_free_size() {
    unsafe {
        info!(
            "heap: {}",
            esp_idf_sys::heap_caps_get_free_size(esp_idf_sys::MALLOC_CAP_INTERNAL)
        );
    }
}

fn spawn_threads() {
    info!("main thread: {:?}", thread::current());
    print_freertos_tasks();

    let mut threads = Vec::new();
    for i in 0..5 {
        let t = thread::spawn(move || {
            info!("child {} thread: {:?}", i, thread::current());
            thread::sleep(Duration::from_secs(1));
        });
        threads.push(t);
    }

    print_freertos_tasks();

    for t in threads {
        t.join().expect("thread failed");
    }
}

fn threads_return_result() {
    let mut threads = Vec::new();
    for i in 0..5 {
        let t = thread::spawn(move || -> anyhow::Result<u32> {
            if i == 4 {
                return Err(anyhow::anyhow!("thread 4 returns error!"));
            }
            Ok(i)
        });
        threads.push(t);
    }

    for t in threads {
        let ret = t.join().expect("thread failed");
        info!("thread returns: {:?}", ret);
    }
}

fn thread_builder() -> anyhow::Result<()> {
    print_heap_free_size();

    let t = std::thread::Builder::new()
        .name("named thread".to_string())
        .stack_size(8092)
        .spawn(|| {
            info!("thread: {:?}", thread::current());
            thread::sleep(Duration::from_secs(1));
        })?;

    print_heap_free_size();
    print_freertos_tasks();

    t.join().expect("thread railed");
    Ok(())
}

fn change_thread_priority() {
    let mut threads = Vec::new();
    for i in 0..5 {
        let t = thread::spawn(move || {
            unsafe {
                esp_idf_sys::vTaskPrioritySet(std::ptr::null_mut(), 5 + i);
            }
            thread::sleep(Duration::from_secs(1));
        });
        threads.push(t);
    }

    print_freertos_tasks();

    for t in threads {
        t.join().expect("thread failed");
    }
}

fn thread_can_borrow_static_lifetime() {
    static I: usize = 1;
    let t = thread::spawn(|| {
        info!("{}", I);
    });
    t.join().expect("thread railed");

    let greeting = "hello";
    let mut threads = Vec::new();
    for _ in 0..5 {
        let t = thread::spawn(move || {
            info!("{}", greeting);
        });
        threads.push(t);
    }
    for t in threads {
        t.join().expect("thread failed");
    }
}
