use std::fs::File;
use std::io::prelude::*;

mod slns;

fn main() {

    let mut file = File::open("ProblemInput/Day1Part1.txt").expect("Failed to open file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to parse file contents");

    let split = contents.split("\n");

    let vec = split.map(|x| x.trim()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let answer = slns::frequency_drift(vec);

    println!("The first answer is: {}", answer);




}

