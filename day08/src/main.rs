use std::collections::HashMap;
use regex::Regex;

fn parse_puzzle(filename: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut instructions: Vec<char> = vec![];
    let mut map: HashMap<String, (String, String)> = Default::default();

    let re = Regex::new(r"(?<index>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(line_str) = line {
                if instructions.is_empty() {
                    instructions = line_str.chars().collect();
                } else if let Some(step) = re.captures(line_str.as_str()) {
                    let (_, [index, left, right]) = step.extract();
                    map.insert(index.to_string(), (left.to_string(), right.to_string()));
                }
            }
        }
    }

    return (instructions, map);
}


pub fn part1(filename: &str) -> usize {
    let (instructions, map) = parse_puzzle(filename);

    let mut steps = 0;
    let mut current_instruction_index = 0;
    let mut current_index_str = "AAA";

    while !current_index_str.eq("ZZZ") {
        if current_instruction_index == instructions.len() {
            current_instruction_index = 0;
        }

        let instruction = instructions[current_instruction_index];
        let step = map.get(current_index_str).unwrap();

        current_index_str = if instruction == 'L' { step.0.as_str() } else { step.1.as_str() };

        // dbg!(instruction, step, current_index_str);

        steps += 1;
        current_instruction_index += 1;
    }


    return steps;
}

pub fn part2(filename: &str) -> usize {
    let (instructions, map) = parse_puzzle(filename);

    let mut steps = 0;
    let mut current_instruction_index = 0;
    let mut nodes: Vec<&String> = map.keys().filter(|k| k.ends_with('A')).collect();

    while !nodes.iter().all(|s| s.ends_with('Z')) {
        if current_instruction_index == instructions.len() {
            current_instruction_index = 0;
        }

        let instruction = instructions[current_instruction_index];

        for i in 0..nodes.len() {
            let step = map.get(nodes[i]).unwrap();
            nodes[i] = if instruction == 'L' { &step.0 } else { &step.1 };
        }

        steps += 1;
        current_instruction_index += 1;
    }


    return steps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("test_day08_1.txt"), 2);
        assert_eq!(part1("test_day08_2.txt"), 6);
    }

    #[test]
    fn test_part2() { assert_eq!(part2("test_day08_3.txt"), 6); }
}

fn main() {
    println!("Day08 Part1 = {}", part1("day08.txt"));
    println!("Day08 Part2 = {}", part2("day08.txt"));
}
