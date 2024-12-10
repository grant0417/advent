use std::collections::VecDeque;

use crate::util::grid::Grid;

fn distance_bfs(map: &Grid<char>) -> Grid<Option<usize>> {
    let mut distances: Grid<Option<usize>> = Grid::new(map.width(), map.height());
    let mut queue = VecDeque::new();

    // Find the starting point 'S' and mark it as distance 0
    let start = map.find(|&c| c == 'S').unwrap();
    distances[start] = Some(0);

    // Add initial directions for 'S'
    if start.y > 0 && matches!(map.get(start - (0, 1).into()), Some('|' | 'L' | 'J')) {
        queue.push_back((start.x, start.y - 1, 1));
    }
    if start.y < map.height() - 1 && matches!(map.get(start + (0, 1).into()), Some('|' | '7' | 'F'))
    {
        queue.push_back((start.x, start.y + 1, 1));
    }
    if start.x > 0 && matches!(map.get(start - (1, 0).into()), Some('-' | 'L' | 'F')) {
        queue.push_back((start.x - 1, start.y, 1));
    }
    if start.x < map.width() - 1 && matches!(map.get(start + (1, 0).into()), Some('-' | '7' | 'J'))
    {
        queue.push_back((start.x + 1, start.y, 1));
    }

    while let Some((x, y, distance)) = queue.pop_front() {
        distances[(x, y)] = Some(distance);

        // Get the directions we can go from this point
        let (north, south, east, west) = match map[(x, y)] {
            '|' => (true, true, false, false),
            '-' => (false, false, true, true),
            'L' => (true, false, true, false),
            'J' => (true, false, false, true),
            '7' => (false, true, false, true),
            'F' => (false, true, true, false),
            t => unreachable!("invalid map: {t}"),
        };

        // Add edges to the queue
        if north && y > 0 && distances.get((x, y - 1).into()) == Some(&None) {
            queue.push_back((x, y - 1, distance + 1));
        }

        if south && y < map.height() - 1 && distances.get((x, y + 1).into()) == Some(&None) {
            queue.push_back((x, y + 1, distance + 1));
        }

        if east && x < map.width() - 1 && distances.get((x + 1, y).into()) == Some(&None) {
            queue.push_back((x + 1, y, distance + 1));
        }

        if west && x > 0 && distances.get((x - 1, y).into()) == Some(&None) {
            queue.push_back((x - 1, y, distance + 1));
        }
    }

    distances
}

pub fn part1(input: &str) -> impl ToString {
    let map = Grid::parse(input);
    let distances = distance_bfs(&map);
    distances.iter().filter_map(|d| *d.0).max().unwrap()
}

pub fn part2(input: &str) -> impl ToString {
    let map = Grid::parse(input);
    let distances = distance_bfs(&map);

    // loop from left to right of each line applying the jordan curve theorem
    // https://en.wikipedia.org/wiki/Jordan_curve_theorem
    let mut count = 0;
    for y in 0..map.height() {
        let mut i = 0;
        for x in 0..map.width() {
            if distances[(x, y)].is_none() {
                if i % 2 == 1 {
                    count += 1;
                }
            } else if matches!(map[(x, y)], '|' | 'L' | 'J') {
                i += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 10;

    const EXAMPLE_1: &str = indoc::indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1).to_string(), "8");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "6907");
    }

    const EXAMPLE_2: &str = indoc::indoc! {"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
    "};

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_2).to_string(), "4");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "541");
    }
}
