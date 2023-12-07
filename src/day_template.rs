fn parse_input(input: &str) -> String {
    input.to_owned()
}

pub fn part1(input: &str) -> impl ToString {
    let _ = parse_input(input);
    0
}

pub fn part2(input: &str) -> impl ToString {
    let _ = parse_input(input);
    0
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 1;

    const EXAMPLE: &str = indoc::indoc! {"
        0
        1
        2
        3
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "0");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "0");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "0");
    }
}
