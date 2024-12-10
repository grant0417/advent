use crate::prelude::*;

#[derive(Debug)]
struct Entry {
    person: String,
    target: String,
    value: i32,
}

fn parse_input(input: &str) -> impl Iterator<Item = Entry> + '_ {
    let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).")
        .unwrap();
    input.lines().map(move |line| {
        let c = re.captures(line).unwrap();
        Entry {
            person: c[1].into(),
            target: c[4].into(),
            value: if &c[2] == "gain" { 1 } else { -1 } * c[3].parse::<i32>().unwrap(),
        }
    })
}

fn solve(input: &str, include_self: bool) -> i32 {
    let mut people = Vec::new();
    let mut opinion_map = HashMap::default();

    for entry in parse_input(input) {
        if let Err(idx) = people.binary_search(&entry.person) {
            people.insert(idx, entry.person.clone());
        }
        opinion_map.insert((entry.person, entry.target), entry.value);
    }

    if include_self {
        people.push("self".into());
    }

    people
        .iter()
        .permutations(people.len())
        .par_bridge()
        .map(|p| {
            let mut val = 0;
            for i in 0..p.len() {
                val += opinion_map
                    .get(&(p[i].clone(), p[(i + 1) % p.len()].clone()))
                    .unwrap_or(&0);
                val += opinion_map
                    .get(&(p[(i + 1) % p.len()].clone(), p[i].clone()))
                    .unwrap_or(&0);
            }
            val
        })
        .max()
        .unwrap()
}

pub fn part1(input: &str) -> impl Display {
    solve(input, false)
}

pub fn part2(input: &str) -> impl Display {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 13;

    const EXAMPLE: &str = indoc! {"
        Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "330");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "733");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "286");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "725");
    }
}
