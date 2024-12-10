use crate::prelude::*;

#[derive(Debug)]
enum Op {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    op: Op,
    start: Point<usize>,
    end: Point<usize>,
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    let re = Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    input.lines().map(move |line| {
        let c = re.captures(line).unwrap();
        Instruction {
            op: match &c[1] {
                "turn on" => Op::TurnOn,
                "turn off" => Op::TurnOff,
                "toggle" => Op::Toggle,
                _ => panic!(),
            },
            start: Point::new(c[2].parse().unwrap(), c[3].parse().unwrap()),
            // Add 1 to make this an exclusive range, makes the later code nicer
            end: Point::new(
                c[4].parse::<usize>().unwrap() + 1,
                c[5].parse::<usize>().unwrap() + 1,
            ),
        }
    })
}

pub fn part1(input: &str) -> impl Display {
    let mut grid: Grid<bool> = Grid::new(1000, 1000);
    for Instruction { op, start, end } in parse_input(input) {
        match op {
            Op::TurnOn => grid.fill(start, end, true),
            Op::TurnOff => grid.fill(start, end, false),
            Op::Toggle => {
                for y in start.y..end.y {
                    for x in start.x..end.x {
                        grid[Point::new(x, y)] = !grid[Point::new(x, y)];
                    }
                }
            }
        }
    }
    grid.iter().filter(|(&b, _)| b).count()
}

pub fn part2(input: &str) -> impl Display {
    let mut grid: Grid<i16> = Grid::new(1000, 1000);
    for Instruction { op, start, end } in parse_input(input) {
        let val = match op {
            Op::TurnOn => 1,
            Op::TurnOff => -1,
            Op::Toggle => 2,
        };

        for y in start.y..end.y {
            for x in start.x..end.x {
                grid[Point::new(x, y)] = (grid[Point::new(x, y)] + val).max(0);
            }
        }
    }
    grid.iter().map(|(&b, _)| b as i64).sum::<i64>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 6;

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "543903");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "14687245");
    }
}
