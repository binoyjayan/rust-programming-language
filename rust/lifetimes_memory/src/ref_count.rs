
struct Person {
    name: std::rc::Rc<String>
}

impl Person {
    fn new(name: std::rc::Rc<String>) -> Person {
        Person {name: name}
    }
    fn greet(&self) {
        println!("My name is {}", self.name);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn ref_counting() {
    let name = std::rc::Rc::new("John".to_string());
    println!("Name = {}, has {} strong pointers", name, std::rc::Rc::strong_count(&name));
    {
        // name would have been moved to Person if it was not a reference counted variable
        let person = Person::new(name.clone());
        println!("Name = {}, has {} strong pointers", name, std::rc::Rc::strong_count(&name));
        person.greet();
    }
    println!("Name = {}, has {} strong pointers", name, std::rc::Rc::strong_count(&name));
    println!("The name is {}", name);
}
