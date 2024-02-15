// https://www.youtube.com/watch?v=dHkzSZnYXmk
use std::sync::Arc;

fn main() {
    println!("Closures and Fn traits");
    foo(bar::<u32>);
    foo(bar::<i32>);
    baz(bar::<i32>);

    // non capturing closure
    let f1 = |_: u32| 0;
    // this closure can be coersed to fn pointer
    foo(f1);
    baz(f1);

    let mut z = String::new();
    // This closure takes a mutable reference of 'z' so it will need 'FnMut'
    let mut f2 = || {
        z.clear();
    };
    quox(&mut f2);

    // This closure moves 'z' into it so it will need 'FnOnce'
    let f3 = move || {
        drop(z);
    };
    quux(f3);

    // We can call the closure twice because the closure
    // captures and owns the string within its context
    let f4 = make_fn();
    f4();
    f4();

    // dyn reference
    let x = String::from("&dyn Fn()");
    let f4 = move || {
        println!("{}", x);
    };
    let f: &dyn Fn() = &f4;
    dyn_fn(f);

    // Boxed dyn
    let x = String::from("Box<dyn Fn()>");
    let f5 = move || {
        println!("{}", x);
    };
    let f: Box<dyn Fn()> = Box::new(f5);
    dyn_fn_boxed(f);

    // Arc dyn
    let x = String::from("Arc<dyn Fn()>");
    let f6 = move || {
        println!("{}", x);
    };
    let f: Arc<dyn Fn()> = Arc::new(f6);
    dyn_fn_arc(f);

    let _ = make_zero();
}

fn bar<T>(_: u32) -> u32 {
    0
}

// closures can only be coerced to `fn` types if they do not capture any variables
// Because of that use FnMut or FnOnce instead of 'fn'
//  FnOnce
//    ^
//  FnMut
//    ^
//   Fn
//    ^
//   fn

fn foo(f: fn(u32) -> u32) {
    println!("foo: fn size: {}", std::mem::size_of_val(&f));
}

// Use trait bound Fn, FnOnce or FnMut
fn baz<F>(f: F)
where
    F: Fn(u32) -> u32,
{
    println!("baz: fn size: {}", std::mem::size_of_val(&f));
}

fn quox<F>(f: &mut F)
where
    F: FnMut(),
{
    println!("quox: fn size: {}", std::mem::size_of_val(&f));
    (f)();
}

fn quux<F>(f: F)
where
    F: FnOnce(),
{
    println!("quux: fn size: {}", std::mem::size_of_val(&f));
    (f)();
}

// Function that returns a closure. It is ok to use Fn and not FnOnce
// because we are moving the variable 'z' into the closure and the
// closure does not reference the variable in the surrounding scope
fn make_fn() -> impl Fn() {
    let z = String::from("Z");
    move || {
        println!("closure: z: {}", z);
    }
}

// dyn pointers are not sized so we need a reference to it
fn dyn_fn(f: &dyn Fn()) {
    f()
}

// Boxed references do not require the 'dyn' reference to be sized
// since it is wrapped in a box.
fn dyn_fn_boxed(f: Box<dyn Fn()>) {
    f()
}

fn dyn_fn_arc(f: Arc<dyn Fn()>) {
    f()
}

const fn make_zero() -> i32 {
    // this function is similar to "let x = || 0"
    0
}
// More on constant functions
// https://internals.rust-lang.org/t/pre-rfc-revamped-const-trait-impl-aka-rfc-2632/15192
