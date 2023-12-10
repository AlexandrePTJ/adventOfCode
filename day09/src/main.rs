fn get_next_value(sequence: Vec<i32>) -> i32 {
    if sequence.iter().all(|v| v.eq(&0)) {
        return 0;
    }

    let mut diff = vec![];
    for v in 0..sequence.len() - 1 {
        diff.push(sequence[v + 1] - sequence[v]);
    }

    let new_value = get_next_value(diff);

    return sequence.last().unwrap() + new_value;
}

fn get_previous_value(sequence: Vec<i32>) -> i32 {
    if sequence.iter().all(|v| v.eq(&0)) {
        return 0;
    }

    let mut diff = vec![];
    for v in 0..sequence.len() - 1 {
        diff.push(sequence[v + 1] - sequence[v]);
    }

    let new_value = get_previous_value(diff);

    return sequence.first().unwrap() - new_value;
}

pub fn part1(filename: &str) -> i64 {
    let mut sum: i64 = 0;

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(line_str) = line {
                let numbers = common::get_numbers::<i32>(line_str.as_str(), None, None);
                sum += get_next_value(numbers) as i64;
            }
        }
    }

    return sum;
}

pub fn part2(filename: &str) -> i64 {
    let mut sum: i64 = 0;

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(line_str) = line {
                let numbers = common::get_numbers::<i32>(line_str.as_str(), None, None);
                sum += get_previous_value(numbers) as i64;
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(get_next_value(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(get_next_value(vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(get_next_value(vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_part2() {
        assert_eq!(get_previous_value(vec![10, 13, 16, 21, 30, 45]), 5);
    }
}

fn main() {
    println!("Day09 Part1 = {}", part1("day09.txt"));
    println!("Day09 Part2 = {}", part2("day09.txt"));
}
