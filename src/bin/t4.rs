use std::sync::Arc;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering;
use std::thread;

#[repr(C)]
struct Context{
    t1: AtomicI64,
    padding1: [i64; 256],
    t2: AtomicI64,
    padding2: [i64; 256],
    z:  AtomicI64,
    padding3: [i64; 256],
    flag: AtomicI64
}
impl Context {
    fn a(& self) {
        while self.flag.load(Ordering::Acquire) == 0{}
        self.t1.store(1, Ordering::Release);
    }
    fn b(& self){
        while self.flag.load(Ordering::Acquire) == 0{}
        self.t2.store(1, Ordering::Release);
    }
    fn c(& self){
        while self.flag.load(Ordering::Acquire) == 0{}
        while self.t1.load(Ordering::Acquire) == 0{}
        if self.t2.load(Ordering::Acquire) == 1 {
            self.z.fetch_add(1, Ordering::AcqRel);
        }
    }
    fn d(& self){
        while self.flag.load(Ordering::Acquire) == 0{}
        while self.t2.load(Ordering::Acquire) == 0{}
        if self.t1.load(Ordering::Acquire) == 1 {
            self.z.fetch_add(1, Ordering::AcqRel);
        }
    }
}
fn run(i:i32){
    let context_arc = Arc::<Context>::new(Context { t1: AtomicI64::new(0), padding1: [0;256], t2:  AtomicI64::new(0), padding2: [0;256], z: AtomicI64::new(0), padding3:[0;256], flag:AtomicI64::new(0)});
    let c1 = context_arc.clone();
    let c2 = context_arc.clone();
    let c3 = context_arc.clone();
    let c4 = context_arc.clone();

    let t1 = thread::spawn(move || c1.a());
    let t2 = thread::spawn(move || c2.b());
    let t3 = thread::spawn(move || c3.c());
    let t4 = thread::spawn(move || c4.d());
    
    context_arc.flag.store(1, Ordering::Release);

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    t4.join().unwrap();
    let v = context_arc.z.load(Ordering::Acquire);
    if v == 0{
        println!("i={}, v={}", i, v);
    }
}
fn main(){
    for i in 0..100000000 {
        run(i);
    }
}