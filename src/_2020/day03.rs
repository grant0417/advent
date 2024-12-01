use std::fmt::Display;

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|s| s == '#').collect())
        .collect()
}

pub fn part1(input: &str) -> impl Display {
    let rows = parse_input(input);
    rows.iter()
        .enumerate()
        .filter(|(i, row)| {
            let x = (i * 3) % row.len();
            row[x]
        })
        .count()
}

pub fn part2(input: &str) -> impl Display {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let rows = parse_input(input);
    let mut product: u64 = 1;
    for (dx, dy) in slopes {
        let mut cnt = 0;
        for (i, row) in rows.iter().step_by(dy).enumerate() {
            let x = (i * dx) % row.len();
            cnt += if row[x] { 1 } else { 0 }
        }
        product *= cnt;
    }
    product
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2020;
    const DAY: u32 = 3;

    const EXAMPLE: &str = indoc::indoc! {"
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "7");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "228");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "336");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "6818112000");
    }
}
