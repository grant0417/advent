use crate::prelude::*;

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u32)
        .collect()
}

fn step(prev: &Vec<u32>, next: &mut Vec<u32>) {
    next.clear();
    let mut i = 0;
    while i < prev.len() {
        let mut j = i;
        while j < prev.len() - 1 && prev[j] == prev[j + 1] {
            j += 1
        }
        next.push((j - i + 1) as u32);
        next.push(prev[i]);
        i = j + 1;
    }
}

fn look_and_say(arr: Vec<u32>, n: usize) -> usize {
    let mut prev = arr;
    let mut next = Vec::new();
    for _ in 0..n {
        step(&prev, &mut next);
        [prev, next] = [next, prev];
    }
    prev.len()
}

pub fn part1(input: &str) -> impl Display {
    let arr = parse_input(input);
    look_and_say(arr, 40)
}

pub fn part2(input: &str) -> impl Display {
    let arr = parse_input(input);
    look_and_say(arr, 50)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 10;

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "252594");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "3579328");
    }
}
