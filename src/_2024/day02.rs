use crate::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> impl Display {
    let grid = parse_input(input);

    let mut safe = 0;
    'line_loop: for line in grid {
        let mut is_increasing = None;
        let mut prev = None;

        for curr in line {
            if let Some(prev) = prev {
                let increase = curr - prev > 0;
                if let Some(is_increasing) = is_increasing {
                    if is_increasing != increase {
                        continue 'line_loop;
                    }
                }
                is_increasing = Some(increase);

                let abs_diff = curr.abs_diff(prev);
                if abs_diff < 1 || abs_diff > 3 {
                    continue 'line_loop;
                }
            }
            prev = Some(curr);
        }

        safe += 1;
    }

    safe
}

pub fn part2(input: &str) -> impl Display {
    let grid = parse_input(input);

    let mut safe = 0;
    for line in grid {
        'try_loop: for i in 0..line.len() {
            let mut safe_line = line.clone();
            safe_line.remove(i);

            let mut is_increasing = None;
            let mut prev = None;

            for curr in safe_line {
                if let Some(prev) = prev {
                    let increase = curr - prev > 0;
                    if let Some(is_increasing) = is_increasing {
                        if is_increasing != increase {
                            continue 'try_loop;
                        }
                    }
                    is_increasing = Some(increase);

                    let abs_diff = curr.abs_diff(prev);
                    if abs_diff < 1 || abs_diff > 3 {
                        continue 'try_loop;
                    }
                }
                prev = Some(curr);
            }

            safe += 1;
            break 'try_loop;
        }
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "4");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "674");
    }
}
