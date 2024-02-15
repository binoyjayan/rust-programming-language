
use std::sync;
use std::thread;

struct Person {
    name: sync::Arc<String>
}

impl Person {
    fn new(name: sync::Arc<String>) -> Person {
        Person {name: name}
    }
    fn greet(&self) {
        println!("My name is {}", self.name);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn atomic_reference() {
    let name = sync::Arc::new("John".to_string());

    let person = Person::new(name.clone());
    let t = thread::spawn(move || {
        person.greet();
    });
    println!("The name is {}", name);
    t.join().unwrap();
}
