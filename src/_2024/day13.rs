use crate::prelude::*;

#[derive(Debug, Clone)]
struct Machine {
    a: Point<i64>,
    b: Point<i64>,
    d: Point<i64>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|m| {
            let re = Regex::new(
                r"Button A: X\+(\d+), Y\+(\d+)\s*Button B: X\+(\d+), Y\+(\d+)\s*Prize: X=(\d+), Y=(\d+)",
            )
            .unwrap();

            let captures = re.captures(m).unwrap();
            Machine {
                a: Point::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                b: Point::new(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
                d: Point::new(captures[5].parse().unwrap(), captures[6].parse().unwrap()),
            }
        })
        .collect()
}

fn find_min_c(machine: &Machine) -> Option<i64> {
    let Machine { a, b, d } = machine;

    // Check if b.x is zero (would cause division by zero)
    if b.x == 0 {
        return None;
    }

    // Calculate numerator and denominator for a
    let numerator = d.y * b.x - d.x * b.y;
    let denominator = a.y * b.x - a.x * b.y;

    // Check if denominator is zero (equations are linearly dependent)
    if denominator == 0 {
        return None;
    }

    // Check if we can get an integer solution for a
    if numerator % denominator != 0 {
        return None;
    }

    let a_count = numerator / denominator;

    // Calculate numerator for b
    let numerator_b = d.x - a_count * a.x;

    // Check if we can get an integer solution for b
    if numerator_b % b.x != 0 {
        return None;
    }

    let b_count = numerator_b / b.x;

    // Calculate minimum
    Some(3 * a_count + b_count)
}

pub fn part1(input: &str) -> impl Display {
    let machines = parse_input(input);
    machines
        .iter()
        .map(|m| find_min_c(&m).unwrap_or(0i64))
        .sum::<i64>()
}

pub fn part2(input: &str) -> impl Display {
    let offset = 10000000000000;
    let machines = parse_input(input);
    machines
        .iter()
        .cloned()
        .map(|mut m| {
            m.d += Point::new(offset, offset);
            find_min_c(&m).unwrap_or(0i64)
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 13;

    const EXAMPLE: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "480");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "32026");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "875318608908");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "89013607072065");
    }
}
