
fn main() {
    println!("Characters and strings");
    // strings_basic();
    // string_formatting();
    number_guessing();
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn strings_basic() {
    // sequence of utf-8 characters
    let ss1: &str = "Hello";          // string slice
    let ss2: &'static str = "Hello";  // static makes the string part of code

    // iterating through a string slice
    for c in ss1.chars() {
        println!("char: {}", c);
    }

    if let Some(first) = ss1.chars().nth(0) {
        println!("First char: {}", first);
    }

    //String
    let mut alphabet = String::new();
    let mut ch = 'a' as u8;
    while ch <= ('z' as u8) {
        alphabet.push(ch as char);
        alphabet.push(',');
        ch += 1;
    }
    println!("alphabet = {}", alphabet);

    // Conversions

    let ss1:&str = &alphabet;

    // Concatenation
    let s1 = alphabet + "END";
    println!("s1 = {}", s1);

    let mut s2 = "Hello World".to_string();
    s2.remove(0);
    s2.push_str("!!!");
    println!("s2 = {}", s2);
    println!("s2 = {}", s2.replace("ello", "Bye"));
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn string_formatting() {
    let name = "John";
    let greeting = format!("Hi, I'm {}, nice to meet you!", name);
    println!("{}", greeting);

    let hello = "Hello";
    let rust = "Rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    // Positional
    let run = "Run";
    let forest = "Forest";
    let rfr = format!("{0} {1} {0}", run, forest);
    println!("{}", rfr);

    // Named arguments
    let info = format!("The name is {last}, {first} {last}.",
                       first = "James", last = "Bond");
    println!("{}", info);

    // Mixed
    let mixed = format!("{1} {} {0} {} {data}",
                                "alpha", "beta", data = "delta");
    println!("{}", mixed);
}

use rand::Rng;
use std::io::{stdin};

#[allow(dead_code)]
#[allow(unused_variables)]
fn number_guessing() {
    let number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Enter guess:");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is too low");
                        } else if guess < number {
                            println!("Your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Correct !!!");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Could not read input. {}", e);
                    }
                }
            },
            Err(_) => continue,
        }

    }
}