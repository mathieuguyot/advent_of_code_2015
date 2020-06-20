use std::fs;

pub fn run_problem_1() {
    let file_name = "data/problem_1.txt";
    let file_content = fs::read_to_string(file_name)
        .expect(&format!("Cannot open file \"{}\"", file_name));
    
    let (floor_counter, basement_index) = floor_computer(&file_content);

    assert_eq!(floor_counter, 232);
    assert_eq!(basement_index.unwrap(), 1783);

    println!("PROBLEM 1");
    println!("  PART 1: {}", floor_counter);
    println!("  PART 2: {}", basement_index.unwrap());
}

fn floor_computer(data: &str) -> (i64, Option<usize>) {
    let mut floor_counter : i64 = 0;
    let mut basement_index : Option<usize> = None;
    for (i, c) in data.chars().enumerate() {
        if c == '(' {
            floor_counter += 1;
        } else if c == ')' {
            floor_counter -= 1;
        }
        if floor_counter == -1 && basement_index.is_none() {
            basement_index = Some(i + 1);
        }
    }
    return (floor_counter, basement_index);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_computer() {
        assert_eq!(floor_computer("(())"), (0, None));
        assert_eq!(floor_computer("()()"), (0, None));
        assert_eq!(floor_computer("((("), (3, None));
        assert_eq!(floor_computer("(()(()("), (3, None));
        assert_eq!(floor_computer("))((((("), (3, Some(1)));
        assert_eq!(floor_computer(")"), (-1, Some(1)));
        assert_eq!(floor_computer("()())"), (-1, Some(5)));
    }
}
