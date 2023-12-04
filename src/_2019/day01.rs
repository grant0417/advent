pub fn need_fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

pub fn need_fuel_recursive(mass: u32) -> u32 {
    let mut total_fuel = 0;
    let mut fuel = need_fuel(mass);

    while fuel > 0 {
        total_fuel += fuel;
        fuel = need_fuel(fuel);
    }

    total_fuel
}

pub fn part1(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .map(need_fuel)
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .map(need_fuel_recursive)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2019;
    const DAY: u32 = 1;

    #[test]
    fn part1_example() {
        assert_eq!(part1("12").to_string(), "2");
        assert_eq!(part1("14").to_string(), "2");
        assert_eq!(part1("1969").to_string(), "654");
        assert_eq!(part1("100756").to_string(), "33583");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "3246455");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("14").to_string(), "2");
        assert_eq!(part2("1969").to_string(), "966");
        assert_eq!(part2("100756").to_string(), "50346");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "4866824");
    }
}
