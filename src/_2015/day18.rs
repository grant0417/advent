use crate::prelude::*;

#[rustfmt::skip]
const NEIGHBORS: [(i64, i64); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1),  (0, 1),  (1, 1),
];

const ON: u8 = b'#';
const OFF: u8 = b'.';

fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse_bytes(input)
}

fn step(curr: &Grid<u8>, next: &mut Grid<u8>) {
    for (v, point) in curr.iter() {
        let point_i64: Point<i64> = point.try_into().unwrap();
        let mut sum = 0;
        for (dx, dy) in NEIGHBORS {
            let neighbor_point = Point::new(point_i64.x + dx, point_i64.y + dy);
            if let Ok(point_usize) = neighbor_point.try_into() {
                if matches!(curr.get(point_usize), Some(&ON)) {
                    sum += 1;
                }
            }
        }
        if let Some(p) = next.get_mut(point) {
            let next_value = match (*v == ON, sum) {
                (true, 2 | 3) => ON,
                (false, 3) => ON,
                (_, _) => OFF,
            };
            *p = next_value;
        }
    }
}

fn run(grid: &Grid<u8>, steps: usize, stuck_corners: bool) -> Grid<u8> {
    let mut curr = grid.clone();
    let mut next = Grid::new(grid.width(), grid.height());
    for _ in 0..steps {
        next.clear_with(b'.');
        step(&curr, &mut next);

        if stuck_corners {
            next[0][0] = ON;
            next[0][grid.width() - 1] = ON;
            next[grid.height() - 1][0] = ON;
            next[grid.height() - 1][grid.width() - 1] = ON;
        }

        (curr, next) = (next, curr);
    }
    curr
}

pub fn part1(input: &str) -> impl Display {
    let grid = parse_input(input);
    let end = run(&grid, 100, false);
    end.iter().filter(|(&v, _)| v == ON).count()
}

pub fn part2(input: &str) -> impl Display {
    let grid = parse_input(input);
    let end = run(&grid, 100, true);
    end.iter().filter(|(&v, _)| v == ON).count()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 18;

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "1061");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1006");
    }
}
