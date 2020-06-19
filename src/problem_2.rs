use regex::Regex;
use std::cmp;
use std::fs;

pub fn run_problem_2() {
    let filename = "data/problem_2.txt";
    let file_content = fs::read_to_string(filename)
        .expect(&format!("Cannot open file \"{}\"", filename));
    
    let dimensions: Vec<Dimension> = parse_str(&file_content);
    let total_square_feet_paper: u64 = dimensions.iter().map(&paper_surface).sum();
    let total_square_feet_ribbon: u64 = dimensions.iter().map(&ribbon_surface).sum();

    println!("PROBLEM 2");
    println!("  PART 1: {}", total_square_feet_paper);
    println!("  PART 2: {}", total_square_feet_ribbon);
}

type Dimension = (u64, u64, u64);

fn parse_str(content: &str) -> Vec<Dimension> {
    let mut vec: Vec<Dimension> = Vec::new();
    let re = Regex::new(r"(?m)^(\d+)x(\d+)x(\d+)$").unwrap();
    for cap in re.captures_iter(content) {
        let l : u64 = u64::from_str_radix(&cap[1], 10).unwrap();
        let w : u64 = u64::from_str_radix(&cap[2], 10).unwrap();
        let h : u64 = u64::from_str_radix(&cap[3], 10).unwrap();
        vec.push((l, w, h));
    }
    return vec;
}

fn paper_surface(d: &Dimension) -> u64 {
    let area_1 = d.0 * d.1;
    let area_2 = d.0 * d.2;
    let area_3 = d.1 * d.2;
    let square_feet = 2 * area_1 + 2 * area_2 + 2 * area_3;
    let slack_feet = cmp::min(area_1,  cmp::min(area_2, area_3));
    return square_feet + slack_feet;
}

fn ribbon_surface(d: &Dimension) -> u64 {
    let max_perimeter = cmp::max(d.0,  cmp::max(d.1, d.2)) * 2;
    let perimeter = d.0 * 2 + d.1 * 2 + d.2 * 2 - max_perimeter;
    let cubic_feet = d.0 * d.1 * d.2;
    return perimeter + cubic_feet;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_str() {
        assert_eq!(parse_str("20x3x11"), [(20,3,11)]);
        assert_eq!(parse_str("20x3x11\n15x27x5"), [(20,3,11), (15,27,5)]);
    }

    #[test]
    fn test_paper_surface() {
        assert_eq!(paper_surface(&(2,3,4)), 58);
        assert_eq!(paper_surface(&(1,1,10)), 43);
    }

    #[test]
    fn test_ribbon_surface() {
        assert_eq!(ribbon_surface(&(2,3,4)), 34);
        assert_eq!(ribbon_surface(&(1,1,10)), 14);
    }
}
