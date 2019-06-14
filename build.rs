extern crate cc;

fn main() {
    cc::Build::new()
        .file("/home/pokemon/ThRustGorge/src/c/math.c")
        .compile("libmath.a");
}