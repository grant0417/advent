use crate::prelude::*;

#[derive(Debug, Clone)]
struct Equation {
    left: u64,
    right: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let (left, right) = l.split_once(": ").unwrap();
            let left = left.parse().unwrap();
            let right = right.split(' ').map(|i| i.parse().unwrap()).collect();
            Equation { left, right }
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concatenate => {
                let b_digits = if b == 0 { 1 } else { b.ilog10() + 1 };
                a * 10_u64.pow(b_digits) + b
            }
        }
    }
}

fn solve<const N: usize>(equation: &Equation, operations: [Operation; N]) -> Option<u64> {
    let operations_count = equation.right.len() - 1;
    let combinations = (N as u64).pow(operations_count as u32);

    for mut combination in 0..combinations {
        let mut iter = equation.right.iter();
        let mut result = *iter.next()?;

        for term in iter {
            let operation = operations[combination as usize % N];
            result = operation.apply(result, *term);
            combination /= N as u64;
        }

        if result == equation.left {
            return Some(result);
        }
    }

    None
}

pub fn part1(input: &str) -> impl Display {
    let equations = parse_input(input);
    let operations = [Operation::Add, Operation::Multiply];
    equations
        .par_iter()
        .filter_map(|eq| solve(eq, operations))
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl Display {
    let equations = parse_input(input);
    let operations = [Operation::Add, Operation::Multiply, Operation::Concatenate];
    equations
        .par_iter()
        .filter_map(|eq| solve(eq, operations))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 7;

    const EXAMPLE: &str = indoc::indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "3749");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "1038838357795");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "11387");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "254136560217241");
    }
}
