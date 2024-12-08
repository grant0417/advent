use crate::prelude::*;

fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse_bytes(input)
}

pub fn part1(input: &str) -> impl Display {
    let grid = parse_input(input);

    let mut points = HashMap::default();
    for (&c, point) in grid.iter() {
        if c != b'.' {
            points.entry(c).or_insert_with(Vec::new).push(point);
        }
    }

    let mut annodes = Grid::new(grid.width(), grid.height());
    for d in points.values() {
        for i in 0..d.len() {
            for j in (i + 1)..d.len() {
                let p0 = d[i].map(|a| a as i64);
                let p1 = d[j].map(|a| a as i64);

                let max_y = p0.y.max(p1.y);
                let min_y = p0.y.min(p1.y);

                let max_x = p0.x.max(p1.x);
                let min_x = p0.x.min(p1.x);

                let dy = max_y - min_y;
                let dx = max_x - min_x;

                let mut a0 = p0;
                let mut a1 = p1;

                if a0.x > a1.x {
                    a0.x += dx;
                    a1.x -= dx;
                } else {
                    a0.x -= dx;
                    a1.x += dx;
                }

                if a0.y > a1.y {
                    a0.y += dy;
                    a1.y -= dy;
                } else {
                    a0.y -= dy;
                    a1.y += dy;
                }

                if let Ok(a0) = a0.try_into() {
                    let a0: Point<usize> = a0;
                    if let Some(p) = annodes.get_mut(a0) {
                        *p = 1;
                    }
                }
                if let Ok(a1) = a1.try_into() {
                    let a1: Point<usize> = a1;
                    if let Some(p) = annodes.get_mut(a1) {
                        *p = 1;
                    }
                }
            }
        }
    }

    annodes.iter().map(|(&c, _)| c as usize).sum::<usize>()
}

pub fn part2(input: &str) -> impl Display {
    let grid = parse_input(input);

    let mut points = HashMap::default();
    for (&c, point) in grid.iter() {
        if c != b'.' {
            points.entry(c).or_insert_with(Vec::new).push(point);
        }
    }

    let mut annodes = Grid::new(grid.width(), grid.height());
    for d in points.values() {
        for i in 0..d.len() {
            for j in (i + 1)..d.len() {
                let p0 = d[i].map(|a| a as i64);
                let p1 = d[j].map(|a| a as i64);

                let max_y = p0.y.max(p1.y);
                let min_y = p0.y.min(p1.y);

                let max_x = p0.x.max(p1.x);
                let min_x = p0.x.min(p1.x);

                let dy = max_y - min_y;
                let dx = max_x - min_x;

                let mut a0 = p0;
                let mut a1 = p1;

                if let Ok(a0) = a0.try_into() {
                    let a0: Point<usize> = a0;
                    if let Some(p) = annodes.get_mut(a0) {
                        *p = 1;
                    }
                }
                if let Ok(a1) = a1.try_into() {
                    let a1: Point<usize> = a1;
                    if let Some(p) = annodes.get_mut(a1) {
                        *p = 1;
                    }
                }

                for _ in 0..100 {
                    if a0.x > a1.x {
                        a0.x += dx;
                        a1.x -= dx;
                    } else {
                        a0.x -= dx;
                        a1.x += dx;
                    }

                    if a0.y > a1.y {
                        a0.y += dy;
                        a1.y -= dy;
                    } else {
                        a0.y -= dy;
                        a1.y += dy;
                    }

                    if let Ok(a0) = a0.try_into() {
                        let a0: Point<usize> = a0;
                        if let Some(p) = annodes.get_mut(a0) {
                            *p = 1;
                        }
                    }
                    if let Ok(a1) = a1.try_into() {
                        let a1: Point<usize> = a1;
                        if let Some(p) = annodes.get_mut(a1) {
                            *p = 1;
                        }
                    }
                }
            }
        }
    }

    annodes.iter().map(|(&c, _)| c as usize).sum::<usize>()
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
