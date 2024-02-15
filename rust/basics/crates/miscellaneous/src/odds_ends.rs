
// from crates.io
extern crate rand;
use rand::Rng;

// Including custom crate
extern crate phrases;
use phrases::greetings::english;
use phrases::greetings::french;

pub fn odds_ends() {
    let mut rng = rand::thread_rng();
    let _b:bool = rng.gen();

    println!("English: {} {}", english::hello(), english::goodbye());
    println!("French: {} {}", french::hello(), french::goodbye());

}

