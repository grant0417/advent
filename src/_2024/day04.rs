use crate::prelude::*;

fn parse_input(input: &str) -> Grid<char> {
    Grid::parse(input)
}

fn check_word(grid: &Grid<char>, word: &str, start: Point<i64>, offset: Point<i64>) -> bool {
    for (i, c) in word.chars().enumerate() {
        let Ok(p): Result<Point<usize>, _> = (start + offset.scale(i as i64)).try_into() else {
            return false;
        };
        if let Some(&gc) = grid.get(p) {
            if c != gc {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) -> impl Display {
    let grid = parse_input(input);
    let mut count = 0;
    for i in 0..grid.width() {
        for j in 0..grid.height() {
            for word in ["XMAS", "SAMX"] {
                for offset in [(1, 0), (0, 1), (1, 1), (-1, 1)] {
                    if check_word(&grid, word, (i as i64, j as i64).into(), offset.into()) {
                        count += 1;
                    };
                }
            }
        }
    }
    count
}

pub fn part2(input: &str) -> impl Display {
    let grid = parse_input(input);
    let patterns = [
        [['M', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'S']],
        [['S', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'M']],
        [['M', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'S']],
        [['S', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'M']],
    ];

    let mut count = 0;
    for i in 0..grid.width() - 2 {
        for j in 0..grid.height() - 2 {
            'pattern_loop: for pattern in &patterns {
                for (k, pattern_row) in pattern.iter().enumerate() {
                    for (l, &p) in pattern_row.iter().enumerate() {
                        if p != '.' {
                            if let Some(&c) = grid.get((i + k, j + l).into()) {
                                if c == p {
                                    // ok
                                } else {
                                    continue 'pattern_loop;
                                }
                            } else {
                                continue 'pattern_loop;
                            }
                        }
                    }
                }
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 4;

    const EXAMPLE: &str = indoc::indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "18");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "2532");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "9");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1941");
    }
}
