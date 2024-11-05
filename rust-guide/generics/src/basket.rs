use super::container::Container;

#[derive(Debug)]
pub struct Basket<T> {
    item: Option<T>,
}

// The first 'T' is the declaration of the generic type
// The second 'T' is a reference to the generic type declared before
impl<T> Basket<T> {
    pub fn new(item: T) -> Self {
        Basket { item: Some(item) }
    }
}

impl<T> Container<T> for Basket<T> {
    fn get(&mut self) -> Option<T> {
        self.item.take()
    }

    fn put(&mut self, item: T) {
        self.item = Some(item);
    }

    fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}
