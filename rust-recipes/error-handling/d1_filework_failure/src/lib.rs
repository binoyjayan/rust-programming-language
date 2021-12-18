
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

// Instead of returning an Option of type Transaction, return a Result of type Transaction
// and an error type of 'failure::Error'
pub fn get_transactions_for(fname: &str, uname: &str) -> Result<Transaction, failure::Error> {
    // return the Result itself back
    let trans = get_transactions(fname)?;

    println!("Iterating throught transactions");

    for t in trans {
        if t.from == uname {
            // return a Result here too.
            return Ok(t);
        }
    }
    // return a Error of type Mess, followed by converting it into failure::Error using '.into()'
    Err(TransactionError::Mess("Could not find transaction with that name").into())
}
