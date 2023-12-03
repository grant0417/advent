fn parse_input(input: impl AsRef<str>) -> Vec<i32> {
    input
        .as_ref()
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect::<Vec<_>>()
}

fn find_range(vals: &[i32]) -> (i32, i32) {
    (*vals.iter().min().unwrap(), *vals.iter().max().unwrap())
}

fn min_fule_linear(vals: &[i32]) -> i32 {
    let (min, max) = find_range(vals);

    (min..=max)
        .map(|end_location| vals.iter().map(|crab| (end_location - crab).abs()).sum())
        .min()
        .unwrap()
}

fn min_fule_exp(vals: &[i32]) -> i32 {
    let (min, max) = find_range(vals);

    (min..=max)
        .map(|end_location| {
            vals.iter()
                .map(|crab| {
                    let n = (end_location - crab).abs();
                    (n * (1 + n)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

pub fn part1(input: &str) -> impl ToString {
    let parsed_input = parse_input(input);
    min_fule_linear(&parsed_input)
}

pub fn part2(input: &str) -> impl ToString {
    let parsed_input = parse_input(input);
    min_fule_exp(&parsed_input)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u32 = 7;

    const EXAMPLE: &str = indoc::indoc! {"
        16,1,2,0,4,2,7,1,2,14
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "37");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "352254");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "168");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "99053143");
    }
}
