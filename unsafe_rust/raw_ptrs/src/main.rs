
//  Run with miri
//  cargo +nightly miri r

fn main() {
    println!("Unsafe Rust");

    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    //print the raw pointers
    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);
    //print the values in unsafe blocks
    unsafe {
        println!("*r1: {:?}", *r1);
        *r2 = 10;
        println!("*r2: {:?}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let len = v.len();
    println!("vec   : {:?}", v);
    let (left, right) = split_at_mut(&mut v, len / 2);
    println!("left  : {:?}", left);
    println!("right : {:?}", right);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
