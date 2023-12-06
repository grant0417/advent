use std::ops::Range;

fn parse_range(input: &str) -> Range<u32> {
    let (start, end) = input.trim_end().split_once('-').unwrap();
    let start = start.parse::<u32>().unwrap();
    let end = end.parse::<u32>().unwrap();
    start..end
}

/// - It is a six-digit number.
/// - The value is within the range given in your puzzle input.
/// - Two adjacent digits are the same (like 22 in 122345).
/// - Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
fn is_valid(pass: &u32, strict_adjecent: bool) -> bool {
    let pass = {
        let mut pass = *pass;
        let mut pass_arr = [0; 6];
        for i in 0..6 {
            pass_arr[5 - i] = pass % 10;
            pass /= 10;
        }
        pass_arr
    };

    let mut has_adj = false;
    for i in 0..5 {
        let val = pass[i];
        let next = pass[i + 1];
        if val > next {
            return false;
        }
        if !strict_adjecent {
            has_adj = has_adj || val == next;
        } else {
            has_adj = has_adj
                || val == next
                    && ((i == 0 && pass[i + 2] != val)
                        || (i == 4 && pass[i - 1] != val)
                        || (i != 0 && i != 4 && pass[i + 2] != val && pass[i - 1] != val))
        }
    }

    has_adj
}

pub fn part1(input: &str) -> impl ToString {
    parse_range(input)
        .filter(|pass| is_valid(pass, false))
        .count()
}

pub fn part2(input: &str) -> impl ToString {
    parse_range(input)
        .filter(|pass| is_valid(pass, true))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2019;
    const DAY: u32 = 4;

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "931");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "609");
    }
}
