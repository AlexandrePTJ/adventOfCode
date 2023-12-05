pub fn get_winning_count(line: &str) -> usize {
    let mut count = 0;

    let colon_idx = line.find(':').unwrap();
    let sep_idx = line.find('|').unwrap();

    let card_numbers = common::get_numbers::<i32>(line, Some(colon_idx), Some(sep_idx));
    let our_numbers = common::get_numbers::<i32>(line, Some(sep_idx), None);

    for n in our_numbers {
        if card_numbers.contains(&n) {
            count += 1
        }
    }

    return count;
}

pub fn part1(filename: &str) -> usize {
    let mut sum = 0;
    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(card) = line {
                let n = get_winning_count(card.as_str());
                if n > 0 {
                    sum += 2_usize.pow((n - 1) as u32);
                }
            }
        }
    }
    return sum;
}

pub fn part2(filename: &str) -> usize {
    let mut deck = vec![];

    let mut card_idx: usize = 1;

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(card) = line {
                while deck.len() < card_idx {
                    deck.push(1);
                }

                let n = get_winning_count(card.as_str());

                for i in 1..n + 1 {
                    if deck.len() < card_idx + i {
                        deck.push(deck[card_idx - 1] + 1);
                    } else {
                        deck[card_idx - 1 + i] += deck[card_idx - 1];
                    }
                }

                card_idx += 1;
            }
        }
    }

    return deck.iter().sum();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() { assert_eq!(part1("test_day04.txt"), 13); }

    #[test]
    fn test_part2() { assert_eq!(part2("test_day04.txt"), 30); }
}

fn main() {
    println!("Day04 Part1 = {}", part1("day04.txt"));
    println!("Day04 Part2 = {}", part2("day04.txt"));
}
