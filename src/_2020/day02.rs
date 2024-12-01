use std::fmt::Display;

#[derive(Debug)]
struct Entry {
    min: usize,
    max: usize,
    c: char,
    password: String,
}

impl Entry {
    fn parse(s: &str) -> Self {
        let (min, s) = s.split_once('-').unwrap();
        let (max, s) = s.split_once(' ').unwrap();
        let (c, s) = s.split_once(':').unwrap();
        let password = s.trim().into();

        Self {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
            c: c.parse().unwrap(),
            password,
        }
    }

    fn is_valid_1(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.c).count();
        self.min <= count && count <= self.max
    }

    fn is_valid_2(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        (chars[self.min - 1] == self.c) != (chars[self.max - 1] == self.c)
    }
}

fn parse_input(input: &str) -> Vec<Entry> {
    input.lines().map(Entry::parse).collect()
}

pub fn part1(input: &str) -> impl Display {
    let entries = parse_input(input);
    entries.iter().filter(|e| e.is_valid_1()).count()
}

pub fn part2(input: &str) -> impl Display {
    let entries = parse_input(input);
    entries.iter().filter(|e| e.is_valid_2()).count()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2020;
    const DAY: u32 = 2;

    const EXAMPLE: &str = indoc::indoc! {"
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "2");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "548");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "1");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "502");
    }
}
