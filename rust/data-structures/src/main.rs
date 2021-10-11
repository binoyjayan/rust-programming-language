use crate::Color::Cymk;

fn main() {
    println!("Data Structures");
    // structures();
    // enums();
    // unions();
    // option_type();
    // arrays();
    // slices();
    // tuples();
    // pattern_matching();
    generics();
}

#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Line {
    start: Point,
    end: Point,
}

#[allow(dead_code)]
fn structures() {
    let p1 = Point{x:3.0, y:4.0};
    let p2 = Point{x:5.0, y:10.0};
    println!("point p1 = ({}, {})", p1.x, p1.y);
    println!("point p2 = ({}, {})", p2.x, p2.y);

    // unused variable
    let _line1 = Line {start: p1, end: p2};
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8,u8,u8), // tuple
    Cymk{cyan: u8, magenta:u8, yellow:u8, black:u8}, // struct
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn enums() {
    let c: Color = Color::Red;
    let b: Color = Color::Rgb(0,0,0);
    let w: Color = Color::Rgb(255,255,255);
    let gr: Color = Color::Rgb(128,128,128);
    let y: Color = Cymk {cyan: 0, magenta:0, yellow:128, black:0};
    match y {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::Rgb(0,0,0)
        | Color::Cymk {cyan:_, magenta:_, yellow:_, black:255} => println!("Black"),
        Color::Rgb(255,255,255) => println!("White"),
        Color::Rgb(r,g,b) => println!("RGB({}, {}, {})", r, g, b),
        Color::Cymk {cyan:255, .. } => println!("Has Cyan(255) component with others"),
        Color::Cymk{cyan, magenta, yellow, black} =>
            println!("CYMK({}, {}, {}, {})", cyan, magenta, yellow, black),
    }
}

#[allow(dead_code)]
union IntOrFloat {
    i: i32,
    f: f32,
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn unions() {
    let mut iof = IntOrFloat {i: 123};
    iof.i = 234;

    // Needs an unsafe block since we do not know what value
    // is present in the memory block
    let val = unsafe {iof.i };
    println!("iof.i: {}", val);
    process_value(IntOrFloat{i: 42});
    process_value(IntOrFloat{f: 4.2});
    process_value(IntOrFloat{i: 4}); // bad
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("i = 42");
            },
            IntOrFloat {f } => {
                println!("f = {}", f);
            },
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn option_type() {
    let x = 3.0;
    let y = 2.0;

    // Option types: Some(v) | None
    let result = if y != 0.0 {Some(x/y)} else { None };

    match result {
        Some(z) => println!("{} / {} = {}", x, y, z),
        None => println!("Cannot divide by zero"),
    }

    // Special rust construct
    if let Some(z) = result {
        println!("{} / {} = {}", x, y, z);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn arrays() {
    // declaration
    let mut a:[i32;5] = [1,2,3,4,5];
    let b = [5,4,3,2,1];

    println!("a has {} elements, first: {}", a.len(), a[0]);
    println!("b has {} elements, first: {}", b.len(), b[0]);
    a[0] = 11;
    println!("first element modified: {}", a[0]);
    println!("{:?}", a);

    if a != [1,2,3,4,5] {
        println!("array does not match [1,2,3,4,5]");
    }

    // Anaother way of initializing arrays
    let c = [222; 10]; // 10 elements of 222

    for i in 0..c.len() {
        println!("c[{}] = {}", i, c[i]);
    }
    println!("c took {} bytes", std::mem::size_of_val(&c));

    // Matrix
    let mtx:[[f32;3]; 2] = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
    ];
    println!("{:?}", mtx);

    // traversing
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn slices() {
    let mut data = [1,2,3,4,5];
    use_slice(&data[1..4]);
    modify_slice(&mut data); // use whole array
}

fn use_slice(slice: &[i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
}

fn modify_slice(slice: &mut[i32]) {
    slice[0] = 111;
    println!("{:?}", slice);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("x: {}, y: {}, sum = {}, product = {}", x, y, sp.0, sp.1);

    // Destructuring
    let (sum, product) = sp;
    println!("x: {}, y: {}, sum = {}, product = {}", x, y, sum, product);

    // tuple of tuples
    let sp2 = sum_and_product(10,20);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last: {}", (combined.1).1);
    // destructuring
    let ((c,d), (e,f)) = combined;

    // single element tuple
    let single = (1,);
    println!("{:?}", single);

}

fn sum_and_product(x: i32, y: i32) -> (i32, i32){
    return (x + y, x * y)
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges ", x, how_many(x));
    }
    let point = (3,4);
    match point {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {}", y),
        (x,0) => println!("y axis, x = {}", x),
        (_,y) => println!("(?, {})", y),
    }
}

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        // inclusive range
        _z@9..=12 => "lots of",
        _ if (x % 2 == 0) => "an even number of",
        _ => "a few",
    }
}

#[warn(dead_code)]
#[allow(unused_variables)]
struct GenericPoint<T> {
    _x: T,
    _y: T,
}

#[allow(dead_code)]
#[allow(unused_variables)]
struct GenericLine<T> {
    start: GenericPoint<T>,
    end: GenericPoint<T>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn generics() {
    let a: GenericPoint<i32>= GenericPoint{_x:0, _y:0};
    let b: GenericPoint<i32>= GenericPoint{_x:1, _y:1};
    let _c: GenericPoint<f64>= GenericPoint{_x:0.0, _y:0.0};

    let _myline: GenericLine<i32> = GenericLine{start: a, end: b};
}



