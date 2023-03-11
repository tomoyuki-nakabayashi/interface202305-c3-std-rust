use log::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    atomic();
    share_with_arc();
    mutable_share_with_mutex();
    channel_example()?;

    loop {}
}

fn atomic() {
    static THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

    let mut threads = Vec::new();
    for _ in 0..5 {
        let t = thread::spawn(move || {
            let old_thread_count = THREAD_COUNT.fetch_add(1, Ordering::Relaxed);
            info!("created threads: {}", old_thread_count + 1);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Data {
    a: usize,
    b: usize,
    c: usize,
}

fn share_with_arc() {
    let data = Arc::new(Data { a: 0, b: 1, c: 2 });

    let mut threads = Vec::new();
    for _ in 0..5 {
        let tdata = data.clone();
        let t = thread::spawn(move || {
            info!("{:?}", tdata);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }
}

fn mutable_share_with_mutex() {
    let data = Arc::new(Mutex::new(Data { a: 0, b: 1, c: 2 }));

    let mut threads = Vec::new();
    for _ in 0..5 {
        let tdata = data.clone();
        let t = thread::spawn(move || -> anyhow::Result<()> {
            let mut locked = tdata.lock().expect("never failed");
            locked.a += 1;
            info!("{:?}", locked);

            Ok(())
        });
        threads.push(t);
    }

    for t in threads {
        let res = t.join().expect("thread failed");
        res.expect("mutex operation failed");
    }
}

fn channel_example() -> anyhow::Result<()> {
    // tx は Sender<i32> という型で、i32 の値を Receiver<i32> に送信する
    // rx は Receiver<i32> という型で、i32 の値を Sender<i32> から受信する
    let (tx, rx) = mpsc::channel();

    let mut threads = Vec::new();
    for i in 0..5 {
        // Sender は clone で増やすことができる
        let tx = tx.clone();
        let t = thread::spawn(move || -> anyhow::Result<()> {
            // 値を送信する
            tx.send(i)?;
            Ok(())
        });
        threads.push(t);
    }

    for _ in 0..5 {
        // 値を受信する
        info!("got {}", rx.recv()?);
    }

    for t in threads {
        let res = t.join().expect("thread failed");
        res.expect("channel send failed");
    }

    Ok(())
}
