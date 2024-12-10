use std::cmp::Ordering;

use crate::prelude::*;

#[derive(Debug, Default)]
struct Sue {
    id: u32,
    items: HashMap<String, u32>,
}

fn parse_input(input: &str) -> Vec<Sue> {
    input
        .lines()
        .map(|line| {
            let (_left, right) = line.split_once(": ").unwrap();
            let id = _left.strip_prefix("Sue ").unwrap().parse().unwrap();
            let mut items = HashMap::default();
            for pair in right.split(", ") {
                let (name, count) = pair.split_once(": ").unwrap();
                items.insert(name.to_owned(), count.parse().unwrap());
            }
            Sue { id, items }
        })
        .collect()
}

fn solve(input: &str, use_ordering: bool) -> impl Display {
    let sues = parse_input(input);

    let map: HashMap<_, _> = [
        ("children", (3, Ordering::Equal)),
        ("cats", (7, Ordering::Greater)),
        ("samoyeds", (2, Ordering::Equal)),
        ("pomeranians", (3, Ordering::Equal)),
        ("akitas", (0, Ordering::Equal)),
        ("vizslas", (0, Ordering::Equal)),
        ("goldfish", (5, Ordering::Less)),
        ("trees", (3, Ordering::Greater)),
        ("cars", (2, Ordering::Equal)),
        ("perfumes", (1, Ordering::Less)),
    ]
    .into_iter()
    .map(|(a, b)| (a.to_owned(), b))
    .collect();

    'sue_loop: for sue in sues {
        for (item, &count) in &sue.items {
            if let Some(&(c, ordering)) = map.get(item) {
                if use_ordering {
                    if count.cmp(&c) != ordering {
                        continue 'sue_loop;
                    }
                } else if c != count {
                    continue 'sue_loop;
                }
            }
        }
        return sue.id;
    }

    panic!("None found");
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
    const DAY: u32 = 16;

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "373");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "260");
    }
}
