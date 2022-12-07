use std::thread;
#[repr(align(128))]
struct V(i32);
static mut V1: V = V(0);
static mut FLAG: i64 = 0;
fn producer() {
    unsafe {
        for _ in 1..10 {
            while FLAG == 1 {}
            V1 = V(2);
            FLAG = 1;
        }
    }
}
fn consumer() {
    unsafe {
        for _ in 1..10 {
            while FLAG == 0 {}
            let v = V1.0;
            if v != 2 {
                println!("Get! V1.0={}", v)
            }
            V1 = V(0);
            FLAG = 0;
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
