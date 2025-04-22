use std::ffi::c_int;

// First way to refer to external functions
// This function cannot be called from safe code
extern "C" {
    fn sum(a: c_int, b: c_int) -> i32;
}
// This function can be called from safe code
pub fn bind_sum(a: i32, b: i32) -> i32 {
    unsafe { sum(a, b) }
}

// Second method to refer to external functions
unsafe extern "C" {
    // declare as safe so that it can be called from safe code
    safe fn diff(a: c_int, b: c_int) -> i32;
}

fn main() {
    let s = bind_sum(10, 20);
    println!("sum(10, 20)  = {}", s);

    let d = diff(20, 10);
    println!("diff(20, 10) = {}", d);
}
