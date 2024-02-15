

fn main() {
    println!("Control flow");
    // single_if();
    // if_else();
    // if_expression();
    // nested();
    // while_and_loop();
    // for_loop();
    // match_statement();
    combination_lock();
}

#[allow(dead_code)]
fn single_if() {
    let temp = 35;
    if temp > 30 {
        println!("Really hot");
    }
}

#[allow(dead_code)]
fn if_else() {
    let temp = 35;
    if temp > 30 {
        println!("Today is really hot");
    } else if temp < 10 {
        println!("Today is really cold");
    } else {
        println!("Today, the temperature is OK");
    }
}

#[allow(dead_code)]
fn if_expression() {
    let temp = 35;
    let day = if temp > 20 {"Sunny"} else {"Cloudy"};
    println!("Today is {}", day);

    let weather =if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"};
    println!("It is {}", weather);
}

#[allow(dead_code)]
fn nested() {
    let temp = 35;
    let weather = if temp > 20 {
        if temp > 30 { "very hot" } else { "hot" }
    } else if temp < 20 {
        if temp < 10 { "very cold" } else { "cold" }
    } else {"ok"};
    println!("Today's atmosphere is {}", weather);
}

#[allow(dead_code)]
fn while_and_loop() {
    let mut x = 1;
    while x < 1024 {
        x *= 2;
        println!("x = {}", x);
    }

    x = 1;
    // while true
    loop {
        if x >= 1024 {
            break;
        }
        x *= 2;
        println!("x = {}", x);
    }

    let mut a = 1;
    let mut b;

    // loop with labels
    'outer: while a < 5 {
        b = 1;
        while b < 5 {
            if a == 3 && b == 3 {
                break 'outer;
            }
            println!("({},{})", a, b);
            b += 1;
        }
        a += 1;
    }
}

#[allow(dead_code)]
fn for_loop() {
    for x in 1..11 {
        if x == 5 {
            continue;
        }
        println!("{}", x);
    }

    for (i, y) in (1..11).enumerate() {
        println!("{}: {}", i, y);
    }
}

#[allow(dead_code)]
fn match_statement() {
    let country_code = 91;
    let country = match country_code {
        1 => "US",
        7 => "Russia",
        44 => "UK",
        46 => "Sweden",
        91 => "India",
        1..=1000 => "unknown", // Not in the list above
        _=> "invalid", // anything else
    };
    println!("The Country with code {} is {}", country_code, country);

    let x = false;
    let s = match x {
        true => "yes",
        _ => "no"
    };
    println!("match true({}): {}", x, s);
}

enum State {
    Locked,
    Failed,
    Unlocked,
}

use std::io::stdin;

#[allow(dead_code)]
fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end())
                    }
                    Err(_) => continue,
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry){
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}