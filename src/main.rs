use std::collections::HashSet;

mod problem_1;
mod problem_2;
mod problem_3;
mod problem_4;
mod problem_5;

fn main() {
    let tests_to_run: HashSet<u64> = [0,1,2,3,5].iter().cloned().collect();

    if tests_to_run.contains(&1) {
        problem_1::run_problem_1();
    }

    if tests_to_run.contains(&2) {
        problem_2::run_problem_2();
    }

    if tests_to_run.contains(&3) {
        problem_3::run_problem_3();
    }

    if tests_to_run.contains(&4) {
        problem_4::run_problem_4();
    }

    if tests_to_run.contains(&5) {
        problem_5::run_problem_5();
    }
}
