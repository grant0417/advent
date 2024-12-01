use std::fmt::Display;

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1(input: &str) -> impl Display {
    let entries = parse_input(input);

    for i in &entries {
        for j in &entries {
            if *i + *j == 2020 {
                return *i * *j;
            }
        }
    }

    panic!("no answer found")
}

pub fn part2(input: &str) -> impl Display {
    let entries = parse_input(input);

    for i in &entries {
        for j in &entries {
            for k in &entries {
                if *i + *j + *k == 2020 {
                    return *i * *j * *k;
                }
            }
        }
    }

    panic!("no answer found")
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2020;
    const DAY: u32 = 1;

    const EXAMPLE: &str = indoc::indoc! {"
        1721
        979
        366
        299
        675
        1456
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "514579");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "542619");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "241861950");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "32858450");
    }
}
