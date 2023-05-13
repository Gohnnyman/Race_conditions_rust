use std::{
    println,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
    thread, time,
};

#[derive(Clone)]
struct RawPtr(*mut u32);
impl RawPtr {
    unsafe fn add(&self, value: u32) {
        *self.0 += value;
    }
}
unsafe impl Send for RawPtr {}
unsafe impl Sync for RawPtr {}

const REPEATS: usize = 10000;

pub fn run() {
    let mut time = time::Instant::now();
    without_sync();
    println!(
        "Time elapsed for without sync: {}ms\n",
        time.elapsed().as_millis()
    );

    time = time::Instant::now();
    with_mutex();
    println!(
        "Time elapsed for with mutex: {}ms\n",
        time.elapsed().as_millis()
    );

    time = time::Instant::now();
    with_atomic();
    println!(
        "Time elapsed for with atomic: {}ms\n",
        time.elapsed().as_millis()
    );
}

fn without_sync() {
    let mut v = 0u32;
    let ptr1 = RawPtr(&mut v as *mut u32);
    let ptr2 = ptr1.clone();

    let t1 = thread::spawn(move || unsafe {
        for _ in 0..REPEATS {
            ptr1.add(1);
        }
    });

    let t2 = thread::spawn(move || unsafe {
        for _ in 0..REPEATS {
            ptr2.add(1);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Without sync:");
    println!("\tv = {}", v);
}

fn with_mutex() {
    let v1 = Arc::new(Mutex::new(0u32));
    let v2 = v1.clone();
    let v = v1.clone();

    let t1 = thread::spawn(move || {
        for _ in 0..REPEATS {
            *v1.lock().unwrap() += 1
        }
    });

    let t2 = thread::spawn(move || {
        for _ in 0..REPEATS {
            *v2.lock().unwrap() += 1
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("With mutex:");
    println!("\tv = {}", v.lock().unwrap());
}

fn with_atomic() {
    let v1 = Arc::new(AtomicU32::new(0u32));
    let v2 = v1.clone();
    let v = v1.clone();

    let t1 = thread::spawn(move || {
        for _ in 0..REPEATS {
            v1.fetch_add(1, Ordering::Relaxed);
        }
    });

    let t2 = thread::spawn(move || {
        for _ in 0..REPEATS {
            v2.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("With atomic:");
    println!("\tv = {}", v.load(Ordering::Relaxed));
}
