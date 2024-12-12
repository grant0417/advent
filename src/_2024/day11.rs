use crate::prelude::*;

type Map = HashMap<u64, usize>;

fn parse_input(input: &str) -> Map {
    input
        .split_whitespace()
        .map(|i| i.parse().unwrap())
        .counts()
        .into_iter()
        .collect()
}

fn step(arr: &Map, new: &mut Map) {
    new.clear();
    for (k, v) in arr {
        match k {
            0 => {
                *new.entry(1).or_insert(0) += *v;
            }
            s if s.ilog10() % 2 == 1 => {
                let base = (s.ilog10() / 2) + 1;
                let left = s / 10u64.pow(base);
                let right = s % 10u64.pow(base);
                *new.entry(left).or_insert(0) += *v;
                *new.entry(right).or_insert(0) += *v;
            }
            s => {
                *new.entry(*s * 2024).or_insert(0) += *v;
            }
        }
    }
}

fn run(mut stones: Map, steps: usize) -> usize {
    let mut new = HashMap::default();
    for _ in 0..steps {
        step(&stones, &mut new);
        (new, stones) = (stones, new)
    }
    stones.values().sum()
}

pub fn part1(input: &str) -> impl Display {
    let stones = parse_input(input);
    run(stones, 25)
}

pub fn part2(input: &str) -> impl Display {
    let stones = parse_input(input);
    run(stones, 75)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 11;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "55312");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "190865");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "225404711855335");
    }
}
