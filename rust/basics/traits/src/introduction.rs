
trait Animal {
    fn name(&self) -> &'static str;
    fn new(name: &'static str) -> Self;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}


impl Animal for Human {
    fn name(&self) ->&'static str {
        self.name
    }
    fn new(name: &'static str) -> Self {
        Human{name}
    }
    fn talk(&self) {
        println!("{} says Hello", self.name());
    }
}

impl Animal for Cat {
    fn name(&self) ->&'static str {
        self.name
    }
    fn new(name: &'static str) -> Self {
        Cat{name }
    }
    fn talk(&self) {
        println!("{} says Meow", self.name());
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32  = 0;
        for x in self {
            result += *x;
        }
        result
    }
}

// Automagically implement debug trait for cirlce
#[derive(Debug)]
struct Circle {
    radius : f64
}

// Automagically implement debug trait for square
#[derive(Debug)]
struct Square {
    side : f64
}

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

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn traits_basic() {
    let j = Human::new("John");
    j.talk();

    // Create object, interrring the object type
    let m: Human = Animal::new("Mike");
    m.talk();

    let c = Cat::new("Missy");
    c.talk();

    let a = vec![1, 2, 3, 4, 5];
    println!("sun = {}", a.sum());
}

/* First approach to pass traits as parameter
 * fn print_info(shape: impl Shape + std::fmt::Debug)
 *
 * Another concise way - trait bound syntax (more concise)
 * fn print_info<T:Shape + std::fmt::Debug>(shape: T) {
 *
 * Third approach to pass traits as parameter - WHERE clause
*/
fn print_info<T>(shape: T) where T: Shape + std::fmt::Debug {
    println!("Shape {:?}", shape);
    println!("The area is {}", shape.area());
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn traits_parameters() {
    let c = Circle{radius: 2.0};
    print_info(c);
}

struct Person {
    name: String,
}

impl Person {
    // A regular new function
    // fn new(name: &str) -> Self {
    //     Person { name: name.to_string() }
    // }

    // One approach to specify a generic type with an into trait
    // that the generic type has to implement
    // fn new<S: Into <String>>(name: S) -> Self {

    // Another way using a where clause
    fn new<S>(name: S) -> Self where S: Into <String> {
        Person { name: name.into() }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn traits_into() {
    let _john = Person::new("John");
    let jname = "Jane".to_string();
    // Explicit conversion from String to &str
    // let _jane = Person::new(jname.as_ref());

    // Implicit conversion - uses the into trait
    let _jane = Person::new(jname);
}


struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Self {
        println!("{} enters game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    // Destructor
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn traits_drop() {
    let goblin = Creature::new("Jeff");
    println!("game proceeds");
    // Explicit call to destructor
    drop(goblin);
    println!("game ends");
}
