pub mod utils;

pub mod d1_p1;
pub mod d1_p2;
pub mod d2_p1;
pub mod d2_p2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn solve_day_2() {
    let contents = utils::get_problem_input("ProblemInput/Day2.txt");

    let split = contents.split("\n");
    let vec: Vec<&str> = split.map(|x| x.trim()).collect();

    let checksum = d2_p1::calculate_checksum(&vec);

    println!("checksum is: {}", checksum);

    let val = d2_p2::find_boxes(&vec).unwrap();

    println!("The boxes are {} and {}", val.0, val.1);
}

#[allow(dead_code)]
pub fn solve_day_3() {
    let contents = utils::get_problem_input("ProblemInput/Day3.txt");

    let split = contents.split("\n");
    let squares: Vec<&str> = split.map(|x| x.trim()).collect();

    let answers = d3::find_num_overlapping_and_best_claim(&squares);

    println!("The number of overlapping squares is: {}", answers.0);
    println!("The best claim is: {}", answers.1);
}

#[allow(dead_code)]
pub fn solve_day_4() {
    let contents = utils::get_problem_input("ProblemInput/Day4.txt");
    let mut entries: Vec<&str> = contents.split("\n").collect();

    d4::solve_day_4(&mut entries);
}

#[allow(dead_code)]
pub fn solve_day_5() {
    let contents = utils::get_problem_input("ProblemInput/Day5.txt");
    d5::run_reaction(&contents);
}

pub fn solve_day_6() {
    let contents = utils::get_problem_input("ProblemInput/Day6.txt");
    d6::day_6_solution(&contents);
}
