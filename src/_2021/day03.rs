pub fn part1(input: &str) -> impl ToString {
    let bit_count = input.lines().next().unwrap().len();

    let mut zero_count = vec![0; bit_count];
    let mut one_count = vec![0; bit_count];

    for line in input.lines() {
        for (i, bit) in line.chars().enumerate() {
            let bit = bit.to_digit(10).unwrap();
            if bit == 0 {
                zero_count[i] += 1;
            } else {
                one_count[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..bit_count {
        if one_count[bit_count - i - 1] > zero_count[bit_count - i - 1] {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    gamma * epsilon
}

pub fn part2(input: &str) -> impl ToString {
    let bit_count = input.lines().next().unwrap().len();

    // Find oxygen generator rating
    let mut oxygen_ratings = input.lines().map(|s| s.to_owned()).collect::<Vec<String>>();

    for i in 0..bit_count {
        let mut zero_count = 0;
        let mut one_count = 0;

        for line in &oxygen_ratings {
            if line.chars().nth(i).unwrap() == '0' {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }

        let most_common = if zero_count > one_count { '0' } else { '1' };

        // Delete lines that don't have the most common bit
        oxygen_ratings = oxygen_ratings
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == most_common)
            .collect::<Vec<_>>();
    }

    // Find co2 scrubber rating
    let mut co2_ratings = input.lines().map(|s| s.to_owned()).collect::<Vec<String>>();

    for i in 0..bit_count {
        let mut zero_count = 0;
        let mut one_count = 0;

        for line in &co2_ratings {
            if line.chars().nth(i).unwrap() == '0' {
                zero_count += 1;
            } else {
                one_count += 1;
            }
        }

        let least_common = if zero_count > one_count { '1' } else { '0' };

        if co2_ratings.len() == 1 {
            break;
        }

        // Delete lines that don't have the least common bit
        co2_ratings = co2_ratings
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == least_common)
            .collect::<Vec<_>>();
    }

    // Convert to bit string to int
    let mut oxygen_rating = 0;
    let mut co2_rating = 0;

    for (j, bit) in oxygen_ratings[0].chars().enumerate() {
        if bit == '1' {
            oxygen_rating += 1 << (bit_count - j - 1);
        }
    }

    for (j, bit) in co2_ratings[0].chars().enumerate() {
        if bit == '1' {
            co2_rating += 1 << (bit_count - j - 1);
        }
    }

    oxygen_rating * co2_rating
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u32 = 3;

    const EXAMPLE: &str = indoc::indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "198");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "749376");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "230");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "2372923");
    }
}
