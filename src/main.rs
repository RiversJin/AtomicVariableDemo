use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, JoinHandle};


const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;
static mut R_MUT: usize = 4;
static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            unsafe{
                R_MUT += 1;
            }
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

fn main() {
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    let r1 = unsafe {
        R_MUT
    };
    println!("R:{:?} r:{:?}",R, r1);
}