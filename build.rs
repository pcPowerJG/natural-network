extern crate cc;

fn main() {
    cc::Build::new()
        .file("./src/c/math.c")
        .compile("libmath.a");
    cc::Build::new()
        .file("./src/c/extern.c")
        .flag("-fPIC")
        .flag("-ldl")
        .flag("-rdynamic")
        .compile("libto_extern.a");
}
