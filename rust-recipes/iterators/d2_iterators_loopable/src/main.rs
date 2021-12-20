
fn main () {
    print_a(&vec!["hello".to_string(), "world".to_string()]);
    print_b(&["hello", "world"]);

    print_c((vec!["hello".to_string(), "world".to_string()]).into_iter());

    print_any((vec!["hello".to_string(), "world".to_string()]).into_iter());
    print_any((vec!["hello", "mars"]).into_iter());
    print_any((&["hello", "venus"]).into_iter());
    print_any((["hello", "jupiter"]).into_iter());

}


// Function for handling String's
fn print_a(v: &Vec<String>) {
    println!("a");

    for (i, val) in v.into_iter().enumerate() {
        println!("      {} = {}", i, val);
    }
}

// Duplicate function for slice
fn print_b(v: &[&str]) {
    println!("b");

    for (i, val) in v.into_iter().enumerate() {
        println!("      {} = {}", i, val);
    }
}

// Generic but does not accept slices
fn print_c<I: Iterator<Item = String>>(v: I) {
    println!("c");

    // Do not need into_iter() here since print_c already passes an into_iter()
    for (i, val) in v.enumerate() {
        println!("      {} = {}", i, val);
    }
}

// Truly generic over any type
// Every Iterator automatically implements IntoIterator
// Item is anything that can be treated as a reference to a String.
fn print_any<S: AsRef<str>, I: IntoIterator<Item = S>>(v: I) {
    println!("any");

    for (i, val) in v.into_iter().enumerate() {
        println!("      {} = {}", i, val.as_ref());
    }
}

