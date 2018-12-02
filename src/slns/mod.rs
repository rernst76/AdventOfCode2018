pub mod utils;

pub mod d1_p1;
pub mod d1_p2;

pub fn solve_day_1() {
    let contents = utils::get_problem_input("ProblemInput/Day1.txt");

    let split = contents.split("\n");

    let vec = split.map(|x| x.trim()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let answer = d1_p1::frequency_drift(&vec);

    println!("The first answer is: {}", answer);

    let answer = d1_p2::frequency_calibrate(&vec);

    println!("The second answer is: {}", answer);
}

pub fn solve_day_2() {
    let contents = utils::get_problem_input("ProblemInput/Day2.txt");

}