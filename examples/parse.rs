extern crate java_parser;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use java_parser::CompilationUnit;

fn main() {
    let paths: Vec<_> = env::args().skip(1).collect();

    for path in paths {
        let mut file = File::open(&path).expect("Couldn't open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Couldn't read file");

        println!(
            "{}, {:#?}",
            path,
            CompilationUnit::parse(contents).expect("Couldn't parse file")
        );
    }
}
