fn main() {
    println!("cargo:rerun-if-changed=src/foo.rs");
    let outdir = std::env::var("OUT_DIR").unwrap();
    let path = std::path::PathBuf::from(outdir).join("foo.rs");
    let contents = r#"
    pub fn foo() {
        println!("Hello");
    }
    "#;
    std::fs::write(path, contents).unwrap();
}

// After cargo build, you can see the output of the build script by running:
// cat target/debug/build/*/output
// cat target/debug/build/*/stderr
// ls target/debug/build/*/out/*.rs
// Here '*' indicates the directory that is the output of the "build" crate.
