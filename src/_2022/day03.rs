use itertools::Itertools as _;

fn priority(c: char) -> u32 {
    if ('a'..='z').contains(&c) {
        c as u32 - 'a' as u32 + 1
    } else if ('A'..='Z').contains(&c) {
        c as u32 - 'A' as u32 + 27
    } else {
        panic!("invalid char: {c}");
    }
}

pub fn part1(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let half = line.len() / 2;
            let (left, right) = line.split_at(half);

            left.chars()
                .unique()
                .map(|c| if right.contains(c) { priority(c) } else { 0 })
                .sum::<u32>()
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl ToString {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut a| {
            let line1 = a.next().unwrap();
            let line2 = a.next().unwrap();
            let line3 = a.next().unwrap();

            for c in line1.chars() {
                if line2.contains(c) && line3.contains(c) {
                    return priority(c);
                }
            }

            panic!("no match found");
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u32 = 3;

    const EXAMPLE: &str = indoc::indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "157");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "7701");
    }

    #[test]
    fn pasrt2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "70");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "2644");
    }
}
