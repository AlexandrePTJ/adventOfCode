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

fn transform(statuses: Vec<usize>, ranges: &Vec<ConvertRange>) -> Vec<usize> {
    let mut new_statuses = vec![];
    for status in statuses {
        let mut new_status = status;
        for range in ranges {
            if range.can_convert(status) {
                new_status = range.convert(status);
            }
        }
        new_statuses.push(new_status);
    }
    return new_statuses;
}

pub fn part1(filename: &str) -> usize {
    let mut seeds = vec![];
    let mut ranges: Vec<ConvertRange> = vec![];

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(line_str) = line {
                if line_str.starts_with("seeds") {
                    seeds = common::get_numbers::<usize>(line_str.as_str(), Some(6), None);
                } else if line_str.trim().is_empty() {
                    if !ranges.is_empty() {
                        seeds = transform(seeds, ranges.as_ref());
                        ranges.clear();
                    }
                } else if !line_str.ends_with("map:") {
                    ranges.push(ConvertRange::from(common::get_numbers(line_str.as_str(), None, None)));
                }
            }
        }
    }

    if !ranges.is_empty() {
        seeds = transform(seeds, ranges.as_ref());
        ranges.clear();
    }

    return *seeds.iter().min().unwrap();
}

pub fn part2(_filename: &str) -> usize {
    return 0;
    // let mut seeds = vec![];
    // let mut ranges: Vec<ConvertRange> = vec![];
    //
    // if let Ok(lines) = common::read_lines(filename) {
    //     for line in lines {
    //         if let Ok(line_str) = line {
    //             if line_str.starts_with("seeds") {
    //                 let seeds_ranges = common::get_numbers::<usize>(line_str.as_str(), Some(6), None);
    //             } else if line_str.trim().is_empty() {
    //                 if !ranges.is_empty() {
    //                     seeds = transform(seeds, ranges.as_ref());
    //                     ranges.clear();
    //                 }
    //             } else if !line_str.ends_with("map:") {
    //                 ranges.push(ConvertRange::from(common::get_numbers(line_str.as_str(), None, None)));
    //             }
    //         }
    //     }
    // }
    //
    // if !ranges.is_empty() {
    //     seeds = transform(seeds, ranges.as_ref());
    //     ranges.clear();
    // }
    //
    // return *seeds.iter().min().unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() { assert_eq!(part1("test_day05.txt"), 35); }

    // #[test]
    // fn test_part2() { assert_eq!(part2("test_day05.txt"), 46); }
}

fn main() {
    println!("Day05 Part1 = {}", part1("day05.txt"));
    // println!("Day05 Part2 = {}", part2("day05.txt"));
}
