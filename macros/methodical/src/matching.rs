// A macro that takes an expression and prints it.
// This example has a single rule that matches an expression.
// Capture the input as metavariable '$e', where e is an identifier
// and 'expr' is a fragment specifier. In this case, it is an expression.
// The expansion is usually written within curly braces and is followed
// by a semicolon.
macro_rules! myprintln {
    ($e:expr) => {
        println!("{}", $e);
    };
}

macro_rules! times_two {
    ($e:expr) => {
        2 * $e
    };
}

macro_rules! multiply_add {
    ($a:expr, $b:expr, $c:expr) => {
        $a * ($b + $c)
    };
}

macro_rules! discard {
    ($e:expr) => {};
}

macro_rules! repeat {
    ($e:expr) => {
        $e;
        $e;
        $e;
    };
}

pub fn matching() {
    myprintln!(2 + 3);
    myprintln!(times_two!(3));
    myprintln!(multiply_add!(1, 2, 3));
    discard!(multiply_add!(2, 3));
    repeat!(myprintln!(100 + 11));
}
