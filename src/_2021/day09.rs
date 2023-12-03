struct HeightMap {
    data: Vec<u32>,
    width: i32,
    height: i32,
}

impl HeightMap {
    fn new(width: i32, height: i32) -> Self {
        Self {
            data: vec![0; (width * height) as usize],
            width,
            height,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&u32> {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.data.get((y * self.width + x) as usize)
        } else {
            None
        }
    }

    fn set(&mut self, x: i32, y: i32, value: u32) {
        let v = self.data.get_mut((y * self.width + x) as usize);
        if let Some(pos) = v {
            *pos = value;
        }
    }

    fn get_adjacent(&self, x: i32, y: i32) -> Vec<&u32> {
        [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
            .iter()
            .filter_map(|(x, y)| self.get(*x, *y))
            .collect()
    }

    fn is_lowest_adjacent(&self, x: i32, y: i32) -> bool {
        let pos_val = self.get(x, y).unwrap();

        self.get_adjacent(x, y).iter().all(|&v| v > pos_val)
    }

    // Get the list of flooded tiles
    fn flood(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut queue = vec![(x, y)];
        let mut inside = Vec::new();

        while let Some((x, y)) = queue.pop() {
            if !inside.contains(&(x, y)) {
                let val = self.get(x, y).unwrap();

                if *val != 9 {
                    inside.push((x, y));

                    if x > 0 {
                        queue.push((x - 1, y));
                    }

                    if x < self.width - 1 {
                        queue.push((x + 1, y));
                    }

                    if y > 0 {
                        queue.push((x, y - 1));
                    }

                    if y < self.height - 1 {
                        queue.push((x, y + 1));
                    }
                }
            }
        }

        inside
    }
}

fn parse_input(input: impl AsRef<str>) -> HeightMap {
    let lines: Vec<_> = input.as_ref().lines().collect();
    let mut height_map = HeightMap::new(lines[0].len() as i32, lines.len() as i32);

    for (y, line) in lines.iter().enumerate() {
        for (x, i) in line.chars().enumerate() {
            height_map.set(x as i32, y as i32, i.to_digit(10).unwrap())
        }
    }

    height_map
}

pub fn part1(input: &str) -> impl ToString {
    let parsed_input = parse_input(input);

    let height = parsed_input.height;
    let width = parsed_input.width;

    let mut sum = 0;

    for y in 0..height {
        for x in 0..width {
            if parsed_input.is_lowest_adjacent(x, y) {
                sum += parsed_input.get(x, y).unwrap() + 1;
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> impl ToString {
    let parsed_input = parse_input(input);

    let height = parsed_input.height;
    let width = parsed_input.width;

    let mut lowest_points = vec![];

    for y in 0..height {
        for x in 0..width {
            if parsed_input.is_lowest_adjacent(x, y) {
                lowest_points.push((x, y));
            }
        }
    }

    let mut three_largest_flood = [0; 3];

    for (x, y) in lowest_points {
        let flood = parsed_input.flood(x, y).len();
        if flood > three_largest_flood[0] {
            three_largest_flood[2] = three_largest_flood[1];
            three_largest_flood[1] = three_largest_flood[0];
            three_largest_flood[0] = flood;
        } else if flood > three_largest_flood[1] {
            three_largest_flood[2] = three_largest_flood[1];
            three_largest_flood[1] = flood;
        } else if flood > three_largest_flood[2] {
            three_largest_flood[2] = flood;
        }
    }

    three_largest_flood.iter().product::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u32 = 9;

    const EXAMPLE: &str = indoc::indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "15");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "591");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "1134");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1113424");
    }
}
