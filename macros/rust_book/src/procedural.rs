use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

pub fn procedural() {
    Pancakes::hello_macro();
}
