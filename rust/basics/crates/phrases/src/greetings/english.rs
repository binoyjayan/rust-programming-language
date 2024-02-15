//! This module contains english phrases
//!
//! # Examples
//!
//! ```
//! let username = "John";
//! println!("{} {}!", phrases::greetings::english::hello(), username);
//! ```
//!


/*
 * To generate documentation:
 *
 * rustdoc english.rs
 */

/// This applies to code that follows it
/// The hello function returns the hello greeting
pub fn hello() -> String {
    "hello".to_string()
}
/// The hello function returns the goodbye greeting
pub fn goodbye() -> String {
    "goodbye".to_string()
}


