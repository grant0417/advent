use std::{collections::VecDeque, ops};

fn char_to_index(c: char) -> usize {
    c as usize - 'a' as usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SevenSegment(u8);

impl SevenSegment {
    fn new(input: &str) -> SevenSegment {
        let mut value: u8 = 0;

        for c in input.chars() {
            value |= 1 << char_to_index(c);
        }

        SevenSegment(value)
    }

    fn count_lit(&self) -> u32 {
        self.0.count_ones()
    }
}

impl ops::BitAnd for SevenSegment {
    type Output = SevenSegment;

    fn bitand(self, rhs: Self) -> Self::Output {
        SevenSegment(self.0 & rhs.0)
    }
}

impl ops::Not for SevenSegment {
    type Output = SevenSegment;

    fn not(self) -> Self::Output {
        SevenSegment(!self.0)
    }
}

fn parse_input(
    input: impl AsRef<str>,
) -> VecDeque<(VecDeque<SevenSegment>, VecDeque<SevenSegment>)> {
    let mut lines = VecDeque::new();
    for line in input.as_ref().lines() {
        let mut split = line
            .split('|')
            .map(|s| s.trim().split(' ').map(SevenSegment::new).collect());

        let left = split.next().unwrap();
        let right = split.next().unwrap();

        lines.push_back((left, right));
    }

    lines
}

pub fn part1(input: &str) -> impl ToString {
    let parsed_input = parse_input(input);

    parsed_input
        .iter()
        .map(|(_, out)| {
            out.iter()
                .filter(|a| [2, 3, 4, 7].contains(&a.count_lit()))
                .count()
        })
        .sum::<usize>()
}

pub fn part2(input: &str) -> impl ToString {
    let parsed_input = parse_input(input);

    let mut sum = 0;

    for (mut in_segments, out_segments) in parsed_input {
        let mut seven_segments: [Option<SevenSegment>; 10] = [None; 10];

        while let Some(segment) = in_segments.pop_front() {
            let segment_count = segment.count_lit();

            let one = seven_segments[1].map(|s| (s & segment).count_lit());
            let four = seven_segments[4].map(|s| (s & segment).count_lit());

            match (segment_count, one, four) {
                (2, _, _) => seven_segments[1] = Some(segment),
                (3, _, _) => seven_segments[7] = Some(segment),
                (4, _, _) => seven_segments[4] = Some(segment),
                (7, _, _) => seven_segments[8] = Some(segment),
                (6, Some(2), Some(4)) => seven_segments[9] = Some(segment),
                (6, Some(2), Some(3)) => seven_segments[0] = Some(segment),
                (6, Some(1), Some(3)) => seven_segments[6] = Some(segment),
                (5, Some(2), Some(3)) => seven_segments[3] = Some(segment),
                (5, Some(1), Some(3)) => seven_segments[5] = Some(segment),
                (5, Some(1), Some(2)) => seven_segments[2] = Some(segment),
                (_, _, _) => in_segments.push_back(segment),
            }
        }

        let mut number = 0;
        for segment in &out_segments {
            for (i, known_segment) in seven_segments.iter().enumerate() {
                if let Some(inner_segment) = known_segment {
                    if segment == inner_segment {
                        number = number * 10 + i;
                    }
                }
            }
        }

        sum += number;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u32 = 8;

    const EXAMPLE: &str = indoc::indoc! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "26");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "440");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "61229");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1046281");
    }
}
