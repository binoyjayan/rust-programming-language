
fn main() {
    println!("Hello, world!");
}

// fn make_str(n: i32) -> &'static str {
//     let s = format!("hello {}", n);
//     &s
//     // ^^ returns a reference to data owned by the current function
// }

fn make_str(n: i32) -> String {
    format!("hello {}", n)
}


fn part(s: &str) -> &str {
    if s.len() > 4 {
        &s[0..4]
    } else {
        s
    }
}
