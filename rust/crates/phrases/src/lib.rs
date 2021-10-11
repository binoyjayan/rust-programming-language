pub mod greetings {
    // A module definition can be in a separate file too
    pub mod english;
    pub mod french {
        pub fn hello() -> String {
            "bonjour".to_string()
        }
        pub fn goodbye() -> String {
            "au revoir".to_string()
        }
    }
}

// Tests can either be added here on inside the test directory
#[test]
fn english_greeting_correct() {
    assert_eq!("hello", greetings::english::hello())
}

