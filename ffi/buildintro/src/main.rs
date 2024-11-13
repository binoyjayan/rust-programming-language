// OUT_DIR can have output from the build script
// env!() macro is used since OUT_DIR is a compile-time environment variable   
mod foo {
    include!(concat!(env!("OUT_DIR"), "/foo.rs"));
}

fn main() {
    foo::foo();
}

// cargo expand
// cargo run
