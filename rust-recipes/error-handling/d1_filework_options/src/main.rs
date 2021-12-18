use serde_derive::*;

// The error type contained in these Enums can optionally contain
// the actual error type (e.g. std::io::Error) but it doesn't have to
// A feature of Enum that we can take advantage of, to build a state tree
// by combining multiple error types
#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
    Mess(&'static str),
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

// Implement custom error type that accepts a &'static str
impl From<&'static str> for TransactionError {
    fn from(e: &'static str) -> Self {
        TransactionError::Mess(e)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}
fn print_transactions(trans: &Vec<Transaction>) {
    println!("Transactions:");
    for t in trans {
        println!("{:?}", t);
    }
}

fn main() -> Result<(), TransactionError> {
    let trans = get_transactions("test/transactions.json").expect("Failed to load transactions");
    print_transactions(&trans);

    // First transaction for 'Matt'.
    // For  now, consider the absence of the transaction as an error
    // or_ok() converts an Option into a Result by providing an error if the Option is 'None'
    // The question mark(?) converts the string (&'static str) into the error type TransactionError
    let t = get_transactions_for("test/transactions.json", "Matt1").ok_or("Could not get first transaction")?;
    println!("Matt's first transaction: {:?}", t);
    // Return empty Result
    Ok(())
}

// Using '?' to leverage the advantages of using the From trait
pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    // Question mark creates a match statement and returns the '.into()' of that error in place
    // Only works if error type we are returning to implements 'From' trait
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}

// Returns an Option of type Transaction - used for presence or absence of a value
pub fn get_transactions_for(fname: &str, uname: &str) -> Option<Transaction> {
    // Ok converts a Result into an Option and discarding the error, if any.
    // The value part in Result becomes the Some() and the Error type becomes None. 
    // The question mark (?) knows how to handle the options
    let trans = get_transactions(fname).ok()?;
    // print_transactions(&trans);

    println!("Iterating throught transactions");

    // Iterate through transactions - there is a better way to handle this using iterators.
    for t in trans {
        if t.from == uname {
            return Some(t);
        }
    }
    None
}

