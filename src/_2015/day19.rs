use crate::prelude::*;

struct Input {
    replacements: Vec<(String, String)>,
    molecule: String,
}

fn parse_input(input: &str) -> Input {
    let (left, right) = input.split_once("\n\n").unwrap();

    let replacements = left
        .lines()
        .map(|line| {
            let (start, end) = line.split_once(" => ").unwrap();
            (start.into(), end.into())
        })
        .collect();

    Input {
        replacements,
        molecule: right.trim().into(),
    }
}

pub fn part1(input: &str) -> impl Display {
    let Input {
        replacements,
        molecule,
    } = parse_input(input);

    let re = Regex::new("[A-Z][a-z]?").unwrap();
    let mut new_molecules = HashSet::default();

    for element in re.find_iter(&molecule) {
        let replacements = replacements
            .iter()
            .filter(|(left, _)| *left == element.as_str())
            .map(|(_, right)| right.clone());

        for replacement in replacements {
            let mut molecule = molecule.clone();
            molecule.replace_range(element.range(), &replacement);
            new_molecules.insert(molecule);
        }
    }

    new_molecules.len()
}

pub fn part2(input: &str) -> impl Display {
    let Input {
        replacements,
        molecule,
    } = parse_input(input);

    println!("{replacements:?}");
    println!("{molecule}");

    0
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 19;

    const EXAMPLE: &str = indoc! {"
        e => H
        e => O
        H => HO
        H => OH
        O => HH

        HOHOHO
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "7");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "509");
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
