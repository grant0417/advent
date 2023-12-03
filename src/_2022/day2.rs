#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn outcome_score(&self, other: &HandShape) -> u32 {
        match (self, other) {
            (HandShape::Rock, HandShape::Scissors)
            | (HandShape::Paper, HandShape::Rock)
            | (HandShape::Scissors, HandShape::Paper) => 6,
            (our, their) if our == their => 3,
            _ => 0,
        }
    }

    // Shape that loeses to this shape
    fn losing_shape(&self) -> HandShape {
        match self {
            HandShape::Rock => HandShape::Scissors,
            HandShape::Paper => HandShape::Rock,
            HandShape::Scissors => HandShape::Paper,
        }
    }

    // Shape that wins to this shape
    fn winning_shape(&self) -> HandShape {
        match self {
            HandShape::Rock => HandShape::Paper,
            HandShape::Paper => HandShape::Scissors,
            HandShape::Scissors => HandShape::Rock,
        }
    }

    fn draw_shape(&self) -> HandShape {
        *self
    }

    fn from_alpha(c: char) -> Option<HandShape> {
        match c {
            'A' | 'X' => Some(HandShape::Rock),
            'B' | 'Y' => Some(HandShape::Paper),
            'C' | 'Z' => Some(HandShape::Scissors),
            _ => None,
        }
    }

    fn score(&self) -> u32 {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }
}

pub fn part1(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let (their_alpha, our_alpha) = line.split_once(' ').unwrap();
            let their = HandShape::from_alpha(their_alpha.chars().next().unwrap()).unwrap();
            let our = HandShape::from_alpha(our_alpha.chars().next().unwrap()).unwrap();
            let score = our.outcome_score(&their) + our.score();
            println!("{:?} vs {:?} = {}", our, their, score);
            score
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let (their_alpha, outcome) = line.split_once(' ').unwrap();
            let their = HandShape::from_alpha(their_alpha.chars().next().unwrap()).unwrap();

            let our = match outcome.chars().next().unwrap() {
                'X' => their.losing_shape(),
                'Y' => their.draw_shape(),
                'Z' => their.winning_shape(),
                _ => unreachable!(),
            };

            let score = our.outcome_score(&their) + our.score();
            println!("{:?} vs {:?} = {}", our, their, score);
            score
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2022;
    const DAY: u32 = 2;

    const EXAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "15");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "12772");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "12");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "11618");
    }
}
