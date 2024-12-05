/// A struct that can split a string into parts with generic delimiters
#[derive(Debug)]
pub struct StrSplit<'r, D> {
    remainder: Option<&'r str>,
    delimiter: D,
}

impl<'r, D> StrSplit<'r, D> {
    /// Create a new `StrSplit` with a given haystack and delimiter.
    pub fn new(haystack: &'r str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

trait Delimiter {
    /// Find the next occurrence of the delimiter in the given string.
    /// Returns the start and end index of the delimiter.
    fn find(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'r, D> Iterator for StrSplit<'r, D>
where
    D: Delimiter,
{
    type Item = &'r str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((begin, end)) = self.delimiter.find(remainder) {
            // Take everything up to the beginning of the delimiter
            let next_str = &remainder[..begin];
            // Modify remainder to everything that follows the end of delimiter
            *remainder = &remainder[end..];
            Some(next_str)
        } else {
            self.remainder.take()
        }
    }
}

// We cannot use AsRef<str> instead of &str and &String and a separate
// implementation for char since upstream crates may add a new impl of
// trait `std::convert::AsRef<str>` for type `char` in future versions
// but it does not exist for now.

impl Delimiter for &str {
    fn find(&self, remainder: &str) -> Option<(usize, usize)> {
        remainder
            .find(self)
            .map(|begin| (begin, begin + self.len()))
    }
}

impl Delimiter for &String {
    fn find(&self, remainder: &str) -> Option<(usize, usize)> {
        remainder
            .find(self.as_str())
            .map(|begin| (begin, begin + self.len()))
    }
}

impl Delimiter for char {
    fn find(&self, remainder: &str) -> Option<(usize, usize)> {
        remainder
            .char_indices()
            .find(|(_, c)| c == self)
            .map(|(begin, _)| (begin, begin + self.len_utf8()))
    }
}

#[test]
fn test_basic() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    let expected = vec!["a", "b", "c", "d", "e"];
    assert!(letters.eq(expected));
}

#[test]
fn test_last_empty() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    let expected = vec!["a", "b", "c", "d", ""];
    assert!(letters.eq(expected));
}

#[test]
fn test_needle_with_different_lifetime() {
    let haystack = "a b";
    {
        let needle = &format!("{}", ' ');
        let letters = StrSplit::new(haystack, needle);
        let expected = vec!["a", "b"];
        assert!(letters.eq(expected));
    }
}

#[test]
fn test_character_delimiter() {
    let haystack = "aXbXcXdXe";
    let letters = StrSplit::new(haystack, 'X');
    let expected = vec!["a", "b", "c", "d", "e"];
    assert!(letters.eq(expected));
}
