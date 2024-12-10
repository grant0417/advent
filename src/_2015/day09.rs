use crate::prelude::*;

#[derive(Debug, PartialEq)]
struct Route {
    start: String,
    end: String,
    distance: u32,
}

fn parse_input(input: &str) -> Vec<Route> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" = ").unwrap();
            let (start, end) = left.split_once(" to ").unwrap();
            Route {
                start: start.into(),
                end: end.into(),
                distance: right.parse().unwrap(),
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Path {
    visited: Vec<String>,
    current_location: String,
    distance: u32,
}

struct Output {
    min_dist: u32,
    max_dist: u32,
}

fn solve(input: &str) -> Output {
    let routes = parse_input(input);

    let mut route_map = HashMap::default();
    let mut locations = HashSet::default();

    for r in &routes {
        locations.insert(r.start.clone());
        locations.insert(r.end.clone());

        route_map
            .entry(r.start.clone())
            .or_insert_with(Vec::new)
            .push((r.end.clone(), r.distance));
        route_map
            .entry(r.end.clone())
            .or_insert_with(Vec::new)
            .push((r.start.clone(), r.distance));
    }

    let mut queue = locations
        .iter()
        .map(|l| Path {
            visited: vec![l.clone()],
            current_location: l.clone(),
            distance: 0,
        })
        .collect::<Vec<_>>();

    let mut min_dist = u32::MAX;
    let mut max_dist = 0;
    while let Some(path) = queue.pop() {
        // Visited all locations
        if path.visited.len() == locations.len() {
            min_dist = path.distance.min(min_dist);
            max_dist = path.distance.max(max_dist);
            continue;
        }

        let routes = route_map.get(&path.current_location).unwrap();
        for (dest, dist) in routes {
            if let Err(idx) = path.visited.binary_search(dest) {
                let mut path = path.clone();
                path.visited.insert(idx, dest.clone());
                path.distance += *dist;
                path.current_location = dest.clone();
                queue.push(path);
            }
        }
    }

    Output { min_dist, max_dist }
}

pub fn part1(input: &str) -> impl Display {
    solve(input).min_dist
}

pub fn part2(input: &str) -> impl Display {
    solve(input).max_dist
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 9;

    const EXAMPLE: &str = indoc! {"
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "605");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "207");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "982");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "804");
    }
}
