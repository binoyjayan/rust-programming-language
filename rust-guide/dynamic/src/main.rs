use std::any::Any;

pub trait Container {
    fn get(&mut self) -> Box<dyn Any>;
    fn put(&mut self, item: Box<dyn Any>);
    fn is_empty(&self) -> bool;
}

#[derive(Debug)]
pub struct Basket {
    item: Option<Box<dyn Any>>,
}

impl Basket {
    pub fn new(item: Box<dyn Any>) -> Self {
        Basket { item: Some(item) }
    }
}

impl Container for Basket {
    fn get(&mut self) -> Box<dyn Any> {
        self.item.take().unwrap()
    }

    fn put(&mut self, item: Box<dyn Any>) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}

#[derive(Debug)]
pub struct Stack {
    items: Vec<Box<dyn Any>>,
}

impl Stack {
    pub fn new(items: Vec<Box<dyn Any>>) -> Self {
        Stack { items }
    }
}

impl Container for Stack {
    fn get(&mut self) -> Box<dyn Any> {
        self.items.pop().unwrap()
    }

    fn put(&mut self, item: Box<dyn Any>) {
        self.items.push(item);
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

fn add<T: 'static>(c: &mut dyn Container, item: T) {
    c.put(Box::new(item));
}

fn main() {
    let mut b1 = Basket::new(Box::new("Apple".to_string()));
    let mut b2 = Basket::new(Box::new(1));

    let mut s1 = Stack::new(vec![Box::new("Apple".to_string())]);
    let mut s2 = Stack::new(vec![Box::new(1)]);

    add(&mut b1, "Orange".to_string());
    add(&mut s1, "Orange".to_string());
    add(&mut b2, 2);
    add(&mut s2, 2);

    println!("b1: {:#?}", b1);
    println!("b2: {:#?}", b2);
    println!("s1: {:#?}", s1);
    println!("s2: {:#?}", s2);
}
