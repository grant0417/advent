use rand::distributions::uniform::SampleRange;
use rustc_hash::FxBuildHasher;

use crate::prelude::*;

const CAPACITY: usize = 1024;

pub fn part1(input: &str) -> impl Display {
    let mut lista = Vec::with_capacity(CAPACITY);
    let mut listb = Vec::with_capacity(CAPACITY);

    let mut current_number = 0u64;
    let mut in_number = false;

    for &byte in input.as_bytes() {
        match byte {
            b'0'..=b'9' => {
                current_number = current_number * 10 + (byte - b'0') as u64;
                in_number = true;
            }
            _ => {
                if in_number {
                    if lista.len() == listb.len() {
                        lista.push(current_number);
                    } else {
                        listb.push(current_number);
                    }
                    current_number = 0;
                    in_number = false;
                }
            }
        }
    }

    lista.sort_unstable();
    listb.sort_unstable();

    let mut dist = 0;
    for i in 0..lista.len() {
        dist += lista[i].abs_diff(listb[i]);
    }
    dist
}

pub fn part2(input: &str) -> impl Display {
    let mut list_a = Vec::with_capacity(CAPACITY);
    let mut counts_b: std::collections::HashMap<u64, u64, FxBuildHasher> =
        HashMap::with_capacity_and_hasher(CAPACITY, FxBuildHasher);

    let mut current_number = 0u64;
    let mut in_number = false;
    let mut is_first_number = true;

    for &byte in input.as_bytes() {
        match byte {
            b'0'..=b'9' => {
                current_number = current_number * 10 + (byte - b'0') as u64;
                in_number = true;
            }
            _ => {
                if in_number {
                    if is_first_number {
                        list_a.push(current_number);
                        is_first_number = false;
                    } else {
                        *counts_b.entry(current_number).or_insert(0) += 1;
                        is_first_number = true;
                    }
                    current_number = 0;
                    in_number = false;
                }
            }
        }
    }

    if in_number && !is_first_number {
        *counts_b.entry(current_number).or_insert(0) += 1;
    }

    let mut v = 0;
    for a in list_a {
        let count = counts_b.get(&a).cloned().unwrap_or(0);
        v += count * a
    }
    v
}

pub fn gen_input<R: rand::Rng>(rng: &mut R) -> String {
    let mut s = String::new();

    let mut push_number = |s: &mut String| {
        s.push(('1'..='9').sample_single(rng));
        for _i in 0..4 {
            s.push(('0'..='9').sample_single(rng))
        }
    };

    for _ in 0..1000 {
        push_number(&mut s);
        s.push_str("   ");
        push_number(&mut s);
        s.push('\n');
    }

    s
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 1;

    const EXAMPLE: &str = indoc::indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3 
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "11");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "2113135");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "31");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "19097157");
    }
}
