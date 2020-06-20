use std::fs;

pub fn run_problem_5() {
    let file_name = "data/problem_5.txt";
    let file_content = fs::read_to_string(file_name)
        .expect(&format!("Cannot open file \"{}\"", file_name));

    println!("PROBLEM 5");
    println!("  PART 1: {}", 0);
    println!("  PART 2: {}", 0);
}