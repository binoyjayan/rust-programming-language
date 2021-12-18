use serde_derive::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

fn main() {
    println!("Transactions");
    // let trans = get_transaction("test/transactions.json").unwrap();
    let trans = get_transactions3("test/transactions.json").expect("Failed to load transactions");
    for t in trans {
        println!("{:?}", t);
    }
}

// Use enum Result<T, E> to handle things that might fail. unwrap is not recommended to handle errors
pub fn get_transactions1(fname: &str) -> Result<Vec<Transaction>, String> {
    // Err("No transactions found".to_string())
    let s = std::fs::read_to_string(fname).unwrap();
    let t: Vec<Transaction> = serde_json::from_str(&s).unwrap();
    Ok(t)
}

// using match to handle results
pub fn get_transactions2(fname: &str) -> Result<Vec<Transaction>, String> {
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(t) => t,
        Err(e) => return Err(e.to_string()),
    };
    Ok(t)
}

// Avoid using match with combinators
// https://doc.rust-lang.org/std/result/
// map_err: maps IO error type to a string type but it does not handle the main type.
// handle the main type in 'and_then()' which converts the loaded string result (ld)
// to another type using the function uses serde_json::from_str and returns another type
pub fn get_transactions3(fname: &str) -> Result<Vec<Transaction>, String> {
    std::fs::read_to_string(fname)
        .map_err(|e| e.to_string())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
}
