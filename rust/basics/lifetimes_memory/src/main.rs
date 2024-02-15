mod ownership;
mod borrowing;
mod lifetime;
mod struct_lifetime;
mod ref_count;
mod atomic_reference;
mod using_mutex;

#[allow(unused_imports)]

use ownership::*;
use borrowing::*;
use lifetime::*;
use struct_lifetime::*;
use ref_count::*;
use atomic_reference::*;
use using_mutex::*;

fn main() {
    println!("Lifetimes and memory");
    // ownership();
    // borrowing();
    // lifetime();
    // struct_lifetime();
    // ref_counting();
    // atomic_reference();
    using_mutexes();
}
