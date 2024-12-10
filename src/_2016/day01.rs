use crate::prelude::*;

fn parse_input(input: &str) -> impl Iterator<Item = &str> {
    input.trim().split(", ")
}

pub fn part1(input: &str) -> impl Display {
    let start = Point::new(0, 0);
    let mut p = start;
    let mut direction: i32 = 0;

    for m in parse_input(input) {
        if &m[0..1] == "R" {
            direction += 1;
        } else {
            direction -= 1;
        };
        let distance: i32 = m[1..].parse().unwrap();

        match direction.rem_euclid(4) {
            0 => p.y += distance, // north
            1 => p.x += distance, // east
            2 => p.y -= distance, // south
            3 => p.x -= distance, // west
            _ => unreachable!(),
        }
    }
    p.manhattan_distance(&start)
}

pub fn part2(input: &str) -> impl Display {
    let start: Point<i32> = Point::new(0, 0);
    let mut p = start;
    let mut direction: i32 = 0;
    let mut visited = HashSet::default();

    for m in parse_input(input) {
        if &m[0..1] == "R" {
            direction += 1;
        } else {
            direction -= 1;
        };
        let distance: i32 = m[1..].parse().unwrap();

        for _ in 0..distance {
            match direction.rem_euclid(4) {
                0 => p.y += 1, // north
                1 => p.x += 1, // east
                2 => p.y -= 1, // south
                3 => p.x -= 1, // west
                _ => unreachable!(),
            }
            if !visited.insert(p) {
                return p.manhattan_distance(&start);
            };
        }
    }
    panic!("cound not find intersection")
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2016;
    const DAY: u32 = 1;

    #[test]
    fn part1_example() {
        assert_eq!(part1("R5, L5, R5, R3").to_string(), "12");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "253");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("R8, R4, R4, R8").to_string(), "4");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "126");
    }
}
