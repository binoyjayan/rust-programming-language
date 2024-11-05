use num_traits::{Float, ToPrimitive};

// First version of solve function: Can pass both f32 or both f64
fn solve1<T: Float>(a: T, b: T) -> f64 {
    let a = a.to_f64().unwrap();
    let b = b.to_f64().unwrap();
    (a.powi(2) + b.powi(2)).sqrt()
}

// Second version of solve function: Can pass different float types
// We need two type parameters here because we need to pass two
// different types of floats
fn solve2<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a = a.to_f64().unwrap();
    let b = b.to_f64().unwrap();
    (a.powi(2) + b.powi(2)).sqrt()
}

// Second version of solve function: Can pass any type of number
fn solve3<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a = a.to_f64().unwrap();
    let b = b.to_f64().unwrap();
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0;
    let b: f32 = 4.0;
    println!("{}", solve1(a, b));

    let a: f32 = 3.0;
    let b: f64 = 4.0;
    println!("{}", solve2(a, b));

    let a: i32 = 3;
    let b: f32 = 4.0;
    println!("{}", solve3(a, b));
}
