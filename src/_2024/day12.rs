use crate::prelude::*;

const DIRECTIONS: [Point<i64>; 4] = [
    Point::new(-1, 0),
    Point::new(1, 0),
    Point::new(0, -1),
    Point::new(0, 1),
];

fn parse_input(input: &str) -> Grid<u8> {
    Grid::parse_bytes(input)
}

struct Output {
    perimeter_price: usize,
    side_price: usize,
}

fn solve(grid: &Grid<u8>) -> Output {
    let mut regions = HashSet::default();

    // flood each region
    for (search_plant, search_point) in grid.iter() {
        let mut queue = vec![search_point];
        let mut visited = HashSet::default();
        let mut local_region = Vec::new();

        while let Some(queue_point) = queue.pop() {
            if let Some(plant) = grid.get(queue_point) {
                if plant == search_plant {
                    let p = queue_point + Point::new(1, 1);
                    if let Err(i) = local_region.binary_search(&p) {
                        local_region.insert(i, p);
                    }

                    queue.extend(DIRECTIONS.iter().filter_map(|direction| {
                        let point: Point<usize> = Point::new(
                            direction.x + queue_point.x as i64,
                            direction.y + queue_point.y as i64,
                        )
                        .try_into()
                        .ok()?;
                        visited.insert(point).then_some(point)
                    }));
                }
            }
        }

        regions.insert(local_region);
    }

    let mut perimeter_price = 0;
    let mut side_price = 0;

    for region in &regions {
        // find perimeter by growing by 1
        let mut grow_1 = HashSet::default();
        for point in region {
            grow_1.extend(DIRECTIONS.iter().filter_map(|direction| {
                let point = Point::new(point.x as i64 + direction.x, point.y as i64 + direction.y)
                    .try_into()
                    .ok()?;
                Some((point, *direction))
            }));
        }

        let edge_points: Vec<(Point<i64>, Point<i64>)> = grow_1
            .iter()
            .filter(|(a, _)| !region.contains(&a))
            .map(|(a, b)| ((*a).try_into().unwrap(), *b))
            .sorted()
            .collect::<Vec<_>>();
        let perimeter = edge_points.len();

        // find edges
        let mut edges: Vec<Vec<(Point<i64>, Point<i64>)>> = Vec::new();
        let mut i = 0;
        'o: while i < edge_points.len() {
            let (point_a, normal_a) = edge_points[i];
            for edge in edges.iter_mut() {
                for (point_b, normal_b) in edge.clone().iter() {
                    if normal_a == *normal_b
                        && (DIRECTIONS.iter().any(|n| *n + point_a == *point_b))
                    {
                        edge.push((point_a, normal_a));
                        i += 1;
                        continue 'o;
                    }
                }
            }

            edges.push(vec![(point_a, normal_a)]);
            i += 1;
        }

        perimeter_price += region.len() * perimeter;
        side_price += region.len() * edges.len();
    }

    Output {
        perimeter_price,
        side_price,
    }
}

pub fn part1(input: &str) -> impl Display {
    let grid = parse_input(input);
    solve(&grid).perimeter_price
}

pub fn part2(input: &str) -> impl Display {
    let grid = parse_input(input);
    solve(&grid).side_price
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 12;

    const EXAMPLE_1: &str = indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC
    "};

    const EXAMPLE_2: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE_1).to_string(), "140");
        assert_eq!(part1(EXAMPLE_2).to_string(), "1930");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "1396298");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE_1).to_string(), "80");
        assert_eq!(part2(EXAMPLE_2).to_string(), "1206");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "853588");
    }
}
