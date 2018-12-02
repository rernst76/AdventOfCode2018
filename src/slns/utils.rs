use std::fs::File;
use std::io::prelude::*;

pub fn get_problem_input(path : &str) -> String {
    let mut file = File::open(path).expect("Failed to open problem input file.");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to parse file contents.");

    return contents;
}