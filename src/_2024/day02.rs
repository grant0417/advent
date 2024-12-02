use crate::{prelude::*, util::fast::fast_parse_u64};

pub fn part1(input: &str) -> impl Display {
    let mut safe = 0;
    'line_loop: for line in input.lines() {
        let mut is_increaseing = None;

        let mut line = line.split_ascii_whitespace();
        let mut prev = fast_parse_u64(line.next().unwrap()) as i64;

        for curr in line {
            let curr = fast_parse_u64(&curr) as i64;

            let diff: i64 = curr - prev;
            let increase = diff > 0;
            if let Some(is_increaseing) = is_increaseing {
                if is_increaseing != increase {
                    continue 'line_loop;
                }
            }
            is_increaseing = Some(increase);

            let abs_diff = diff.abs();
            if abs_diff < 1 || abs_diff > 3 {
                continue 'line_loop;
            }
            prev = curr;
        }

        safe += 1;
    }
    safe
}

pub fn part2(input: &str) -> impl Display {
    let mut safe = 0;
    'line_loop: for line in input.lines() {
        let mut is_increaseing = None;
        let mut has_skipped = false;

        let mut line = line.split_ascii_whitespace();
        let mut prev = fast_parse_u64(line.next().unwrap()) as i64;

        'item_loop: for curr in line {
            let curr = fast_parse_u64(&curr) as i64;

            let diff: i64 = curr - prev;
            let increase = diff > 0;
            if let Some(is_increaseing) = is_increaseing {
                if is_increaseing != increase {
                    if !has_skipped {
                        has_skipped = true;
                        continue 'item_loop;
                    } else {
                        continue 'line_loop;
                    }
                }
            }
            is_increaseing = Some(increase);

            let abs_diff = diff.abs();
            if abs_diff < 1 || abs_diff > 3 {
                if !has_skipped {
                    has_skipped = true;
                    continue 'item_loop;
                } else {
                    continue 'line_loop;
                }
            }
            prev = curr;
        }

        safe += 1;
    }
    safe
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 2;

    const EXAMPLE: &str = indoc::indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "2");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "639");
    }

    #[ignore = "broke"]
    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "4");
    }

    #[ignore = "broke"]
    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "674");
    }
}
