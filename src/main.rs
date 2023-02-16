use std::mem::transmute;

mod utils;

fn main() {
    let mut f = 0.45f64;
    let mut u = unsafe {
        transmute::<f64, u64>(f)
    };
    u = 0;
    println!("{} = {}", f, u);
}