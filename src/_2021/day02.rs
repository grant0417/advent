pub fn part1(input: &str) -> impl ToString {
    let mut depth = 0;
    let mut hor_distance = 0;

    for line in input.lines() {
        let mut data = line.split(' ');

        let direction = data.next().unwrap();
        let x = data.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => hor_distance += x,
            "up" => depth -= x,
            "down" => depth += x,
            _ => panic!("Unknown direction: {direction}"),
        }
    }

    hor_distance * depth
}

pub fn part2(input: &str) -> impl ToString {
    let mut aim = 0;
    let mut hor_distance = 0;
    let mut depth = 0;

    for line in input.lines() {
        let mut data = line.split(' ');

        let direction = data.next().unwrap();
        let x = data.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => {
                hor_distance += x;
                depth += x * aim;
            }
            "up" => aim -= x,
            "down" => aim += x,
            _ => panic!("Unknown direction: {direction}"),
        }
    }

    format!("{}", hor_distance * depth)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u32 = 2;

    const EXAMPLE: &str = indoc::indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "150");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "1815044");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "900");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1739283308");
    }
}
