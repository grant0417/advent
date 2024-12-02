use crate::prelude::*;

fn parse_line(input: &str) -> (&str, &str) {
    let mut lines = input.lines().map(|l| l.trim());
    (lines.next().unwrap(), lines.next().unwrap())
}

fn trace_line(line: &str, mut on_point: impl FnMut((i32, i32, i32))) {
    let mut x = 0;
    let mut y = 0;
    let mut step = 0;
    for inst in line.split(',') {
        let dir = &inst[0..1];
        let num = inst[1..].parse::<i32>().unwrap();

        match dir {
            "R" => (0..num).for_each(|_| {
                (step, x) = (step + 1, x + 1);
                on_point((x, y, step))
            }),
            "L" => (0..num).for_each(|_| {
                (step, x) = (step + 1, x - 1);
                on_point((x, y, step))
            }),
            "U" => (0..num).for_each(|_| {
                (step, y) = (step + 1, y + 1);
                on_point((x, y, step))
            }),
            "D" => (0..num).for_each(|_| {
                (step, y) = (step + 1, y - 1);
                on_point((x, y, step))
            }),
            other => panic!("Unknown dir: {other}"),
        };
    }
}

pub fn part1(input: &str) -> impl ToString {
    let (line0, line1) = parse_line(input);
    let mut locations = HashSet::default();
    let mut min_intersection = i32::MAX;

    trace_line(line0, |(x, y, _)| {
        locations.insert((x, y));
    });

    trace_line(line1, |(x, y, _)| {
        if locations.contains(&(x, y)) {
            let dist = x.abs() + y.abs();
            if dist != 0 {
                min_intersection = min_intersection.min(dist);
            }
        }
    });

    min_intersection
}

pub fn part2(input: &str) -> impl ToString {
    let (line0, line1) = parse_line(input);
    let mut locations = HashMap::default();

    trace_line(line0, |(x, y, steps)| {
        locations.insert((x, y), (1u8, steps));
    });

    trace_line(line1, |(x, y, steps)| {
        let a = locations.entry((x, y)).or_insert((0, 0));
        a.0 |= 2u8;
        a.1 += steps;
    });

    locations
        .values()
        .filter_map(|(a, b)| if *a == 3 { Some(*b) } else { None })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2019;
    const DAY: u32 = 3;

    const EXAMPLE: &str = indoc::indoc! {"
        R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "159");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "316");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "610");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "16368");
    }
}
