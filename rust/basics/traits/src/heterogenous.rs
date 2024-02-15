
trait Animal {
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn name(&self) ->&'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says Hello", self.name());
    }
}

impl Animal for Cat {
    fn name(&self) ->&'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says Meow", self.name());
    }
}

enum Creature {
    Human(Human),
    Cat(Cat)
}

// Vectors of different objects
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn heterogenous_vectors() {
    let mut creatures = Vec::new();
    /*
     * Cannot store heterogeneous struct objects in an array
     *
     * creatures.push(Human{"John"});
     * creatures.push(Cat{("Missy"});
    */
    // But, we can store heterogeneous enums (that stores structs) in the vector
    creatures.push(Creature::Human(Human{name: "John"}));
    creatures.push(Creature::Cat(Cat{name: "Missy"}));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) =>  c.talk()
        }
    }

    // Or we can wrap them in a Box as the size of the object is known
    let mut animals: Vec<Box<Animal>>  = Vec::new();
    animals.push(Box::new(Human{name: "John"}));
    animals.push(Box::new(Cat{name: "Missy"}));

    for a in animals.iter() {
        a.talk();
    }
}
