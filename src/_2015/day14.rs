use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
enum State {
    Running(u32),
    Resting(u32),
}

#[derive(Debug)]
struct ReindeerStats {
    _name: String,
    speed: u32,
    run_duration: u32,
    rest_duration: u32,

    position: u32,
    state: State,
    points: u32,
}

fn parse_input(input: &str) -> impl Iterator<Item = ReindeerStats> + '_ {
    let re = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();
    input.lines().map(move |line| {
        let c = re.captures(line).unwrap();
        ReindeerStats {
            _name: c[1].to_owned(),
            speed: c[2].parse().unwrap(),
            run_duration: c[3].parse().unwrap(),
            rest_duration: c[4].parse().unwrap(),

            position: 0,
            state: State::Running(c[3].parse().unwrap()),
            points: 0,
        }
    })
}

fn run(reindeer: &mut Vec<ReindeerStats>, seconds: usize) {
    for _ in 0..seconds {
        for r in &mut *reindeer {
            if matches!(r.state, State::Running(_)) {
                r.position += r.speed;
            }
            match r.state {
                State::Running(duration) => {
                    if duration == 1 {
                        r.state = State::Resting(r.rest_duration);
                    } else {
                        r.state = State::Running(duration - 1)
                    }
                }
                State::Resting(duration) => {
                    if duration == 1 {
                        r.state = State::Running(r.run_duration)
                    } else {
                        r.state = State::Resting(duration - 1)
                    }
                }
            }
        }
        reindeer.sort_by_key(|r| r.position);
        let max_pos = reindeer.iter().map(|r| r.position).max().unwrap();
        for r in &mut *reindeer {
            if r.position == max_pos {
                r.points += 1;
            }
        }
    }
    println!("{reindeer:?}");
}

pub fn part1(input: &str) -> impl Display {
    let mut reindeer = parse_input(input).collect::<Vec<_>>();
    run(&mut reindeer, 2503);
    reindeer.iter().map(|r| r.position).max().unwrap()
}

pub fn part2(input: &str) -> impl Display {
    let mut reindeer = parse_input(input).collect::<Vec<_>>();
    run(&mut reindeer, 2503);
    reindeer.iter().map(|r| r.points).max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 14;

    const EXAMPLE: &str = indoc! {"
        Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
    "};

    #[test]
    fn example() {
        let mut reindeer = parse_input(EXAMPLE).collect::<Vec<_>>();
        run(&mut reindeer, 1000);
        let dancer = reindeer.iter().find(|r| r._name == "Dancer").unwrap();
        let comet = reindeer.iter().find(|r| r._name == "Comet").unwrap();
        assert_eq!(dancer.position, 1056);
        assert_eq!(comet.position, 1120);
        assert_eq!(dancer.points, 689);
        assert_eq!(comet.points, 312);
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "2655");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1059");
    }
}
