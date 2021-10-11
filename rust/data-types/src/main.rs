#[allow(unused_variables)]

use std::mem;

fn main() {
    println!("Data types and variables");
    // data_types();
    // operators();
    // scope_shadowing();
    // constants();
    stack_and_heap();
}


#[allow(dead_code)]
fn data_types() {
    // u:unsigned, 8 bits (immutable)
    let a:u8 = 245;
    println!("a = {}", a);

    // mutable
    let mut b:i8 = 123;
    println!("b = {}", b);
    b = 124;
    println!("b = {}", b);

    // auto type
    let mut c = 123456789;
    println!("c = {}, takes {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    //usize,isize
    let d:isize = -456; // 8 bytes signed on 64-bit CPU,OS
    let sz = mem::size_of_val(&d); // sizes are usize by default
    println!("d = {}, takes {} bytes", d, mem::size_of_val(&d));
    println!("sz = {}, takes {} bytes", sz, mem::size_of_val(&sz));

    // characters
    let e: char = 'x';
    println!("e = {}, takes {} bytes", e, mem::size_of_val(&e));

    // floating point (IEEE754)
    let f: f32 = 2.5;
    let g = 2.5; // default is double
    println!("f = {}, takes {} bytes", f, mem::size_of_val(&f));
    println!("g = {}, takes {} bytes", g, mem::size_of_val(&g));

    // boolean
    let h: bool = true;
    println!("h = {}, takes {} bytes", h, mem::size_of_val(&h));

}

#[allow(dead_code)]
fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4;
    let mut b = 4;
    println!("a = {}", a);
    a = a + 1;
    b = b - 1;
    println!("b = {}", a);
    let r = a % b;
    println!("rem = {}", r);
    let b_cubed = i32::pow(b, 3);
    println!("{} cubed is {}", b, b_cubed);
    let c = 2.5;
    let c_cubed = f64::powi(c, 3); // power is integral
    println!("{} cubed is {}", c, c_cubed);
    let b_to_pi = f64::powf(c, std::f64::consts::PI);
    println!("{} ^ PI ({}) is {}", c, std::f64::consts::PI, b_to_pi);

    //bitwise
    let op_or = 1 | 2;
    println!("1|2 = {}", op_or);
    let op_sh = 1 << 10;
    println!("1<<10= {}", op_sh);
}

#[allow(dead_code)]
fn scope_shadowing() {
    let a = 123;
    println!("outside a = {}", a);
    {
        let a = 456;
        println!("inside a = {}", a);
    }
    println!("outside a = {}", a);
    let a = 789;
    println!("shadowing a = {}", a);
}

const MY_PI:f64 = 3.14;           // no fixed address
static MY_PI_STATIC:f64 = 3.14;   // static global (has an address)
static mut MY_PI_MUT:f64 = 3.14;  // static global (has an address)

#[allow(dead_code)]
fn constants() {
    println!("MY PI(const) = {}", MY_PI);
    println!("MY PI(static) = {}", MY_PI_STATIC);
    unsafe {
        // Need an unsafe block to access global mutable addresses
        println!("MY PI(mut) = {}", MY_PI_MUT);
        MY_PI_MUT = 3.14159;
        println!("MY PI(mut) = {}", MY_PI_MUT);
    }
}

#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}
#[allow(dead_code)]
fn origin() -> Point {
    Point{x: 0.0, y:0.0}
}

#[allow(dead_code)]
fn stack_and_heap() {
    let x = Box::new(5);
    println!("*x = {}", *x);

    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes {} bytes", mem::size_of_val(&p1));
    println!("p2 takes {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("p3 takes {} bytes", mem::size_of_val(&p3));
    println!("p3 = ({}, {})", p3.x, p3.y);
}

