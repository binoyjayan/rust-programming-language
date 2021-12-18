
// lib.rs : Main library
use serde_derive::*;

// Look for a module error in  the current directory with the name error.rs
// Or a directory named error containing a file named mod.rs
mod error;
// Make this accessible outside crate
pub use error::TransactionError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

pub fn print_transactions(trans: &Vec<Transaction>) {
    println!("Transactions:");
    for t in trans {
        println!("{:?}", t);
    }
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}

// Returns an Option of type Transaction - used for presence or absence of a value
pub fn get_transactions_for(fname: &str, uname: &str) -> Option<Transaction> {
    let trans = get_transactions(fname).ok()?;

    println!("Iterating throught transactions");

    for t in trans {
        if t.from == uname {
            return Some(t);
        }
    }
    None
}
