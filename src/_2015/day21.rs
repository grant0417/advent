use crate::prelude::*;

#[derive(Debug, Default)]
struct Hda {
    hit_points: u32,
    damage: u32,
    armor: u32,
}

fn parse_input(input: &str) -> Hda {
    let mut hda = Hda::default();
    for line in input.lines() {
        if let Some(hit_points) = line.strip_prefix("Hit Points: ") {
            hda.hit_points = hit_points.parse().unwrap();
        } else if let Some(damage) = line.strip_prefix("Damage: ") {
            hda.damage = damage.parse().unwrap()
        } else if let Some(armor) = line.strip_prefix("Armor: ") {
            hda.armor = armor.parse().unwrap();
        }
    }
    hda
}

pub fn part1(input: &str) -> impl Display {
    let a = parse_input(input);
    println!("{a:?}");
    0
}

pub fn part2(input: &str) -> impl Display {
    let _ = parse_input(input);
    0
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 21;

    const EXAMPLE: &str = indoc! {"
        Hit Points: 8
        Damage: 5
        Armor: 5
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "0");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "0");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "0");
    }
}
