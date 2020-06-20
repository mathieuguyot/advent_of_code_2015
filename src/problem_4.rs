use std::fs;
use md5;

pub fn run_problem_4() {
    let file_name = "data/problem_4.txt";
    let file_content = fs::read_to_string(file_name)
        .expect(&format!("Cannot open file \"{}\"", file_name));

    let part_1_res : u64 = find_index_hash(&file_content, "00000");
    let part_2_res : u64 = find_index_hash(&file_content, "000000");

    assert_eq!(part_1_res, 346386);
    assert_eq!(part_2_res, 9958218);

    println!("PROBLEM 4");
    println!("  PART 1: {}", part_1_res);
    println!("  PART 2: {}", part_2_res);
}

fn find_index_hash(key: &str, wanted_beggining: &str) -> u64 {
    let mut index : u64 = 0;
    loop {
        let computed_key : String = key.to_string() + &index.to_string();
        let res = format!("{:x}", md5::compute(computed_key)); 
        if res.starts_with(wanted_beggining) {
            break;
        }
        else {
            index += 1;
        }
    }
    return index;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_index_hash() {
        assert_eq!(find_index_hash("abcdef", "00000"), 609043);
        assert_eq!(find_index_hash("pqrstuv", "00000"), 1048970);
    }
}
