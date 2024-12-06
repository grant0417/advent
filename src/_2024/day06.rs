use crate::prelude::*;

fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse_bytes(input)
}

fn solve(mut map: Grid<u8>) -> Option<Grid<bool>> {
    let mut visited = Grid::new(map.width(), map.height());

    // Find the ^, >, <, or v character
    let mut char = b'0';
    let mut pos: Point<i32> = Point::new(0, 0);

    's: for i in 0..map.height() {
        for j in 0..map.width() {
            if matches!(map[i][j], b'^' | b'>' | b'<' | b'v') {
                pos = Point::new(j as i32, i as i32);
                char = map[i][j];
                break 's;
            }
        }
    }

    let mut count = 0;
    loop {
        count += 1;
        if count > 10_000 {
            return None;
        }

        visited[pos.y as usize][pos.x as usize] = true;

        let (move_dir, next) = match char {
            b'^' => (Point::new(0, -1), b'>'),
            b'v' => (Point::new(0, 1), b'<'),
            b'>' => (Point::new(1, 0), b'v'),
            b'<' => (Point::new(-1, 0), b'^'),
            _ => unreachable!(),
        };

        let next_pos = Point::new(pos.x + move_dir.x, pos.y + move_dir.y);

        if next_pos.x < 0
            || next_pos.x as usize >= map.width()
            || next_pos.y < 0
            || next_pos.y as usize >= map.height()
        {
            break;
        }

        if map[next_pos.y as usize][next_pos.x as usize] != b'#' {
            map[pos.y as usize][pos.x as usize] = b'.';
            map[next_pos.y as usize][next_pos.x as usize] = char;
            pos = next_pos;
        } else {
            map[pos.y as usize][pos.x as usize] = next;
            char = next;
        }
    }

    println!("{count}");

    Some(visited)
}

pub fn part1(input: &str) -> impl Display {
    let map = parse_input(input);
    let visited = solve(map).unwrap();
    visited.iter().filter(|(&c, _)| c).count()
}

pub fn part2(input: &str) -> impl Display {
    let map = parse_input(input);

    let mut positions = Vec::with_capacity(map.width() * map.height());
    for i in 0..map.width() {
        for j in 0..map.height() {
            if map[i][j] == b'.' {
                positions.push((i, j))
            }
        }
    }

    positions
        .par_iter()
        .filter(|(i, j)| {
            let mut map = map.clone();
            map[*i][*j] = b'#';
            solve(map).is_none()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 6;

    const EXAMPLE: &str = indoc::indoc! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "41");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "5086");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "6");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1770");
    }
}
