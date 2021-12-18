
use failure::Error;
extern crate d1_filework;

use d1_filework::{get_transactions, get_transactions_for, print_transactions};

// Now main returns an error of type failure::Error
// failure::Error is a wrapper around a single pointer to an error.
// It implements 'into()' for anything that implements 'fail'
// We are able to use '?' below (with ok_or()) because of that.
fn main() -> Result<(), Error> {
    let trans = get_transactions("test/transactions.json").expect("Failed to load transactions");
    print_transactions(&trans);
    
    // implements the 'fail' as defined in the
    let t = get_transactions_for("test/transactions.json", "Matt");
    match t {
        Ok(v) => println!("Found transaction: {:?}", v),
        Err(e) => println!("{}\nBacktrace: \n{}", e, e.backtrace()),
    }
    // Pass env RUST_BACKTRACE=1 to program for displaying backtrace using 'backtrace()'
    Ok(())
}


// Notes:
// While developing a library, a custom error type must be created that implements the Fail.
// While developing an application, use failure Error type without custom types
