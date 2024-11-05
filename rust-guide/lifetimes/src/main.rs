// If there are more than one argument to the function,
// it will assume that the life time of the value returned
// is either the same as the first argument or as that of the
// the second argument. But it does not know the which one.
// Since it cannot choose between the two, we'll have to
// provide lifetime annotations to the function.
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;
    for lang in languages {
        if found {
            return lang;
        }
        if lang == current {
            found = true;
        }
    }
    languages.last().unwrap()
}

// This function will return the last language in the list
// Since there is only one argument, Rust compiler assumes that
// the lifetime of the value returned will be the same as the argument.
fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

/// Cases where lifetime annotations can be omitted
/// fn generate(set: &[i32], range: i32) -> &str
/// fn leave(message: &Message, text: String) -> &str
/// Lifetime of return values are tied to self
/// fn get_name(&self, def_name: &str) -> &str

// Here we cannot say for sure which of the two arguments
// is the lifetime of the value returned. So use the same lifetime.
fn longest_language<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let languages = vec![
        "Rust".to_string(),
        "C".to_string(),
        "TypeScript".to_string(),
        "Python".to_string(),
    ];

    let next = next_language(&languages, "Rust");
    println!("Next language is {}", next);
    let last = last_language(&languages);
    println!("Last language is {}", last);
    let a = "Rust".to_string();
    {
        // In this case, the lifetimes of the arguments are limited
        // to the inner block.
        let b = "Go".to_string();
        let longest = longest_language(&a, &b);
        println!("Longest language is {}", longest);
    }
    let longest = longest_language("C", "C++");
    println!("Longest language is {}", longest);
}
