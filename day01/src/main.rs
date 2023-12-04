use std::collections::HashMap;
use std::ops::Mul;

pub fn part1(filename: &str) -> Option<i32> {
    let mut sum = 0;

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(cmd) = line {
                let d = cmd.find(char::is_numeric)?;
                let u = cmd.rfind(char::is_numeric)?;
                sum += cmd.get(d..d + 1)?.parse::<i32>().unwrap() * 10 + cmd.get(u..u + 1)?.parse::<i32>().unwrap();
            }
        }
    }

    return Option::from(sum);
}

pub fn part2(filename: &str) -> Option<i32> {
    let mut sum = 0;

    let keys = HashMap::from([
        ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8),
        ("nine", 9),
        ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5),
        ("6", 6), ("7", 7), ("8", 8), ("9", 9)]);

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(cmd) = line {
                let mut start_idx = cmd.len();
                let mut end_idx = 0;
                let mut start_value = 0;
                let mut end_value = 0;

                for (key, value) in &keys {
                    let start = cmd.find(key);
                    match start {
                        Some(start) => if start < start_idx {
                            start_idx = start;
                            start_value = value.mul(10);
                        },
                        None => {}
                    }

                    let end = cmd.rfind(key);
                    match end {
                        Some(end) => if end >= end_idx {
                            end_idx = end;
                            end_value = value.mul(1);
                        }
                        None => {}
                    }
                }

                sum += start_value + end_value;
            }
        }
    }

    return Option::from(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test_day01_part1.txt").unwrap(), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("test_day01_part2.txt").unwrap(), 281);
    }
}

fn main() {
    println!("Day01 Part1 = {}", part1("day01.txt").unwrap());
    println!("Day01 Part2 = {}", part2("day01.txt").unwrap());
}
