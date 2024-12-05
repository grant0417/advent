use std::cmp::Ordering;

use crate::prelude::*;

struct Manual {
    rules: Vec<(u32, u32)>,
    page_numbers_lines: Vec<Vec<u32>>,
}

fn parse_input(input: &str) -> Manual {
    let (rules, page_numbers_lines) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|l| {
            let (l, r) = l.split_once('|').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    let page_numbers_lines = page_numbers_lines
        .lines()
        .map(|l| l.split(",").map(|i| i.parse().unwrap()).collect())
        .collect();

    Manual {
        rules,
        page_numbers_lines,
    }
}

pub fn part1(input: &str) -> impl Display {
    let Manual {
        rules,
        page_numbers_lines,
    } = parse_input(input);
    let mut count = 0;

    'outer: for pages in &page_numbers_lines {
        for i in 0..pages.len() {
            let pages_slice = &pages[i..];

            for &(left, right) in &rules {
                if pages_slice[0] == right && pages_slice[1..].iter().contains(&left) {
                    continue 'outer;
                }
            }
        }

        count += pages[pages.len() / 2];
    }

    count
}

pub fn part2(input: &str) -> impl Display {
    let Manual {
        rules,
        page_numbers_lines: page_numbers,
    } = parse_input(input);
    let mut incorrect = Vec::new();

    'outer: for pages in page_numbers {
        for i in 0..pages.len() {
            let pages_slice = &pages[i..];

            for &(left, right) in &rules {
                if pages_slice[0] == right && pages_slice[1..].iter().contains(&left) {
                    incorrect.push(pages.clone());
                    continue 'outer;
                }
            }
        }
    }

    let mut count = 0;
    for mut pages in incorrect {
        pages.sort_unstable_by(|a, b| {
            for (rule_a, rule_b) in &rules {
                match (a, b) {
                    _ if a == rule_a && b == rule_b => return Ordering::Less,
                    _ if a == rule_b && b == rule_a => return Ordering::Greater,
                    _ => continue,
                }
            }
            Ordering::Equal
        });

        count += pages[pages.len() / 2];
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 5;

    const EXAMPLE: &str = indoc::indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "143");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "5964");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "123");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "4719");
    }
}