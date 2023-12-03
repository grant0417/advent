#![allow(clippy::needless_range_loop)]

pub fn part1(input: &str) -> impl ToString {
    let elfs = input.split("\n\n");

    let mut most = 0;
    for elf in elfs {
        most = most.max(
            elf.lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>(),
        );
    }

    most
}

pub fn part2(input: &str) -> impl ToString {
    let elfs = input.split("\n\n");

    let mut top_3 = [0; 3];
    for elf in elfs {
        let mut sum = elf
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .sum::<u32>();

        for i in 0..3 {
            if sum > top_3[i] {
                (top_3[i], sum) = (sum, top_3[i]);
            }
        }
    }

    top_3.iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u32 = 1;

    const EXAMPLE: &str = indoc::indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "24000");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "69310");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "45000");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "206104");
    }
}
