use crate::prelude::*;

pub fn part1(input: &str) -> impl Display {
    let n: usize = input.trim().parse().unwrap();

    let r = (1_usize..1_000_000_000)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|i| {
            let mut sum = 0;
            for j in 1..=*i {
                if i % j == 0 {
                    sum += j * 10;
                }
            }
            sum >= n
        });

    r.unwrap()
}

pub fn part2(input: &str) -> impl Display {
    let n: usize = input.trim().parse().unwrap();

    let r = (1_usize..1_000_000_000)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|i| {
            let mut sum = 0;
            for j in 1..=*i {
                if i % j == 0 && i / j <= 50 {
                    sum += j * 11;
                }
            }
            sum >= n
        });

    r.unwrap()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 20;

    #[ignore = "slow bruteforce"]
    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "665280");
    }

    #[ignore = "slow bruteforce"]
    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "705600");
    }
}
