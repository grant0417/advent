use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Unexpected character: {:?}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<Spring>,
    counts: Vec<u64>,
}

impl Record {
    fn parse(s: &str) -> Self {
        let (springs, counts) = s.split_once(' ').unwrap();
        let springs = springs.chars().map(Spring::parse).collect();
        let counts = counts.split(',').map(|s| s.parse().unwrap()).collect();
        Self { springs, counts }
    }

    fn permutation(&self, mut i: usize) -> Self {
        let mut record = self.clone();
        for val in &mut record.springs {
            if *val == Spring::Unknown {
                *val = if i % 2 == 0 {
                    Spring::Damaged
                } else {
                    Spring::Operational
                };
                i /= 2;
            }
        }
        record
    }

    fn is_valid(&self) -> bool {
        let mut counts = vec![];
        let mut prev = None;
        for spring in &self.springs {
            match spring {
                Spring::Operational => {}
                Spring::Damaged => match prev {
                    Some(Spring::Damaged) => {
                        *counts.last_mut().unwrap() += 1;
                    }
                    _ => counts.push(1),
                },
                Spring::Unknown => return false,
            }
            prev = Some(*spring);
        }
        counts == self.counts
    }

    fn brute_force_arangements(&self) -> usize {
        let unknown = self
            .springs
            .iter()
            .filter(|&&s| s == Spring::Unknown)
            .count() as u32;

        (0..(2_usize.pow(unknown)))
            .into_par_iter()
            .by_exponential_blocks()
            .filter(|&i| {
                let perm = self.permutation(i);
                perm.is_valid()
            })
            .count()
    }
}

fn parse_input(input: &str) -> Vec<Record> {
    input.lines().map(Record::parse).collect()
}

pub fn part1(input: &str) -> impl Display {
    let records = parse_input(input);
    records
        .par_iter()
        .map(|r| r.brute_force_arangements())
        .sum::<usize>()
}

pub fn part2(_input: &str) -> impl Display {
    0
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 12;

    const EXAMPLE: &str = indoc::indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    // const EXAMPLE: &str = indoc::indoc! {"
    //     #.#.### 1,1,3
    //     .#...#....###. 1,1,3
    //     .#.###.#.###### 1,3,1,6
    //     ####.#...#... 4,1,1
    //     #....######..#####. 1,6,5
    //     .###.##....# 3,2,1
    // "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "21");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "7718");
    }

    #[ignore = "not finished"]
    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "525152");
    }

    #[ignore = "not finished"]
    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "0");
    }
}
