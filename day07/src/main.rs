use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    WeakJoker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    As,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    Three,
    House,
    Four,
    Five,
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}


fn card_from_char(c: char, use_joker: bool) -> Card {
    match c {
        'A' => Card::As,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => if use_joker { Card::WeakJoker } else { Card::Joker },
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("Invalid char")
    }
}

fn hand_type_from_cards(cards: &Vec<Card>, use_joker: bool) -> HandType {
    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort();
    sorted_cards.reverse();

    let mut dedup_cards = sorted_cards.to_vec();
    dedup_cards.dedup();

    let has_joker = dedup_cards.iter().any(|c| c == &Card::WeakJoker);

    let hand_type = match dedup_cards.len() {
        1 => HandType::Five,
        2 => {
            if sorted_cards[1] == sorted_cards[3] {
                HandType::Four
            } else { HandType::House }
        }
        3 => {
            if sorted_cards[0] == sorted_cards[2] || sorted_cards[1] == sorted_cards[3] || sorted_cards[2] == sorted_cards[4] {
                HandType::Three
            } else { HandType::TwoPair }
        }
        4 => HandType::OnePair,
        5 => HandType::High,
        _ => panic!("Invalid hand")
    };

    if use_joker && has_joker && hand_type < HandType::Five {
        for replacement_card in dedup_cards {
            if replacement_card != Card::WeakJoker {
                let mut new_cards: Vec<Card> = vec![];
                for card in cards {
                    new_cards.push(if card == &Card::WeakJoker { replacement_card } else { *card });
                }
                let new_hand_type = hand_type_from_cards(&new_cards, false);
                if new_hand_type >= hand_type {
                    return new_hand_type;
                }
            }
        }
    }

    return hand_type;
}

impl Hand {
    pub fn from(line: &str, use_joker: bool) -> Hand {
        let cards = line.chars().into_iter().map(|c| card_from_char(c, use_joker)).collect();
        let hand_type = hand_type_from_cards(&cards, use_joker);
        Hand { cards, hand_type }
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_order = self.hand_type.cmp(&other.hand_type);
        if hand_type_order == Ordering::Equal {
            for n in 0..5 {
                let card_order = self.cards[n].cmp(&other.cards[n]);
                if card_order != Ordering::Equal {
                    return card_order;
                }
            }
        }
        return hand_type_order;
    }
}

fn compute_score(game: &BTreeMap<Hand, usize>) -> usize {
    let mut sum = 0;
    let mut rank = 1;
    for h in game {
        sum += h.1 * rank;
        rank += 1;
    }
    return sum;
}

pub fn part1(filename: &str) -> usize {
    let mut game: BTreeMap<Hand, usize> = Default::default();

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(line_str) = line {
                if let Some(sep) = line_str.find(' ') {
                    game.insert(
                        Hand::from(line_str.get(..sep).unwrap().trim(), false),
                        line_str.get(sep..).unwrap().trim().parse().unwrap(),
                    );
                }
            }
        }
    }

    return compute_score(&game);
}

pub fn part2(filename: &str) -> usize {
    let mut game: BTreeMap<Hand, usize> = Default::default();

    if let Ok(lines) = common::read_lines(filename) {
        for line in lines {
            if let Ok(line_str) = line {
                if let Some(sep) = line_str.find(' ') {
                    game.insert(
                        Hand::from(line_str.get(..sep).unwrap().trim(), true),
                        line_str.get(sep..).unwrap().trim().parse().unwrap(),
                    );
                }
            }
        }
    }

    return compute_score(&game);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() { assert_eq!(part1("test_day07.txt"), 6440); }

    #[test]
    fn test_part2() { assert_eq!(part2("test_day07.txt"), 5905); }
}

fn main() {
    println!("Day07 Part1 = {}", part1("day07.txt"));
    println!("Day07 Part2 = {}", part2("day07.txt")); // <should be inf 249027203
}
