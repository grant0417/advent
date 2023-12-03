pub fn part1(input: &str) -> impl ToString {
    let values = input.lines().map(|x| x.parse::<i32>().unwrap());

    let mut increments = 0;
    let mut previous_value = None;

    for value in values {
        if let Some(v) = previous_value {
            if value > v {
                increments += 1;
            }
        }

        previous_value = Some(value);
    }

    increments
}

pub fn part2(input: &str) -> impl ToString {
    let values = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let windows = values.windows(3).map(|window| window.iter().sum::<i32>());

    let mut increments = 0;
    let mut previous_value = None;

    for value in windows {
        if let Some(v) = previous_value {
            if value > v {
                increments += 1;
            }
        }

        previous_value = Some(value);
    }

    increments
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u32 = 1;

    const EXAMPLE: &str = indoc::indoc! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "7");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "1466");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "5");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1491");
    }
}
