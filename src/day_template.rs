pub fn part1(input: &str) -> impl ToString {
    0
}

pub fn part2(input: &str) -> impl ToString {
    0
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 1;

    #[test]
    fn part1_example() {
        let input = "0";
        assert_eq!(part1(&input).to_string(), "0");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "0");
    }

    #[test]
    fn part2_example() {
        let input = "0";
        assert_eq!(part2(&input).to_string(), "0");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "0");
    }
}