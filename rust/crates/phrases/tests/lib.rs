#[cfg(test)]
mod tests {
    extern crate phrases;

#[test]
#[should_panic]
#[ignore]
fn french_greeting_panic() {
    assert_eq!("hello", phrases::greetings::french::hello())
}

}

