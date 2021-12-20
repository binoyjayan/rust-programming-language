
// Iterator that skips every other element
// This iterator can wrap around another iterator
// Here, we are creating a combinator method for iterators

// Here, I must be an iterator that we are wrapping with our SkipIterator
pub struct SkipIterator<I: Iterator> {
    inner: I,
}

// Now implement Iterator for SkipIterator where I itself is an Iterator.
// This requires two types - the iterator type 'I' and the return type 'T'
// Use where clause instead of using colon (:) at impl level
// where I is an Iterator <Item = T>. Return type is 'T'
// The following two lines mean the same thing (second one is easier to read)
// impl <I: Iterator <Item = T>, T> Iterator for SkipIterator<I> where I:  {
impl <I, T> Iterator for SkipIterator<I> where I: Iterator <Item = T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()?; // return immediately if None
        self.inner.next()
    }
}

// In order to make it work with any other trait, we should make our own trait
// The trait should itself be an Iterator. It is achived by specifying a bound
// on the trait using a colon.
// We also need to require that this is 'Sized'.
// Almost all type in Rust implements Sized. It is usually an auto type.
pub trait IterCombi: Iterator + Sized {
    fn skip_half(self) -> SkipIterator<Self> {
        SkipIterator {inner : self}
    }
}

// This means every 'iterator' and 'Sized' object implements IterCombi.
// So, for everything that implements Iterator and Sized, 
// it now implements IterCombi as long as the trait IterCombi
// is in scope while running. So, every 'Iterator + Sized' object thus
// gains access to the new method skip_half() implemented in 'IterCombi'
impl <I:Iterator + Sized> IterCombi for I { }


#[cfg(test)]
mod test_skip {
    use super::*;
    #[test]
    fn test_skip_half() {
        // Create a new iterator.
        // (0..10) is a syntactic sugar for creating a Range iterator 0 to 9 inclusive
        let v: i32 = (0..10).skip_half().sum();
        assert_eq!(v, 1 + 3 + 5 + 7 + 9);
    }

}

// step_by: part of std library
#[test]
fn test_step_by() {
    let v: i32 = (0..10).step_by(3).sum();
    assert_eq!(v, 0 + 3 + 6 + 9);
}

use itertools::Itertools;
// interleave: part of itertools library
#[test]
fn test_interleave() {
    let v: Vec<i32> = (0..4).interleave((10..14).rev()).collect();
    assert_eq!(v, vec![0, 13, 1, 12, 2, 11, 3, 10]);
}

// intersperse: part of itertools library
#[test]
fn test_intersperse() {
    let s1 = "hello world etc";
    let v: Vec<&str> = s1.split(" ").intersperse(",").collect();
    assert_eq!(v, ["hello", ",", "world", ",", "etc"]);

    let s2 = s1.split(" ").join(",");
    assert_eq!(s2, "hello,world,etc");
}


