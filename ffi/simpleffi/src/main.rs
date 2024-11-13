
use std::ffi::c_int;

extern "C" {
    fn sum(a: c_int, b: c_int) -> i32;
}

pub fn bind_sum(a: i32, b: i32) -> i32 {
    unsafe {        
        sum(a, b)
    }
}

fn main() {
    let s = bind_sum(10, 20);
    println!("sum(10, 20) = {}", s);
}