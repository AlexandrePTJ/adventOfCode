fn get_next_number(line: &str, start_idx: usize) -> Option<(usize, usize)> {
    let first_number = line.get(start_idx..).unwrap().find(char::is_numeric)?;
    let next_not_number = line.get(first_number..).unwrap().find(|c: char| !c.is_numeric())?;
    Some((first_number, next_not_number))
}

pub fn part1(filename: &str) -> (Vec<i32>, i32) {
    let mut result = vec![];


    if let Ok(mut lines) = common::read_lines(filename) {

        let current_line = lines.next();

        match current_line {
            None => {}
            Some(current_line) => {
                if let Ok(line) = current_line {
                    let (start_idx, end_idx) = get_next_number(line.as_str(), 0).or_else(||Some((line.len(), line.len()))).unwrap();
                }
            }
        }


            // let next_line = lines.next();
            //

            // while start_idx < current_line.len() {}
    }

    let sum = result.iter().sum();
    (result, sum)
}

pub fn part2(filename: &str) {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (result, sum) = part1("test_day03.txt");

        assert_eq!(result, vec![114, 58]);
        assert_eq!(sum, 4361);
    }

    // #[test]
    // fn test_part2() {
    //     let (result, sum) = part2("test_day02.txt");
    //
    //     assert_eq!(result, vec![48, 12, 1560, 630, 36]);
    //     assert_eq!(sum, 2286);
    // }
}

fn main() {
    println!("Day03 Part1 = {}", part1("day03.txt").1);
    // println!("Day03 Part2 = {}", part2("day03.txt").1);
}
