use crate::prelude::*;

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn brute_force(containers: &[u32], target: u32) -> Vec<u32> {
    let mut v = vec![0; containers.len()];

    for i in 0..2u32.pow(containers.len() as u32) {
        let mut j = i;
        let mut sum = 0;
        for c in containers {
            if j % 2 == 1 {
                sum += c;
            }
            j /= 2;
            if j == 0 {
                break;
            }
        }
        if sum == target {
            v[i.count_ones() as usize] += 1
        }
    }
    v
}

pub fn part1(input: &str) -> impl Display {
    let containers = parse_input(input);
    let eggnog = 150;
    brute_force(&containers, eggnog).iter().sum::<u32>()
}

pub fn part2(input: &str) -> impl Display {
    let containers = parse_input(input);
    let eggnog = 150;
    brute_force(&containers, eggnog)
        .into_iter()
        .find(|&v| v != 0)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 17;

    #[test]
    fn example() {
        let v = brute_force(&[20, 15, 10, 5, 5], 25);
        assert_eq!(v.iter().sum::<u32>(), 4);
        assert_eq!(*v.iter().find(|&&v| v != 0).unwrap(), 3);
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "1638");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "17");
    }
}
