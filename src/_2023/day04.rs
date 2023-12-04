fn parse_card(card: &str) -> impl Iterator<Item = (Vec<u32>, Vec<u32>)> + '_ {
    card.lines().map(|line| {
        let (_, numbers) = line.split_once(':').unwrap();
        let (winning_str, our_str) = numbers.split_once('|').unwrap();
        let winning = winning_str
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let our = our_str
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        (winning, our)
    })
}

pub fn part1(input: &str) -> impl ToString {
    parse_card(input)
        .map(|(winning, our)| {
            let mut score = 0;
            for i in our {
                if winning.contains(&i) {
                    if score == 0 {
                        score += 1;
                    } else {
                        score *= 2;
                    }
                }
            }
            score
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl ToString {
    let cards = parse_card(input).collect::<Vec<_>>();

    let mut cards_played_cache = vec![0; cards.len()];

    for card_to_play in (0..cards.len()).rev() {
        let mut cards_played = 1;

        let (winning, our) = &cards[card_to_play];
        let mut score = 0;
        for i in our {
            if winning.contains(i) {
                score += 1;
            }
        }

        for i in 1..=score {
            cards_played += cards_played_cache[card_to_play + i];
        }

        cards_played_cache[card_to_play] = cards_played;
    }

    cards_played_cache.iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 4;

    const EXAMPLE: &str = indoc::indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "13");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "21485");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "30");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "11024379");
    }
}
