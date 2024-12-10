use crate::prelude::*;

fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse_bytes(input)
}

enum Mode {
    Any,
    Unique,
}

const DIRECTIONS: [Point<i64>; 4] = [
    Point::new(-1, 0),
    Point::new(1, 0),
    Point::new(0, -1),
    Point::new(0, 1),
];

fn search(grid: Grid<u8>, mode: Mode) -> usize {
    let zeros: Vec<_> = grid
        .iter()
        .filter(|(&i, _)| i == b'0')
        .map(|(_, p)| p)
        .collect();
    let mut trail_scores = vec![];

    for zero in zeros {
        let zero: Point<i64> = zero.try_into().unwrap();
        let mut queue = DIRECTIONS.map(|p| (zero + p, 0)).to_vec();

        let mut dest_any = HashSet::default();
        let mut dest_unique = Vec::new();

        while let Some((point, prev)) = queue.pop() {
            if let Ok(point_usize) = point.try_into() {
                let point_usize: Point<usize> = point_usize;
                if let Some(&next) = grid.get(point_usize) {
                    let next = next.saturating_sub(b'0');
                    if next == prev + 1 {
                        if next == 9 {
                            dest_any.insert(point_usize);
                            dest_unique.push(point_usize);
                        } else {
                            queue.extend(DIRECTIONS.map(|p| (point + p, next)));
                        }
                    }
                }
            }
        }

        let score = match mode {
            Mode::Any => dest_any.len(),
            Mode::Unique => dest_unique.len(),
        };
        trail_scores.push(score);
    }

    trail_scores.iter().sum::<usize>()
}

pub fn part1(input: &str) -> impl Display {
    let grid = parse_input(input);
    search(grid, Mode::Any)
}

pub fn part2(input: &str) -> impl Display {
    let grid = parse_input(input);
    search(grid, Mode::Unique)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 10;

    const EXAMPLE_1: &str = indoc! {"
        ...0...
        ...1...
        ...2...
        6543456
        7.....7
        8.....8
        9.....9
    "};

    const EXAMPLE_2: &str = indoc! {"
        ..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987....
    "};

    const EXAMPLE_3: &str = indoc! {"
        10..9..
        2...8..
        3...7..
        4567654
        ...8..3
        ...9..2
        .....01
    "};

    const EXAMPLE_4: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    const EXAMPLE_5: &str = indoc! {"
        .....0.
        ..4321.
        ..5..2.
        ..6543.
        ..7..4.
        ..8765.
        ..9....
    "};

    const EXAMPLE_6: &str = indoc! {"
        012345
        123456
        234567
        345678
        4.6789
        56789.
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1).to_string(), "2");
        assert_eq!(part1(EXAMPLE_2).to_string(), "4");
        assert_eq!(part1(EXAMPLE_3).to_string(), "3");
        assert_eq!(part1(EXAMPLE_4).to_string(), "36");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "841");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_5).to_string(), "3");
        assert_eq!(part2(EXAMPLE_2).to_string(), "13");
        assert_eq!(part2(EXAMPLE_6).to_string(), "227");
        assert_eq!(part2(EXAMPLE_4).to_string(), "81");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1875");
    }
}
