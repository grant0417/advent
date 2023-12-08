use aho_corasick::AhoCorasick;

pub fn part1(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.chars().filter(|c| c.is_ascii_digit());
            let first = numbers.next().unwrap().to_digit(10).unwrap();
            let last = numbers.last().map_or(first, |c| c.to_digit(10).unwrap());
            first * 10 + last
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl ToString {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let ac = AhoCorasick::new(nums).unwrap();

    input
        .lines()
        .map(|line| {
            let mut find = ac.find_overlapping_iter(line);
            let first = find.next().map(|m| m.pattern().as_u32() % 9 + 1).unwrap();
            let last = find.last().map_or(first, |m| m.pattern().as_u32() % 9 + 1);

            first * 10 + last
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 1;

    #[test]
    fn part1_example() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part1(input).to_string(), "142");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "54601");
    }

    #[test]
    fn part2_example() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part2(input).to_string(), "281");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "54078");
    }
}
