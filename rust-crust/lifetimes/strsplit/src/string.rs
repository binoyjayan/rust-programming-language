/// A struct that can split a string into parts with delimiters that are strings.
#[derive(Debug)]
pub struct StrSplitStr<'r, 'd> {
    remainder: Option<&'r str>,
    delimiter: &'d str,
}

impl<'r, 'd> StrSplitStr<'r, 'd> {
    /// Create a new `StrSplit` with a given haystack and delimiter.
    pub fn new(haystack: &'r str, delimiter: &'d str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// Implement the Iterator trait for StrSplitStr
// Use lifetime ellision for 'd, since we are only returning
// references to the remainder.
impl<'r> Iterator for StrSplitStr<'r, '_> {
    type Item = &'r str;

    fn next(&mut self) -> Option<Self::Item> {
        // If remainder is None, return None, else get a mutable reference
        // to 'remainder' within the Option. Use 'as_mut' to avoid copy semantics
        // while taking the value from within the Option.
        let remainder = self.remainder.as_mut()?;

        if let Some(next_index) = remainder.find(self.delimiter) {
            // Take everything up to the beginning of the delimiter
            let next_str = &remainder[..next_index];
            // Modify remainder to everything that follows the delimiter
            *remainder = &remainder[(next_index + self.delimiter.len())..];
            Some(next_str)
        } else {
            self.remainder.take()
        }
    }
}

#[test]
fn test_basic() {
    let haystack = "a b c d e";
    let letters = StrSplitStr::new(haystack, " ");
    let expected = vec!["a", "b", "c", "d", "e"];
    assert!(letters.eq(expected));
}

#[test]
fn test_last_empty() {
    let haystack = "a b c d ";
    let letters = StrSplitStr::new(haystack, " ");
    let expected = vec!["a", "b", "c", "d", ""];
    assert!(letters.eq(expected));
}

#[test]
fn test_needle_with_different_lifetime() {
    let haystack = "a b";
    {
        let needle = &format!("{}", ' ');
        let letters = StrSplitStr::new(haystack, needle);
        let expected = vec!["a", "b"];
        assert!(letters.eq(expected));
    }
}
