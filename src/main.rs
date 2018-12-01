use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let mut file = File::open("ProblemInput/Day1Part1.txt").expect("Failed to open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("that didn't work");

    println!("{}", contents);


}

