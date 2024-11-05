mod basket;
mod container;
mod stack;

use container::Container;

// Generic function that can accept any kind of container
// specialized to work with strings
fn add_string<T: Container<String>>(c: &mut T, s: String) {
    c.put(s);
}

// Generic function that can accept any kind of container
// specialized to work with numbers
fn add_num<T: Container<usize>>(c: &mut T, s: usize) {
    c.put(s);
}

fn main() {
    let mut b1 = basket::Basket::new("Apple".to_string());
    let mut b2 = basket::Basket::new(1);

    let mut s1 = stack::Stack::new(vec!["Apple".to_string()]);
    let mut s2 = stack::Stack::new(vec![1]);

    add_string(&mut b1, "Orange".to_string());
    add_string(&mut s1, "Orange".to_string());

    add_num(&mut b2, 2);
    add_num(&mut s2, 2);

    println!("b1: {:#?}", b1);
    println!("b2: {:#?}", b2);
    println!("s1: {:#?}", s1);
    println!("s2: {:#?}", s2);
}
