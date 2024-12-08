use crate::prelude::*;

fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse_bytes(input)
}

fn get_antennas(grid: &Grid<u8>) -> HashMap<u8, Vec<Point<i64>>> {
    let mut antennas = HashMap::default();
    for (&c, point) in grid.iter() {
        if c != b'.' {
            antennas
                .entry(c)
                .or_insert_with(Vec::new)
                .push(point.try_into().unwrap());
        }
    }
    antennas
}

fn set_annode(annodes: &mut Grid<u8>, point: Point<i64>) {
    if let Ok(point) = point.try_into() {
        let point: Point<usize> = point;
        if let Some(p) = annodes.get_mut(point) {
            *p = 1;
        }
    }
}

fn get_annodes(
    antennas: &HashMap<u8, Vec<Point<i64>>>,
    width: usize,
    height: usize,
    part2: bool,
) -> Grid<u8> {
    let mut annodes = Grid::new(width, height);
    for points in antennas.values() {
        for (i, &p0) in points.iter().enumerate() {
            for &p1 in points.iter().skip(i + 1) {
                let (min_y, max_y) = (p0.y.min(p1.y), p0.y.max(p1.y));
                let (min_x, max_x) = (p0.x.min(p1.x), p0.x.max(p1.x));

                let delta = Point {
                    x: max_x - min_x,
                    y: max_y - min_y,
                };

                let mut a0 = p0;
                let mut a1 = p1;

                if part2 {
                    set_annode(&mut annodes, a0);
                    set_annode(&mut annodes, a1);
                }

                let iter = if part2 { width.max(height) } else { 1 };
                for _ in 0..iter {
                    if a0.x > a1.x {
                        a0.x += delta.x;
                        a1.x -= delta.x;
                    } else {
                        a0.x -= delta.x;
                        a1.x += delta.x;
                    }

                    if a0.y > a1.y {
                        a0.y += delta.y;
                        a1.y -= delta.y;
                    } else {
                        a0.y -= delta.y;
                        a1.y += delta.y;
                    }

                    set_annode(&mut annodes, a0);
                    set_annode(&mut annodes, a1);
                }
            }
        }
    }
    annodes
}

fn solve(input: &str, part2: bool) -> usize {
    let grid = parse_input(input);
    let antennas = get_antennas(&grid);
    let annodes = get_annodes(&antennas, grid.width(), grid.height(), part2);
    annodes.iter().map(|(&c, _)| c as usize).sum::<usize>()
}

pub fn part1(input: &str) -> impl Display {
    solve(input, false)
}

pub fn part2(input: &str) -> impl Display {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 8;

    const EXAMPLE: &str = indoc::indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "14");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "336");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "34");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1131");
    }
}
