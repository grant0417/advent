use crate::prelude::*;

fn parse_input(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut lines = input.lines();

    let directions = lines.next().unwrap().chars().collect();

    lines.next();

    let nodes = lines
        .map(|line| {
            let mut parts = line.split(" = ");
            let key = parts.next().unwrap().to_string();
            let value = parts
                .next()
                .unwrap()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(", ");
            let left = value.clone().next().unwrap().to_string();
            let right = value.clone().last().unwrap().to_string();

            (key, (left, right))
        })
        .collect::<HashMap<_, _>>();

    (directions, nodes)
}

pub fn part1(input: &str) -> impl ToString {
    let (directions, nodes) = parse_input(input);

    let mut node = "AAA".to_owned();
    let mut count = 0;

    loop {
        for direction in directions.iter() {
            count += 1;
            let (left, right) = nodes.get(&node).unwrap();
            node = match direction {
                'L' => left.clone(),
                'R' => right.clone(),
                _ => unreachable!(),
            };

            if node == "ZZZ" {
                return count;
            }
        }
    }
}

pub fn part2(input: &str) -> impl ToString {
    let (directions, nodes) = parse_input(input);

    let start_nodes = nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    let mut counts = vec![];

    for node in start_nodes {
        let mut count: usize = 0;
        let mut node = node;
        'outer_loop: loop {
            for direction in directions.iter() {
                count += 1;
                let (left, right) = nodes.get(&node).unwrap();
                node = match direction {
                    'L' => left.clone(),
                    'R' => right.clone(),
                    _ => unreachable!(),
                };

                if node.ends_with('Z') {
                    break 'outer_loop;
                }
            }
        }
        counts.push(count);
    }

    lcm(counts.into_iter())
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 8;

    const PART1_EXAMPLE_1: &str = indoc::indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    const PART1_EXAMPLE_2: &str = indoc::indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(PART1_EXAMPLE_1).to_string(), "2");
        assert_eq!(part1(PART1_EXAMPLE_2).to_string(), "6");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "19199");
    }

    const PART2_EXAMPLE: &str = indoc::indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn part2_example() {
        assert_eq!(part2(PART2_EXAMPLE).to_string(), "6");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "13663968099527");
    }
}
