pub fn part1(input: &str) -> String {
    input.to_string()
}

pub fn part2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const DAY: u32 = 0;

    #[test]
    fn part1_example() {
        let input = "0";
        assert_eq!(part1(&input), "0");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(DAY).await;
        assert_eq!(part1(&input), "0");
    }

    #[test]
    fn part2_example() {
        let input = "0";
        assert_eq!(part2(&input), "0");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(DAY).await;
        assert_eq!(part2(&input), "0");
    }
}
