extern crate pkg_config;
extern crate gcc;

fn main() {
    let lib = pkg_config::Config::new()
                                 .atleast_version("2.24.0")
                                 .probe("gio-2.0")
                                 .unwrap();

    let mut builder = gcc::Build::new();

    for i in &lib.include_paths {
        builder.include(i);
    }

    builder.include("src");
    builder.file("gvdb/gvdb-builder.c");
    builder.file("gvdb/gvdb-reader.c");
    builder.compile("libgvdb.a");
}
