# Variance

Variance is how subtyping between more complex types relates to subtyping between their components.

## Covariance - Most types

Covariance refers to the ability to use a more derived type (a subtype) than expected.

Consider the function foo defined as follows:

```Rust
fn foo(x: &'a str) {}
```

This function can be called with a string of lifetime 'a or a string of lifetime 'static:

```Rust
 foo(&'a str);
 foo(&'static str);
```

Here, the parameter x of the function foo is covariant. It allows a string of any lifetime that outlives 'a to be passed, including 'static, which outlives all lifetimes.

This covariance can also be seen with the type of a variable.
Consider a variable x of type &'a str:

```Rust
let x: &'a str;
```

Can assign:

```Rust
x = &'a str;
x = &'static str;

&'static str // Atleast (or more) useful (subtype of &'a str)
&'a str      // Less useful
```

Here, x is covariant with respect to its type. It can be assigned a reference to a string of any lifetime that outlives 'a, including 'static.

In this context, 'static is considered a subtype of 'a because 'static outlives 'a. The lifetime 'static is useful in at least as many contexts as 'a, which is why covariance allows us to use 'static wherever 'a is expected.

```Rust
'static <: 'a
&'static T <: &'a T
```

## Contravariance: Subtyping relation for complex types is flipped

The subtyping relation of the simple types is reversed for the complex types.
The function takes the short lifetime is the more useful one.

e.g.

```Rust
fn foo(f: Fn(&'static str)) {
  // The caller might pass an argument with a lifetime longer than required
  // Therefore, the function passed to `foo` must be a function
  // whose parameter has a more constrained lifetime such as 'static
  f("hello world");
}
```

Therefore, it is possible to call:

```Rust
foo(fn(&'a str))
foo(fn(&'st str))
```

Given a variable x of type Fn(&'a str)), it expects a function that can take a string with a short lifetime.
If a function is provided that requires a string with a long lifetime, it cannot be used by x.

Normally, a function expecting a string with a short lifetime is satisfied if provided a string with a long lifetime.
However, when a function is provided instead of a string, the lifetime requirements are reversed.

```Rust
Fn(&'static str)      // Has stricter requirements of its caller
                      // Can only take a longer lifetime string
Fn(&'a str)           // More useful (more generally applicable).
                      // Can take long or short lifetime string.

Fn(&'a T) <: Fn(&' static T)
```

## Invariance

A type is invariant if no subtyping relationship is allowed

T<'a> and T<'b> are considered entirely different types even if 'a: 'b.

```Rust
&'a T       // Here, the type is covariant in both 'a and in 'T'
&'a mut T   // Here, the type is covariant in 'a but invariant in 'T'
```
e.g.

```Rust
fn foo<'a>(s: &mut &'a str, x: &'a str) {
    *s = x;
}

let mut s: &'static str = "hello";
let z = "world".to_string();
foo(&mut s, &z);                         <<-- Doesn't compile
//   foo(&mut &'static str, &'a str)     <<-- Cannot make "'static str" into "'a" like so:
//   foo(&mut &'a str,      &'a str)
```

Since 's' is behind a mutable reference, they are invariant behind the type 'T' in this case '&str'
So, the compiler does not allow "'static str" to become "'a"

Now about why the type it is 'covariant' in the lifetime 'a:

e.g.

```Rust
fn bar() {
    let mut y = true;
    let mut z = &mut y;       // Assume that lifetime is 'y
    let x = Box::new(true);
    let x = Box::leak(x);     // has 'static lifetime
    z = x;                    // &'y mut bool = &'static mut bool
}
```
