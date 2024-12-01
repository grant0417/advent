use std::{fmt::Display, u64};

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let first = iter.next().unwrap().parse::<u64>().unwrap();
            let second = iter.next().unwrap().parse::<u64>().unwrap();
            (first, second)
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> impl Display {
    let (mut lista, mut listb): (Vec<u64>, Vec<u64>) = parse_input(input).into_iter().unzip();
    lista.sort();
    listb.sort();

    let mut dist = 0;
    for i in 0..lista.len() {
        dist += lista[i].abs_diff(listb[i]);
    }
    dist
}

pub fn part2(input: &str) -> impl Display {
    let (lista, listb): (Vec<u64>, Vec<u64>) = parse_input(input).into_iter().unzip();
    let mut val = 0;

    for a in lista {
        let count = listb.iter().filter(|&&b| a == b).count() as u64;
        val += count * a
    }

    val
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 1;

    const EXAMPLE: &str = indoc::indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3 
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "11");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "2113135");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "31");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "19097157");
    }
}
