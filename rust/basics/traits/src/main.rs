
mod introduction;
mod overloading;
mod dispatches;
mod heterogenous;

use introduction::*;
use overloading::*;
use dispatches::*;
use heterogenous::*;

fn main() {
    println!("Traits");
    // traits_basic();
    // traits_parameters();
    // traits_into();
    // traits_drop();
    // traits_operator_overloading();
    // static_dispatch(); // compile time
    // dynamic_dispatch_simple(); // runtime
    // dynamic_dispatch();
    heterogenous_vectors();
}