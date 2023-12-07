use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    (time, distance)
}

fn ways_to_beat(time: usize, distance: usize) -> usize {
    (0..=time)
        .into_par_iter()
        .map(|i| if i * (time - i) > distance { 1 } else { 0 })
        .sum()
}

pub fn part1(input: &str) -> impl ToString {
    let (time, distance) = parse_input(input);

    time.iter()
        .zip(distance.iter())
        .map(|(time, distance)| ways_to_beat(*time, *distance))
        .product::<usize>()
}

pub fn part2(input: &str) -> impl ToString {
    let (time, distance) = parse_input(input);

    let time = time.iter().join("").parse::<usize>().unwrap();
    let distance = distance.iter().join("").parse::<usize>().unwrap();

    ways_to_beat(time, distance)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 6;

    const EXAMPLE: &str = indoc::indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "288");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "608902");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "71503");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "46173809");
    }
}
