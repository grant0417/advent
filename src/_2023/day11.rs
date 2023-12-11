use ahash::HashMap;

use crate::util::grid::Grid;

fn sum_distances(grid: &Grid<char>, space: usize) -> usize {
    // Find the empty columns
    let mut empty_y = vec![];
    for y in 0..grid.height() {
        let mut is_empty = true;
        for x in 0..grid.width() {
            if grid[(x, y)] == '#' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_y.push(y);
        }
    }

    // Find the empty rows
    let mut empty_x = vec![];
    for x in 0..grid.width() {
        let mut is_empty = true;
        for y in 0..grid.height() {
            if grid[(x, y)] == '#' {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            empty_x.push(x);
        }
    }

    // Assign each point a number and store it in a hashmap
    let points: HashMap<_, _> = grid
        .iter()
        .filter(|(&c, _)| c == '#')
        .enumerate()
        .map(|(i, (_, p))| (i + 1, p))
        .collect();

    let mut sum = 0;
    for i in 1..=points.len() {
        for j in i + 1..=points.len() {
            // calculate the distance between the two points and add the space size for each empty row/column
            let p1 = points[&i];
            let p2 = points[&j];

            sum += p1.manhattan_distance(&p2);

            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);

            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            for x in &empty_x {
                if (min_x..max_x).contains(x) {
                    sum += space - 1;
                }
            }

            for y in &empty_y {
                if (min_y..max_y).contains(y) {
                    sum += space - 1;
                }
            }
        }
    }

    sum
}

pub fn part1(input: &str) -> impl ToString {
    let grid = Grid::parse(input);
    sum_distances(&grid, 2)
}

pub fn part2(input: &str) -> impl ToString {
    let grid = Grid::parse(input);
    sum_distances(&grid, 1000000)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 11;

    const EXAMPLE: &str = indoc::indoc! {"
       ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "374");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "9734203");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "82000210");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "568914596391");
    }
}
