use std::time::{SystemTime, UNIX_EPOCH};

static mut SEED: u32 = 0;

unsafe fn rand_global(start: u32, end: u32) -> u32 {
    if SEED == 0 {
        let epoc = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap();
        SEED = epoc.as_millis() as u32;
    }
    println!("{}", SEED);
    SEED ^= SEED << 13;
    SEED ^= SEED << 17;
    SEED ^= SEED << 5;
    return SEED % (end - start + 1) + start;
}

fn main() {
    for i in 0..100 {
        unsafe {
            if i % 5 == 0 { println!(""); }
            let v = rand_global(1,6);
            print!("{:3}", v);
        }
    }
}