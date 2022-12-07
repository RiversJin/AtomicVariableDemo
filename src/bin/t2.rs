use std::{thread, sync::atomic::AtomicI64};
use std::sync::atomic::Ordering;
#[repr(align(128))]
struct V(i32);
static mut V1: V = V(0);
static FLAG:AtomicI64 = AtomicI64::new(0);
fn producer() {
    unsafe {
        for _ in 1..10 {
            while FLAG.load(Ordering::Relaxed) == 1 {}
            V1 = V(2);
            FLAG.store(1, Ordering::Relaxed);
        }
    }
}
fn consumer() {
    unsafe {
        for _ in 1..10 {
            while FLAG.load(Ordering::Relaxed) == 0 {}
            let v = V1.0;
            if v != 2 {
                println!("Get! V1.0={}", v)
            }
            V1 = V(0);
            FLAG.store(0, Ordering::Relaxed);
        }
    }
}

fn main() {
    for _ in 0..1000000 {
        let p = thread::spawn(|| producer());
        let c = thread::spawn(|| consumer());
        p.join().unwrap();
        c.join().unwrap();
    }
}
