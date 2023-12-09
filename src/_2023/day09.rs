fn parse_input(input: &str) -> impl Iterator<Item = Vec<i64>> + '_ {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect()
    })
}

fn diffs_between(row: &Vec<i64>) -> Vec<i64> {
    let mut diffs = vec![];
    for i in 0..row.len() - 1 {
        diffs.push(row[i + 1] - row[i]);
    }
    diffs
}

pub fn part1(input: &str) -> impl ToString {
    parse_input(input)
        .map(|row| {
            let mut diffs = vec![diffs_between(&row)];

            loop {
                let last = diffs.last().unwrap();
                if last.iter().all(|&diff| diff == 0) {
                    break;
                }
                diffs.push(diffs_between(last));
            }

            diffs.iter().map(|diff| diff.last().unwrap()).sum::<i64>() + row.last().unwrap()
        })
        .sum::<i64>()
}

pub fn part2(input: &str) -> impl ToString {
    parse_input(input)
        .map(|row| {
            let mut diffs = vec![diffs_between(&row)];

            loop {
                if diffs.last().unwrap().iter().all(|&diff| diff == 0) {
                    break;
                }

                diffs.push(diffs_between(diffs.last().unwrap()));
            }

            row.first().unwrap()
                - diffs
                    .iter()
                    .rev()
                    .fold(0, |acc, diff| diff.first().unwrap() - acc)
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 9;

    const EXAMPLE: &str = indoc::indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "114");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "2038472161");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "2");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1091");
    }
}
