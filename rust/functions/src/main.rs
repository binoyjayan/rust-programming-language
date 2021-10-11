
fn main() {
    println!("Functions");
    // functions();
    // methods();
    closures();
    higher_order_functions();
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn functions() {
    let mut x = 123;
    print_value(x);
    inc_value(&mut x);
    print_value(x);
    let a = 10;
    let b = 20;
    let prod = product(a, b);
    println!("{} * {} = {}", a, b, prod);
    print_value(prod);
}

#[allow(dead_code)]
fn print_value(x: i32) {
    println!("value = {}", x);
}

#[allow(dead_code)]
fn inc_value(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

struct Point {
    x: f64,
    y: f64,
}
struct Line {
    start: Point,
    end: Point,
}

//Implement method 'len'
impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn methods() {
    let p1 = Point{x: 3.0, y: 4.0};
    let p2 = Point{x: 5.0, y: 10.0};
    let myline = Line{start: p1, end: p2};
    println!("Length = {}", myline.len());
}

fn say_hello() {println!("Hello"); }

#[allow(dead_code)]
#[allow(unused_variables)]
fn closures() {
    let sh = say_hello;
    sh();

    // Closure definition - use ||
    let plus_one = |x: i32| -> i32 {
        x + 1
    };
    println!("1 plus 1  = {}", plus_one(1));

    // Closures have access to the variables outside it
    let mut two = 2;
    {
        let plus_two = |x: i32| -> i32 {
            // a reference of two is borrowed by the closure
            x + two
        };
        println!("1 plus {}  = {}", two, plus_two(1));
    }
    /* This is possible only because the closure 'plus_two'
     * was inside a scope due to which the it gets destroyed
     * by the time we borrow a mutable reference to two.
     */

    let borrow_two = &mut two;

    // Pass by reference
    let plus_three = |x: &mut i32| {
        *x += 3;
    };
    let mut z = 10;
    plus_three(&mut z);
    println!("10 plus 3  = {}", z);
}

/*
 * functions that take functions
 *   f(g) { let x = g(); }
 * functions that return functions (it is also called generators)
 *   f() -> g
*/
#[allow(dead_code)]
#[allow(unused_variables)]
fn higher_order_functions() {
    let limit = 500;
    let mut sum = 0;

    // this is a simple closure
    // let above_limit = |y| y > limit;

    // generator
    let above_limit = greater_than(limit);

    // infinite range
    for i in 0.. {
        let isq = i * i;
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }
    println!("sum of even squares = {}", sum);


    // Using Rust built-in HOFs
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit )
        .filter(|x: &u32| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("sum of even squares (using HOFs) = {}", sum);
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

// generator returning a function
fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    // move keyword is needed because otherwise
    // limit will get destroyed before above_limit does
    // 'Fn' is used to define the signature of the function
    // that is returned by this higher order function
    move |y| y > limit
}


