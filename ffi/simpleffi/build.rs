fn main() {
    println!("cargo:rerun-if-changed=src/math.c");
    // build src/math.c and produce a library in the output directory
    cc::Build::new().file("src/math.c").compile("math");
}
