use ahash::HashMap;

#[derive(Debug)]
struct Hand {
    cards: Vec<u8>,
    map: HashMap<u8, u8>,
    bid: usize,
}

fn run_cmp(left: bool, right: bool, left_hand: &Hand, right_hand: &Hand) -> std::cmp::Ordering {
    match (left, right) {
        (true, true) => left_hand.cmp_orders(right_hand),
        (true, false) => std::cmp::Ordering::Greater,
        (false, true) => std::cmp::Ordering::Less,
        (false, false) => std::cmp::Ordering::Equal,
    }
}

impl Hand {
    fn compare_cards(&self, other: &Self) -> std::cmp::Ordering {
        let five_compare = run_cmp(
            self.is_five_of_a_kind(),
            other.is_five_of_a_kind(),
            self,
            other,
        );
        if five_compare != std::cmp::Ordering::Equal {
            return five_compare;
        }

        let four_compare = run_cmp(
            self.is_four_of_a_kind(),
            other.is_four_of_a_kind(),
            self,
            other,
        );
        if four_compare != std::cmp::Ordering::Equal {
            return four_compare;
        }

        let full_compare = run_cmp(self.is_full_house(), other.is_full_house(), self, other);
        if full_compare != std::cmp::Ordering::Equal {
            return full_compare;
        }

        let three_compare = run_cmp(
            self.is_three_of_a_kind(),
            other.is_three_of_a_kind(),
            self,
            other,
        );
        if three_compare != std::cmp::Ordering::Equal {
            return three_compare;
        }

        let two_compare = run_cmp(self.is_two_pairs(), other.is_two_pairs(), self, other);
        if two_compare != std::cmp::Ordering::Equal {
            return two_compare;
        }

        let one_compare = run_cmp(self.is_one_pair(), other.is_one_pair(), self, other);
        if one_compare != std::cmp::Ordering::Equal {
            return one_compare;
        }

        self.cmp_orders(other)
    }

    fn cmp_orders(&self, other: &Self) -> std::cmp::Ordering {
        for i in 0..5 {
            let left = self.cards[i];
            let right = other.cards[i];
            let compare = left.cmp(&right);
            if compare != std::cmp::Ordering::Equal {
                return compare;
            }
        }
        panic!("invalid hands");
    }

    fn is_five_of_a_kind(&self) -> bool {
        self.map.values().any(|&v| v == 5)
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.map.values().any(|&v| v == 4)
    }

    fn is_full_house(&self) -> bool {
        self.map.values().any(|&v| v == 3) && self.map.values().any(|&v| v == 2)
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.map.values().any(|&v| v == 3)
    }

    fn is_two_pairs(&self) -> bool {
        self.map.values().filter(|&&v| v == 2).count() == 2
    }

    fn is_one_pair(&self) -> bool {
        self.map.values().any(|&v| v == 2)
    }
}

fn sum_hands(hands: &[Hand]) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid)
        .sum::<usize>()
}

fn char_to_u8(c: char, jokers: bool) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' if !jokers => 11,
        'J' => 0,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

fn parse_input(input: &str, jokers: bool) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards = cards
                .chars()
                .map(|c| char_to_u8(c, jokers))
                .collect::<Vec<_>>();
            let bid = bid.parse::<usize>().unwrap();

            let mut map = HashMap::default();
            let mut joker_count = 0;
            for &c in &cards {
                if c == 0 && jokers {
                    joker_count += 1;
                } else {
                    *map.entry(c).or_insert(0) += 1;
                }
            }

            // add jokers to the card with the highest value
            if jokers {
                let max_value = *map.values().max().unwrap_or(&0);
                match map.iter_mut().find(|(_, v)| **v == max_value) {
                    Some((_, v)) => *v += joker_count,
                    None => {
                        map.insert(0, joker_count);
                    }
                }
            }

            Hand { cards, bid, map }
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> impl ToString {
    let mut hands = parse_input(input, false);
    hands.sort_unstable_by(|a, b| a.compare_cards(b));
    sum_hands(&hands)
}

pub fn part2(input: &str) -> impl ToString {
    let mut hands = parse_input(input, true);
    hands.sort_unstable_by(|a, b| a.compare_cards(b));
    sum_hands(&hands)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 7;

    const EXAMPLE: &str = indoc::indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "6440");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "250232501");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "5905");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "249138943");
    }
}
