pub struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    pub fn compute_distances(&self) -> usize {
        let mut distances = 0;
        for hold in 1..self.time + 1 {
            let remaining = self.time - hold;
            let distance_reached = remaining * hold;
            if distance_reached > self.distance {
                distances += 1;
            }
        }
        return distances;
    }
}

fn get_races() -> Vec<Race> {
    vec![
        Race { time: 40, distance: 219 },
        Race { time: 81, distance: 1012 },
        Race { time: 77, distance: 1365 },
        Race { time: 72, distance: 1089 },
    ]
}


pub fn part1(races: Vec<Race>) -> usize {
    let mut n = 0;

    for race in races {
        let distances = race.compute_distances();
        n = if n == 0 { distances } else { n * distances };
    }

    return n;
}

pub fn part2(race: Race) -> usize { race.compute_distances() }

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_races() -> Vec<Race> {
        vec![
            Race { time: 7, distance: 9 },
            Race { time: 15, distance: 40 },
            Race { time: 30, distance: 200 },
        ]
    }

    #[test]
    fn test_part1() { assert_eq!(part1(get_test_races()), 288); }

    #[test]
    fn test_part2() { assert_eq!(part2(Race { time: 71530, distance: 940200 }), 71503); }
}

fn main() {
    println!("Day05 Part1 = {}", part1(get_races()));
    println!("Day05 Part2 = {}", part2(Race { time: 40817772, distance: 219101213651089 }));
}
