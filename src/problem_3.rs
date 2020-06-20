use std::fs;
use std::collections::HashSet;

pub fn run_problem_3() {
    let file_name = "data/problem_3.txt";
    let file_content = fs::read_to_string(file_name)
        .expect(&format!("Cannot open file \"{}\"", file_name));

    let direction: Vec<Direction> = parse_direction_file(&file_content);
    let res_part_1 = delivered_houses_number(&direction);
    let res_part_2 = delivered_houses_number_with_robo(&direction);

    assert_eq!(res_part_1, 2081);
    assert_eq!(res_part_2, 2341);
    
    println!("PROBLEM 3");
    println!("  PART 1: {}", res_part_1);
    println!("  PART 2: {}", res_part_2);
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

type Position = (i64, i64);

fn str_to_direction(c: char) -> Option<Direction> {
    let direction: Option<Direction> = match c {
        '<' => Some(Direction::WEST),
        '>' => Some(Direction::EAST),
        'v' => Some(Direction::SOUTH),
        '^' => Some(Direction::NORTH),
        _   => None
    };
    return direction;
}

fn parse_direction_file(data: &str) -> Vec<Direction> {
    let mut directions: Vec<Direction> = Vec::new();
    for (_, c) in data.chars().enumerate() {
        match str_to_direction(c) {
            Some(direction) => directions.push(direction),
            None => ()
        }
    }
    return directions;
}

fn move_in_direction(position: &Position,
                     visited_positions: &mut HashSet<Position>,
                     direction: Direction) -> Position
{
    let mut new_position: Position = *position;
    match direction {
        Direction::EAST  => new_position.0 += 1,
        Direction::WEST  => new_position.0 -= 1,
        Direction::SOUTH => new_position.1 -= 1,
        Direction::NORTH => new_position.1 += 1,
    }
    visited_positions.insert(new_position);
    return new_position;
}

fn delivered_houses_number(directions: &Vec<Direction>) -> usize {
    let mut position: Position = (0,0);
    let mut visited_houses: HashSet<Position> = [position].iter().cloned().collect();

    for direction in directions {
        position = move_in_direction(&position, &mut visited_houses, *direction);
    }

    return visited_houses.len();
}

fn delivered_houses_number_with_robo(directions: &Vec<Direction>) -> usize {
    let mut position_santa: Position = (0,0);
    let mut position_robo: Position = (0,0);
    let mut visited_houses: HashSet<Position> = [position_santa].iter().cloned().collect();

    let mut i: usize = 0;
    loop {
        if i < directions.len() {
            position_santa = move_in_direction(&position_santa, &mut visited_houses, directions[i]);
            i += 1;
        } else { break; }
        if i < directions.len() {
            position_robo = move_in_direction(&position_robo, &mut visited_houses, directions[i]);
            i += 1;
        } else { break; }
    }

    return visited_houses.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_direction() {
        assert_eq!(str_to_direction('<'), Some(Direction::WEST));
        assert_eq!(str_to_direction('>'), Some(Direction::EAST));
        assert_eq!(str_to_direction('v'), Some(Direction::SOUTH));
        assert_eq!(str_to_direction('^'), Some(Direction::NORTH));
        assert_eq!(str_to_direction('P'), None);
    }

    #[test]
    fn test_parse_direction_file() {
        assert_eq!(parse_direction_file(&"<^>v"), [Direction::WEST, 
                                                   Direction::NORTH, 
                                                   Direction::EAST, 
                                                   Direction::SOUTH]);
        assert_eq!(parse_direction_file(&""), []);
        assert_eq!(parse_direction_file(&"<"), [Direction::WEST]);
    }

    #[test]
    fn test_move_in_direction() {
        let mut visited_positions: HashSet<Position> = [(0,0)].iter().cloned().collect();
        assert_eq!(move_in_direction(&(0,0), &mut visited_positions, Direction::NORTH), (0,1));
        assert_eq!(visited_positions.len(), 2);
        assert_eq!(visited_positions.contains(&(0,1)), true);
    }

    #[test]
    fn test_delivered_houses_number() {
        assert_eq!(delivered_houses_number(&parse_direction_file(">")), 2);
        assert_eq!(delivered_houses_number(&parse_direction_file("^>v<")), 4);
        assert_eq!(delivered_houses_number(&parse_direction_file("^v^v^v^v^v")), 2);
    }

    #[test]
    fn test_delivered_houses_number_with_robo() {
        assert_eq!(delivered_houses_number_with_robo(&parse_direction_file("^v")), 3);
        assert_eq!(delivered_houses_number_with_robo(&parse_direction_file("^>v<")), 3);
        assert_eq!(delivered_houses_number_with_robo(&parse_direction_file("^v^v^v^v^v")), 11);
    }
}
