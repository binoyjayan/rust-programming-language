use serde_derive::*;

// The error type contained in these Enums can optionally contain
// the actual error type (e.g. std::io::Error) but it doesn't have to
// A feature of Enum that we can take advantage of, to build a state tree
// by combining multiple error types
#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
}

// Implement custom error type 'LoadError' to handle 'std::io::Error'
impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

// Implement custom error type 'ParseError' to handle 'serde_json::Error'
impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

fn main() {
    println!("Transactions");
    let trans = get_transactions("test/transactions.json").expect("Failed to load transactions");
    for t in trans {
        println!("{:?}", t);
    }
}

// Note the custom error type 'TransactionError' instead of a string
// The 1st 'into' methods converts the io::error to LoadError
// The 2nd 'into' methods converts the serde_json::error to ParseError
pub fn get_transactions1(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    std::fs::read_to_string(fname)
        .map_err(|e| e.into())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.into()))
}

// Using match statement with into() feature provided by the From trait
pub fn get_transactions2(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    Ok(
        match serde_json::from_str(&match std::fs::read_to_string(fname) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        }) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        },
    )
}

// Using '?' to leverage the advantages of using the From trait
pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    // Question mark creates a match statement and returns the '.into()' of that error in place
    // Only works if error type we are returning to implements 'From' trait
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}

