extern crate cc;

fn main() {
    cc::Build::new()
        .file("./src/c/math.c")
        .compile("libmath.lib");
    cc::Build::new()
        .file("./src/c/extern.c")
        .flag("-ldlfcn")
        //.flag("-fPIC")
        //.flag("-ldl")
        .flag("-rdynamic")
        .compile("to_extern");
    cc::Build::new()
        .file("./src/c/open_file.c")
        //.flag("-fPIC")
        //.flag("-ldlfcn")
        .flag("-rdynamic")
        .compile("open_file");
}