fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn comput_fish_count(input: &str, day: usize) -> usize {
    let parsed_input = parse_input(input);
    let mut fish_cohorts = [0usize; 9];

    for i in parsed_input {
        fish_cohorts[i] += 1;
    }

    for _ in 0..day {
        fish_cohorts.rotate_left(1);
        fish_cohorts[6] += fish_cohorts[8];
    }

    fish_cohorts.iter().sum()
}

pub fn part1(input: &str) -> impl ToString {
    comput_fish_count(input, 80)
}

pub fn part2(input: &str) -> impl ToString {
    comput_fish_count(input, 256)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u32 = 6;

    const EXAMPLE: &str = indoc::indoc! {"
        3,4,3,1,2
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "5934");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "379414");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "26984457539");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1705008653296");
    }
}
