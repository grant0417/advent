use crate::prelude::*;

fn parse_input(input: &str) -> impl Iterator<Item = &str> + '_ {
    input.lines()
}

fn has_three_vowels(s: &&str) -> bool {
    s.bytes().filter(|c| b"aeiou".contains(c)).count() >= 3
}

fn has_one_letter_twice(s: &&str) -> bool {
    s.bytes().tuple_windows().any(|(a, b)| a == b)
}

fn does_not_contain_invalid_str(s: &&str) -> bool {
    !s.bytes()
        .tuple_windows()
        .any(|(a, b)| [(b'a', b'b'), (b'c', b'd'), (b'p', b'q'), (b'x', b'y')].contains(&(a, b)))
}

fn contains_pair_twice(s: &&str) -> bool {
    for i in 0..s.len() - 1 {
        let pair = &s[i..=i + 1];
        if s.matches(pair).count() >= 2 {
            return true;
        }
    }
    false
}

fn one_letter_gap_pair(s: &&str) -> bool {
    s.bytes().tuple_windows().any(|(a, _, b)| a == b)
}

pub fn part1(input: &str) -> impl Display {
    parse_input(input)
        .filter(has_three_vowels)
        .filter(has_one_letter_twice)
        .filter(does_not_contain_invalid_str)
        .count()
}

pub fn part2(input: &str) -> impl Display {
    parse_input(input)
        .filter(contains_pair_twice)
        .filter(one_letter_gap_pair)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 5;

    const EXAMPLE1: &str = indoc! {"
        ugknbfddgicrmopn
        aaa
        jchzalrnumimnmhp
        haegwjzuvuyypxyu
        dvszwmarrgswjxmb
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE1).to_string(), "2");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "258");
    }

    const EXAMPLE2: &str = indoc! {"
        qjhvhtzxzqqjkmpb
        xxyxx
        uurcxstgmygtbstg
        ieodomkazucvgmuy
    "};

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE2).to_string(), "2");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "53");
    }
}
