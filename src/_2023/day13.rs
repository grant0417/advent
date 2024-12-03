use crate::prelude::*;

fn parse_input(input: &str) -> Vec<Grid<char>> {
    input.split("\n\n").map(|s| Grid::parse(s)).collect()
}

pub fn part1(input: &str) -> impl Display {
    let s = parse_input(input);
    for grid in s {
        grid.get(Point::new(grid.height(), grid.width()));
    }
    0
}

pub fn part2(input: &str) -> impl Display {
    let _ = parse_input(input);
    0
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 13;

    const EXAMPLE: &str = indoc::indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "0");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "0");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "0");
    }
}
