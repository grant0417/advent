struct OctopusGrid {
    points: Vec<u32>,
    width: i32,
    height: i32,
}

impl OctopusGrid {
    fn step(&mut self) -> usize {
        for point in self.points.iter_mut() {
            *point += 1;
        }

        let mut flashed = vec![false; (self.width * self.height).try_into().unwrap()];

        'outer_loop: loop {
            for y in 0..self.height {
                for x in 0..self.width {
                    let index: usize = (self.height * y + x).try_into().unwrap();

                    if !flashed[index] && self.points[index] > 9 {
                        flashed[index] = true;
                        self.flash_point(x, y);
                        continue 'outer_loop;
                    }
                }
            }

            break;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let index: usize = (self.height * y + x).try_into().unwrap();

                if self.points[index] >= 10 {
                    self.points[index] = 0;
                }
            }
        }

        flashed.into_iter().filter(|b| *b).count()
    }

    fn flash_point(&mut self, x: i32, y: i32) {
        let x_min = 0.max(x - 1);
        let x_max = (self.width - 1).min(x + 1);

        let y_min = 0.max(y - 1);
        let y_max = (self.height - 1).min(y + 1);

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let index: usize = (self.height * y + x).try_into().unwrap();
                self.points[index] += 1;
            }
        }
    }
}

fn parse_input(input: impl AsRef<str>) -> OctopusGrid {
    let lines: Vec<_> = input.as_ref().lines().collect();

    let height = lines.len();
    let width = lines[0].chars().count();

    let points = lines
        .into_iter()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();

    OctopusGrid {
        width: width.try_into().unwrap(),
        height: height.try_into().unwrap(),
        points,
    }
}

pub fn part1(input: &str) -> impl ToString {
    let mut octopus_grid = parse_input(input);

    let flash_count: usize = (0..100).map(|_| octopus_grid.step()).sum();

    flash_count
}

pub fn part2(input: &str) -> impl ToString {
    let mut octopus_grid = parse_input(input);

    let mut i = 1;

    loop {
        let flashed = octopus_grid.step();

        if flashed
            == (octopus_grid.width * octopus_grid.height)
                .try_into()
                .unwrap()
        {
            break;
        }

        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u32 = 11;

    const EXAMPLE: &str = indoc::indoc! {"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "1656");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "1773");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "195");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "494");
    }
}
