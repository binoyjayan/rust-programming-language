use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    // let lines: Vec<&str> = text.lines().collect();
    let lines: Vec<&str> = text.split('\n').collect();
    let mut errors = Vec::new();
    for line in lines {
        if line.starts_with("ERROR") {
            errors.push(line.to_string());
        }
    }
    errors
}

// Return an io error
fn _main1() -> Result<(), std::io::Error> {
    Err(std::io::Error::other("error ----"))
}

// Use expect to handle the error
fn _main2() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("logs.txt").expect("Error reading file");
    let errors = extract_errors(content.as_str());
    fs::write("errors.txt", format!("{}\n", errors.join("\n"))).expect("Error writing file");
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("logs.txt")?;
    let errors = extract_errors(content.as_str());
    fs::write("errors.txt", format!("{}\n", errors.join("\n")))?;
    println!("Errors written to errors.txt");
    Ok(())
}
