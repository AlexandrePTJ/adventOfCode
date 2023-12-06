struct ConvertRange {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

impl ConvertRange {
    fn can_convert(&self, status: usize) -> bool {
        self.source_start <= status && status < self.source_start + self.length
    }

    fn convert(&self, status: usize) -> usize {
        self.destination_start + status - self.source_start
    }

    fn from(values: Vec<usize>) -> ConvertRange {
        ConvertRange {
            destination_start: values[0],
            source_start: values[1],
            length: values[2],
        }
    }
}

fn transform(seed: usize, garden: &Vec<Vec<ConvertRange>>) -> usize {
    let mut status = seed;

    for ranges in garden {
        for range in ranges {
            if range.can_convert(status) {
                status = range.convert(status);
                break;
            }
        }
    }

    return status;
}

fn parse_puzzle(filename: &str) -> (Vec<usize>, Vec<Vec<ConvertRange>>) {
    let mut seeds = vec![];
    let mut garden: Vec<Vec<ConvertRange>> = vec![];
    let mut last_idx = 0;

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(line_str) = line {
                if line_str.starts_with("seeds") {
                    seeds = common::get_numbers::<usize>(line_str.as_str(), Some(6), None);
                } else if line_str.ends_with("map:") {
                    garden.push(vec![]);
                    last_idx += 1;
                } else if !line_str.ends_with("map:") && !line_str.trim().is_empty() && !garden.is_empty() {
                    garden[last_idx - 1].push(ConvertRange::from(common::get_numbers(line_str.as_str(), None, None)));
                }
            }
        }
    }
    return (seeds, garden);
}

pub fn part1(filename: &str) -> usize {
    let (seeds, garden) = parse_puzzle(filename);
    let mut smallest = usize::MAX;

    for seed in seeds {
        smallest = std::cmp::min(transform(seed, &garden), smallest);
    }

    return smallest;
}

pub fn part2(filename: &str) -> usize {
    let (seeds, garden) = parse_puzzle(filename);
    let mut smallest = usize::MAX;
    let count = seeds.len() / 2;

    for n in 0..count {
        let start_seed = seeds[n * 2];
        let end_seed = start_seed + seeds[n * 2 + 1];

        for seed in start_seed..end_seed + 1 {
            smallest = std::cmp::min(transform(seed, &garden), smallest);
        }
    }

    return smallest;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() { assert_eq!(part1("test_day05.txt"), 35); }

    #[test]
    fn test_part2() { assert_eq!(part2("test_day05.txt"), 46); }
}

fn main() {
    println!("Day05 Part1 = {}", part1("day05.txt"));
    println!("Day05 Part2 = {}", part2("day05.txt"));
}
