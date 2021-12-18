
// Consider the library as a separate crate
extern crate d1_filework;

// import everything using '*'
// use d1_filework::*;

// Import function individually
use d1_filework::{TransactionError, get_transactions, get_transactions_for, print_transactions};

fn main() -> Result<(), TransactionError> {
    let trans = get_transactions("test/transactions.json").expect("Failed to load transactions");
    print_transactions(&trans);
    let m = get_transactions_for("test/transactions.json", "Matt1").ok_or("Could not get first transaction")?;
    println!("Matt's first transaction: {:?}", m);
    let j = get_transactions_for("test/transactions.json", "jack").ok_or("Could not get first transaction")?;
    println!("Jack's first transaction: {:?}", j);
    Ok(())
}


