use crate::prelude::*;

fn parse_input(input: &str) -> impl Iterator<Item = (u32, u32, u32)> + '_ {
    input.lines().map(|line| {
        let mut split = line.split('x');
        let l = split.next().unwrap().parse().unwrap();
        let w = split.next().unwrap().parse().unwrap();
        let h = split.next().unwrap().parse().unwrap();
        (l, w, h)
    })
}

pub fn part1(input: &str) -> impl Display {
    parse_input(input)
        .map(|(l, w, h)| {
            let lw = l * w;
            let wh = w * h;
            let hl = h * l;
            2 * lw + 2 * wh + 2 * hl + lw.min(wh).min(hl)
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl Display {
    parse_input(input)
        .map(|(l, w, h)| {
            let mut sorted = [l, w, h];
            sorted.sort();
            l * w * h + 2 * sorted[0] + 2 * sorted[1]
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 2;

    const EXAMPLE: &str = "2x3x4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "58");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "1588178");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "34");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "3783758");
    }
}
