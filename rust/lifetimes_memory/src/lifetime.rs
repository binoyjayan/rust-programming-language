
struct Person {
    name: String
}

impl Person {
    // Compiler generated lifetimes - fn get_ref_name<'a>(&'a self) -> &'a String {
    fn get_ref_name(&self) -> &String {
        &self.name
    }
}

// Lifetime of the Company is tied to the lifetime of Person - 'z
struct Company<'z> {
    name: String,
    ceo: &'z Person
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn lifetime() {
    /*
    * static is a special name.
    * It is an example of a lifetime (lives till the end of the program)
    *     &'static str
    */

    let boss = Person { name: String::from("Elon Musk")};
    let tesla = Company { name: String::from("Tesla"), ceo : &boss };

    let s: &String;
    {
        let p = Person { name: "John".to_string()};
        s = p.get_ref_name();
        println!("ref: {}", s);
    }
    println!("ceo of {} is {}", tesla.name, tesla.ceo.name);
}
