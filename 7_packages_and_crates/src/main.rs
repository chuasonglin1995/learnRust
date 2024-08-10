// crate = binary crate or library crate
// Binary crate: Compiles to an executable, has a main function
// Library crate: functions that other programs can use

// Package is a bundle of one or more crates
// contains a cargo.toml file that describes how to build those crates

// if a package contains src/main.rs and src/lib.rs, it has two crates
// a package can have multiple binary creates by placing files in the src/bin directory, each file is a binary crate

fn main() {
    println!("Hello, world!");
}
