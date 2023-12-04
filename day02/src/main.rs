use std::cmp;

fn check_set(line: &str, start_idx: usize) -> (usize, bool) {
    let sub_str = line.get(start_idx..).unwrap();
    let end_idx = sub_str.find(';').or(Option::from(sub_str.len())).unwrap();
    let set_str = sub_str.get(..end_idx).unwrap();

    let mut is_valid = true;

    for take in set_str.split(',') {
        let details: Vec<&str> = take.trim().split(' ').collect();
        let color = details.get(1).unwrap();
        let value = details.get(0).unwrap().parse::<i32>().unwrap();

        match color {
            &"red" => { if value > 12 { is_valid = false; } }
            &"green" => { if value > 13 { is_valid = false; } }
            &"blue" => { if value > 14 { is_valid = false; } }
            _ => {}
        }
    }
    (end_idx + start_idx, is_valid)
}

fn get_game_idx(line: &str) -> i32 {
    let colon_idx = line.find(':').unwrap();
    line.get(5..colon_idx).unwrap().parse::<i32>().unwrap()
}

// red, green, blue
fn get_set(line: &str, start_idx: usize) -> (usize, i32, i32, i32) {
    let sub_str = line.get(start_idx..).unwrap();
    let end_idx = sub_str.find(';').or(Option::from(sub_str.len())).unwrap();
    let set_str = sub_str.get(..end_idx).unwrap();

    let mut red: i32 = 0;
    let mut green: i32 = 0;
    let mut blue: i32 = 0;

    for take in set_str.split(',') {
        let details: Vec<&str> = take.trim().split(' ').collect();
        let color = details.get(1).unwrap();
        let value = details.get(0).unwrap().parse::<i32>().unwrap();

        match color {
            &"red" => { red = value }
            &"green" => { green = value }
            &"blue" => { blue = value }
            _ => {}
        }
    }
    (end_idx + start_idx, red, green, blue)
}

pub fn part1(filename: &str) -> (Vec<i32>, i32) {
    let mut result = vec![];

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(game) = line {
                let colon_idx = game.find(':').unwrap();

                let (mut next_idx, mut is_valid) = check_set(game.as_str(), colon_idx + 2);
                while next_idx < game.len() && is_valid {
                    (next_idx, is_valid) = check_set(game.as_str(), next_idx + 2);
                }

                if is_valid {
                    result.push(get_game_idx(game.as_str()));
                }
            }
        }
    }

    let sum = result.iter().sum();
    (result, sum)
}

pub fn part2(filename: &str) -> (Vec<i32>, i32) {
    let mut result = vec![];

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(game) = line {
                let colon_idx = game.find(':').unwrap();

                let (mut next_idx, mut max_red, mut max_green, mut max_blue) = get_set(game.as_str(), colon_idx + 2);
                while next_idx < game.len() {
                    let (n_next_idx, red, green, blue) = get_set(game.as_str(), next_idx + 2);

                    max_red = cmp::max(red, max_red);
                    max_green = cmp::max(green, max_green);
                    max_blue = cmp::max(blue, max_blue);

                    next_idx = n_next_idx;
                }
                result.push(max_red * max_green * max_blue)
            }
        }
    }

    let sum = result.iter().sum();
    (result, sum)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (result, sum) = part1("test_day02.txt");

        assert_eq!(result, vec![1, 2, 5]);
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_part2() {
        let (result, sum) = part2("test_day02.txt");

        assert_eq!(result, vec![48, 12, 1560, 630, 36]);
        assert_eq!(sum, 2286);
    }
}

fn main() {
    println!("Day02 Part1 = {}", part1("day02.txt").1);
    println!("Day02 Part2 = {}", part2("day02.txt").1);
}
