fn _demo1() {
    let colors = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
    // Create an iterator from a vector Iter<'_, String>
    let mut colors_iter = colors.iter();
    println!("{:#?}", colors_iter.next()); // red
    println!("{:#?}", colors_iter.next()); // green
    println!("{:#?}", colors_iter.next()); // blue
    println!("{:#?}", colors_iter.next()); // None
    println!("{:#?}", colors_iter.next()); // None
}

// Types of Iterators
// There are three types of iterators in Rust that are created
// using the iter(), iter_mut(), and into_iter() methods.
// iter() returns an iterator that gives an immutable reference to each element
// iter_mut() returns an iterator that gives a mutable reference to each element
// into_iter() returns an iterator that takes ownership of each element
//              unless called on a ref to a vector.

// Use an iterator consumer (for_each) to print elements
// For for_each is an iterator consumer that calls 'next()'
// on the iterator and unwraps the option that is returned.
// The loop stops when the iterator returns None.
// Note that the items are iterated only when a consumer
// is called. However, iterator adapters (such as map) are
// lazy and do not consume the iterator.
// It relies on the consumer to consume the iterator

// Demonstrating the use of 'iter()'
fn print_elements(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

// Demonstrating the use of 'iter_mut()'
// Take a mutable reference to a slice of strings and
// pass it to the iterator consumer for_each to truncate.
fn shorten_strings(elements: &mut [String], len: usize) {
    elements
        .iter_mut()
        .for_each(|el| el.truncate(el.len() - len));
}

// Demonstrating the use of 'collect()'. The collect knows the data type
// of the collection target by looking at the type annotation.
// It does not necessarily have to be a vector. It could be other
// types such as a LinkedList.
fn to_upper_case(elements: &mut [String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect()
}

// Demonstrating the use of 'into_iter()'
// Take all elements from the first vector to the second vector
// Since 'into_iter()' takes ownership of the elements,
// the elements can be moved somewhere else.
// However, into_iter() cannot always give ownership of the elements.
// This can happen depending upon how the function is called.
// Three ways to call the function on a vector "v":
// &v.into_iter() - Iterator created out of a read only reference
//                  Iterator will produce refs to each value
// &mut v.into_iter() - Iterator created out of a mutable reference
//                      Iterator will produce mutable refs to each value
// v.into_iter() - Iterator created out of a value
//                 Iterator will produce each value and moves ownership
//                 of the value to the consumer
// This means that 'into_iter()' can be used in place of either
// 'iter()' and 'iter_mut()' by changing what we call into_iter() on.
// Take ownership of argument a since we are moving the elements
// out of 'a'. But we only need a mutable reference to 'b' since
// we are only borrowing it and modifying it.
fn move_elements(a: Vec<String>, b: &mut Vec<String>) {
    // 'el' in this case is a String that we have ownership of
    a.into_iter().for_each(|el| b.push(el));
}

// Explore vectors of strings in to a vector of vectors of strings
// by splitting each string in the vector into its characters.
fn explode(v: &[String]) -> Vec<Vec<String>> {
    v.iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

// search for a sub string in each of the string in the vector
// If match is found return that color return a fallback string.
// pattern - substring to search for. Use a reference since we are
//           only using it for calculation and not taking ownership
// fallback - string to return if no match is found. Use a reference too.
// Return a String since we do want it to refer to a something on the vector
// find() return either Some(ref color) or None. Use map_or() to return
// return the color wrapped in option or the fallback if the option is None.
// Alternatively, we could use unwrap_or() instead of map_or().
fn find_color_or(v: &[String], pattern: &str, fallback: &str) -> String {
    v.iter()
        .find(|el| el.contains(pattern))
        .map_or(fallback.to_string(), |el| el.to_string())
}

fn main() {
    let mut colors = vec![
        "red color".to_string(),
        "green color".to_string(),
        "blue color".to_string(),
    ];
    // print_elements(&colors);
    shorten_strings(&mut colors, 6);
    let upper = to_upper_case(&mut colors);
    let mut dest = Vec::new();
    move_elements(upper, &mut dest);
    // 'colors' cannot be used anymore since it has been moved
    print_elements(&dest);
    let exploded = explode(&dest);
    println!("{:#?}", exploded);
    let found = find_color_or(&dest, "RE", "NONE");
    println!("{:#?}", found);
}
