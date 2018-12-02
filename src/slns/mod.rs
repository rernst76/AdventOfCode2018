pub mod utils;

pub mod d1_p1;
pub mod d1_p2;
pub mod d2_p1;
pub mod d2_p2;

pub fn solve_day_1() {
    let contents = utils::get_problem_input("ProblemInput/Day1.txt");

    let split = contents.split("\n");

    let vec = split
        .map(|x| x.trim())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let answer = d1_p1::frequency_drift(&vec);

    println!("The first answer is: {}", answer);

    let answer = d1_p2::frequency_calibrate(&vec);

    println!("The second answer is: {}", answer);
}

pub fn solve_day_2() {
    let contents = utils::get_problem_input("ProblemInput/Day2.txt");

    let split = contents.split("\n");
    let vec: Vec<&str> = split.map(|x| x.trim()).collect();

    let checksum = d2_p1::calculate_checksum(&vec);

    println!("checksum is: {}", checksum);

    let val = d2_p2::find_boxes(&vec).unwrap();

    println!("The boxes are {} and {}", val.0, val.1);
}