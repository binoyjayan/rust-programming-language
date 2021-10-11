
trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("String: {}", *self)
    }
}

// momomorphisation - a static dispatch approach
fn print<T: Printable>(z: T) {
    println!("{}", z.format());
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn static_dispatch() {
    let a = 10;
    let s = "hello".to_string();

    print(a); // Similar to println!("{}", a.format());
    print(s); // Similar to println!("{}", s.format());
}

/*
 * --------------------------------------------------
 * Dynamic dispatch
 * --------------------------------------------------
 */

// dynamic dispatch. Compiler does not infer the function called
fn print_it(z: &Printable) {
    println!("{}", z.format());
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn dynamic_dispatch_simple() {
    let a = 10;
    let s = "hello".to_string();

    print_it(&a);
    print_it(&s);
}

struct Square { side: f64 }
struct Circle { radius: f64 }

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// An example where dynamic dispatch maybe necessary
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn dynamic_dispatch() {
    let shapes : [&Shape; 4] = [
        &Circle { radius: 2.0},
        &Circle { radius: 2.0},
        &Square { side: 3.0},
        &Square { side: 4.0}
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape[{}] has area {}", i, shape.area());
    }
}

