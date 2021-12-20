use std::ops::AddAssign;

// Add a Rangeable type - trait aliasing
pub trait Rangeable: PartialOrd + AddAssign + Copy {}

// For any type T that implements AddAssign, PartialOrd, Copy, impl Rangeable
// This tells rust that types such as float implements Rangeable
// only do it only in the library where the crate is created or where the type is created
impl<T:AddAssign + PartialOrd + Copy> Rangeable for T {}

// Make iterator generic over diifferent types
pub struct GenRangeIterator<T:Rangeable> {
    curr: T,
    stop: T,
    step: T,
}

// impl<T:PartialOrd + AddAssign + Copy> GenRangeIterator<T> {
impl<T:Rangeable> GenRangeIterator<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        GenRangeIterator {
            curr: start,
            stop,
            step,
        }
    }
}

// Limit the type that can use >= operator by using trait 'PartialOrd'
// Use AddAssign for using += operator
// Use Copy for copying the generic type (instead of moving them)
// This is similar to how it is done for primitive number types such as int, float
// impl<T:PartialOrd + AddAssign + Copy> Iterator for GenRangeIterator<T> {
impl<T:Rangeable> Iterator for GenRangeIterator<T> {
    type Item = T;
    // next should have a mutable ref to self as it can modify the object
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.stop {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

// Test area - cfg(test) will not be oart of final binary
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gen_test1() {
        let mut m = 0.0;
        let it = GenRangeIterator::new(5.0, 12.0, 3.0);
        for s in it {
            m += s;
        }
        assert_eq!(m, 5. + 8. + 11., "Float test");
    }
}

