use core::cmp::Ordering;
use std::{cmp, fmt, panic};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.y.cmp(&other.y)
    }
}

impl cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineType {
    Horizontal,
    Vertical,
    // 45 degree forwards
    ForwardDiagonal,
    // 45 degree backwards
    BackwardDiagonal,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    fn line_type(&self) -> LineType {
        let left_point = self.start.min(self.end);
        let right_point = self.start.max(self.end);

        match (left_point.x, left_point.y, right_point.x, right_point.y) {
            (x, _, x2, _) if x == x2 => LineType::Vertical,
            (_, y, _, y2) if y == y2 => LineType::Horizontal,
            (x, y, x2, y2) if x < x2 && y < y2 => LineType::ForwardDiagonal,
            (x, y, x2, y2) if x < x2 && y > y2 => LineType::BackwardDiagonal,
            _ => panic!("unexpected line type"),
        }
    }
}

#[derive(Debug, Clone)]
struct SeaBed {
    width: usize,
    height: usize,
    grid: Vec<i32>,
}

impl fmt::Display for SeaBed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = y * self.width + x;
                let c = match self.grid[i] {
                    0 => '.',
                    n => char::from_digit(n as u32, 16).unwrap_or('+'),
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl SeaBed {
    fn new(width: usize, height: usize) -> SeaBed {
        SeaBed {
            width,
            height,
            grid: vec![0; width * height],
        }
    }

    fn plot(&mut self, point: &Point) {
        let index = point.y * self.width as i32 + point.x;
        self.grid[index as usize] += 1;
    }

    fn plot_horizontal_vertical(&mut self, line: &Line) {
        // Only concider horizontal or vertical lines
        let line_type = line.line_type();

        // horizontal line
        if line_type == LineType::Horizontal {
            let y = line.start.y;
            let x_start = line.start.x.min(line.end.x);
            let x_end = line.start.x.max(line.end.x);

            for x in x_start..=x_end {
                self.plot(&Point { x, y });
            }
        }

        // vertical line
        if line_type == LineType::Vertical {
            let x = line.start.x;
            let y_start = line.start.y.min(line.end.y);
            let y_end = line.start.y.max(line.end.y);

            for y in y_start..=y_end {
                self.plot(&Point { x, y });
            }
        }
    }

    fn plot_line(&mut self, line: &Line) {
        let line_type = line.line_type();

        match line_type {
            LineType::Horizontal | LineType::Vertical => self.plot_horizontal_vertical(line),
            LineType::ForwardDiagonal | LineType::BackwardDiagonal => {
                let left_point = line.start.min(line.end);
                let right_point = line.start.max(line.end);

                let dx = right_point.x - left_point.x;

                for i in 0..=dx {
                    let x = left_point.x + i;
                    let y = if line_type == LineType::ForwardDiagonal {
                        left_point.y + i
                    } else {
                        left_point.y - i
                    };
                    self.plot(&Point { x, y });
                }
            }
        }
    }

    fn count_danger(&self) -> usize {
        self.grid.iter().filter(|&c| *c >= 2).count()
    }
}

pub fn part1(input: &str) -> impl ToString {
    let mut max_x = 0;
    let mut max_y = 0;

    let lines: Vec<_> = input
        .lines()
        .map(|line| {
            let mut points = line.split(" -> ").map(|coord_pair| {
                let mut coord_pair_iterator =
                    coord_pair.split(',').map(|s| s.parse::<i32>().unwrap());

                let point = Point {
                    x: coord_pair_iterator.next().unwrap(),
                    y: coord_pair_iterator.next().unwrap(),
                };

                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);

                point
            });
            let point_a = points.next().unwrap();
            let point_b = points.next().unwrap();

            match point_a.cmp(&point_b) {
                Ordering::Less => Line::new(point_a, point_b),
                Ordering::Greater => Line::new(point_b, point_a),
                Ordering::Equal => panic!("Invalid line"),
            }
        })
        .collect();

    let mut sea_bed = SeaBed::new(
        (max_x + 1).try_into().unwrap(),
        (max_y + 1).try_into().unwrap(),
    );

    for line in lines {
        sea_bed.plot_horizontal_vertical(&line);
    }

    sea_bed.count_danger()
}

pub fn part2(input: &str) -> impl ToString {
    let mut max_x = 0;
    let mut max_y = 0;

    let lines: Vec<_> = input
        .lines()
        .map(|line| {
            let mut points = line.split(" -> ").map(|coord_pair| {
                let mut coord_pair_iterator =
                    coord_pair.split(',').map(|s| s.parse::<i32>().unwrap());

                let point = Point {
                    x: coord_pair_iterator.next().unwrap(),
                    y: coord_pair_iterator.next().unwrap(),
                };

                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);

                point
            });
            let point_a = points.next().unwrap();
            let point_b = points.next().unwrap();

            match point_a.cmp(&point_b) {
                Ordering::Less => Line::new(point_a, point_b),
                Ordering::Greater => Line::new(point_b, point_a),
                Ordering::Equal => panic!("Invalid line"),
            }
        })
        .collect();

    let mut sea_bed = SeaBed::new(
        (max_x + 1).try_into().unwrap(),
        (max_y + 1).try_into().unwrap(),
    );

    for line in lines {
        sea_bed.plot_line(&line);
    }

    sea_bed.count_danger()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u32 = 5;

    const EXAMPLE: &str = indoc::indoc! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "5");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "6548");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "12");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "19663");
    }
}
