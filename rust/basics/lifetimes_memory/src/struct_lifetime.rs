
// Lifetime of Person is tied to the lifetime of name
struct Person<'z> {
    name: &'z str
}

// Lifetime is needed for the implementation also
impl<'z> Person<'z> {
    fn talk(&self) {
        println!("My name is {}", self.name)
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn struct_lifetime() {
    let person = Person { name: "Dmitri" };

    person.talk();
}
