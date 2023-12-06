use std::collections::HashMap;
use std::ops::Deref;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    line: i32,
    column: i32,
}

struct StarValues(i32, i32);

fn get_next_number(line: &str, start_idx: usize) -> (Option<&str>, usize, usize) {
    if let Some(sl) = line.get(start_idx..) {
        if let Some(sl_start_idx) = sl.find(char::is_numeric) {
            let num_start = start_idx + sl_start_idx;
            let ssl = line.get(num_start..).unwrap();
            let num_end =
                if let Some(sl_next_not_number_idx) = ssl.find(|c: char| !c.is_numeric()) {
                    num_start + sl_next_not_number_idx - 1
                } else {
                    line.len() - 1
                };
            let num_str = line.get(num_start..num_end + 1);

            return (num_str, num_start, num_end);
        }
    }
    return (None, start_idx, line.len());
}

fn check_dot_only(line: &str, start_idx: usize, end_idx: usize) -> bool {
    if let Some(sub) = line.get(start_idx..end_idx) {
        return sub.chars().all(|c| c == '.');
    }
    return true;
}

fn check_line(current: &str, previous: &str, next: &str) -> Vec<i32>
{
    let mut result = vec![];

    let dot = '.';
    let (mut number, mut start_idx, mut end_idx) = get_next_number(current, 0);

    while let Some(n) = number {
        let mut has_adjacent = false;
        let mut extent_start_idx = 0;

        if start_idx > 0 {
            if current.chars().nth(start_idx - 1).unwrap() != dot { has_adjacent = true; }
            extent_start_idx = start_idx - 1;
        }
        if end_idx < current.len() - 1 {
            if current.chars().nth(end_idx + 1).unwrap() != dot { has_adjacent = true; }
        }

        for line in [previous, next] {
            if !check_dot_only(line, extent_start_idx, std::cmp::min(end_idx + 2, line.len())) {
                has_adjacent = true;
            }
        }

        if has_adjacent {
            if let Ok(v) = n.parse::<i32>() {
                result.push(v);
            }
        }

        (number, start_idx, end_idx) = get_next_number(current, end_idx + 1);
    }

    return result;
}

fn insert_or_get(line: i32, column: i32, stars: &mut HashMap<Position, StarValues>) -> &StarValues {
    let position = Position { line, column };
    let v = stars.get(&position);

    match v {
        None => {
            let values = StarValues(0, 0);
            return *stars.insert(position, values);
        }
        Some(_) => { v }
    }
}

fn check_stars(current: &str, previous: &str, next: &str, stars: &mut HashMap<Position, StarValues>, current_idx: i32) {
    let (mut number, mut start_idx, mut end_idx) = get_next_number(current, 0);

    while let Some(n) = number {
        let mut has_star = false;
        let mut extent_start_idx = 0;
        let mut star: Option<Position> = None;

        if start_idx > 0 {
            if current.chars().nth(start_idx - 1).unwrap() == '*' { star = Some(Position { line: current_idx, column: (start_idx - 1) as i32 }); }
            extent_start_idx = start_idx - 1;
        }
        if end_idx < current.len() - 1 {
            if current.chars().nth(end_idx + 1).unwrap() == '*' { has_star = true; }
        }

        // for line in [previous, next] {
        //     if !check_dot_only(line, extent_start_idx, std::cmp::min(end_idx + 2, line.len())) {
        //         has_adjacent = true;
        //     }
        // }

        if has_star {
            // if let Ok(v) = n.parse::<i32>() {
            //     result.push(v);
            // }
        }

        (number, start_idx, end_idx) = get_next_number(current, end_idx + 1);
    }
}

pub fn part1(filename: &str) -> (Vec<i32>, i32) {
    let mut result = vec![];


    if let Ok(lines) = common::read_lines(filename) {
        let mut current_line = "".to_string();
        let mut previous_line = "".to_string();
        let mut next_line: String;

        for row in lines {
            if let Ok(line) = row {
                next_line = line;

                if !current_line.is_empty() {
                    let mut line_res = check_line(current_line.as_str(), previous_line.as_str(), next_line.as_str());
                    result.append(&mut line_res);
                }

                previous_line = current_line;
                current_line = next_line;
            }
        }

        let mut line_res = check_line(current_line.as_str(), previous_line.as_str(), "");
        result.append(&mut line_res);
    }

    let sum = result.iter().sum();
    (result, sum)
}

pub fn part2(filename: &str) -> (Vec<i32>, i32) {
    let mut result = vec![];

    if let Ok(lines) = common::read_lines(filename) {
        let mut current_line = "".to_string();
        let mut previous_line = "".to_string();
        let mut next_line: String;
        let mut line_idx = 0;
        let mut stars: HashMap<Position, StarValues>;

        for row in lines {
            if let Ok(line) = row {
                next_line = line;

                if !current_line.is_empty() {
                    let mut line_res = check_stars(current_line.as_str(), previous_line.as_str(), next_line.as_str(), &stars, line_idx);
                    result.append(&mut line_res);
                }

                previous_line = current_line;
                current_line = next_line;
            }
            line_idx += 1;
        }

        let mut line_res = check_line(current_line.as_str(), previous_line.as_str(), "");
        result.append(&mut line_res);
    }

    let sum = result.iter().sum();
    (result, sum)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (result, sum) = part1("test_day03.txt");

        assert_eq!(result, vec![467, 35, 633, 617, 592, 755, 664, 598]);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_part2() {
        let (result, sum) = part2("test_day03.txt");

        assert_eq!(result, vec![16345, 451490]);
        assert_eq!(sum, 467835);
    }
}

fn main() {
    println!("Day03 Part1 = {}", part1("day03.txt").1);
    // println!("Day03 Part2 = {}", part2("day03.txt").1);
}
