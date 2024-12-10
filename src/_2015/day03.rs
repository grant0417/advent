use crate::prelude::*;

fn parse_input(input: &str) -> impl Iterator<Item = char> + '_ {
    input.trim().chars()
}

pub fn part1(input: &str) -> impl Display {
    let mut seen = HashSet::default();
    let mut position = Point::new(0, 0);
    seen.insert(position);

    for c in parse_input(input) {
        match c {
            '>' => position.x += 1,
            '<' => position.x -= 1,
            '^' => position.y += 1,
            'v' => position.y -= 1,
            _ => {}
        }
        seen.insert(position);
    }
    seen.len()
}

pub fn part2(input: &str) -> impl Display {
    let mut seen = HashSet::default();
    let mut santa_pos = Point::new(0, 0);
    let mut robot_pos = Point::new(0, 0);
    seen.insert(santa_pos);

    for (i, c) in parse_input(input).enumerate() {
        let position = if i % 2 == 0 {
            &mut santa_pos
        } else {
            &mut robot_pos
        };
        match c {
            '>' => position.x += 1,
            '<' => position.x -= 1,
            '^' => position.y += 1,
            'v' => position.y -= 1,
            _ => {}
        }
        seen.insert(*position);
    }
    seen.len()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 3;

    const EXAMPLE: &str = "^v^v^v^v^v";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "2");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "2081");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "11");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "2341");
    }
}
