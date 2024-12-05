use std::sync::LazyLock;

use regex::Regex;

use crate::prelude::*;

enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(mul\((\d*),(\d*)\)|do\(\)|don't\(\))").unwrap());

    RE.captures_iter(input)
        .map(|m| match m.get(0).unwrap().as_str() {
            "do()" => Instruction::Do,
            "don't()" => Instruction::Dont,
            _ => Instruction::Mul(
                m.get(2).unwrap().as_str().parse().unwrap(),
                m.get(3).unwrap().as_str().parse().unwrap(),
            ),
        })
}

pub fn part1(input: &str) -> impl Display {
    parse_input(input)
        .map(|i| match i {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl Display {
    parse_input(input)
        .fold((true, 0), |(enabled, i), instruction| match instruction {
            Instruction::Mul(a, b) if enabled => (enabled, i + a * b),
            Instruction::Mul(_, _) => (enabled, i),
            Instruction::Do => (true, i),
            Instruction::Dont => (false, i),
        })
        .1
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 3;

    const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1).to_string(), "161");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "188116424");
    }

    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_2).to_string(), "48");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "104245808");
    }
}
