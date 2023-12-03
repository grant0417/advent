#[derive(Debug, Clone)]
struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Clone)]
struct Cubes {
    id: u32,
    segments: Vec<Colors>,
}

impl Cubes {
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    fn parse_game(game: &str) -> Cubes {
        let mut line = game.split_whitespace();
        line.next().unwrap();
        let id = line.next().unwrap();
        let id = id.strip_suffix(':').unwrap().parse().unwrap();

        let mut segment = vec![Colors {
            red: 0,
            green: 0,
            blue: 0,
        }];

        let mut new_segment = false;
        while let Some(num) = line.next() {
            let num: u32 = num.parse().unwrap();
            let color = line.next().unwrap();

            if new_segment {
                segment.push(Colors {
                    red: 0,
                    green: 0,
                    blue: 0,
                });
                new_segment = false;
            }

            match color.strip_suffix([',', ';']).unwrap_or(color) {
                "red" => segment.last_mut().unwrap().red += num,
                "green" => segment.last_mut().unwrap().green += num,
                "blue" => segment.last_mut().unwrap().blue += num,
                _ => unreachable!(),
            }

            if color.ends_with(';') {
                new_segment = true;
            }
        }

        Cubes {
            id,
            segments: segment,
        }
    }

    fn possible(&self, reds: u32, greens: u32, blues: u32) -> bool {
        for segment in &self.segments {
            if segment.red > reds || segment.green > greens || segment.blue > blues {
                return false;
            }
        }
        true
    }

    // min number of cubes to satisfy all segments, multiplied together
    fn min_cubes_power(&self) -> u32 {
        let mut min = Colors {
            red: 0,
            green: 0,
            blue: 0,
        };

        for segment in &self.segments {
            min.red = min.red.max(segment.red);
            min.green = min.green.max(segment.green);
            min.blue = min.blue.max(segment.blue);
        }

        min.red * min.green * min.blue
    }
}

pub fn part1(input: &str) -> impl ToString {
    let reds = 12;
    let greens = 13;
    let blues = 14;

    input
        .lines()
        .map(Cubes::parse_game)
        .filter(|c| c.possible(reds, greens, blues))
        .map(|c| c.id)
        .sum::<u32>()
}

pub fn part2(input: &str) -> impl ToString {
    input
        .lines()
        .map(Cubes::parse_game)
        .map(|c| c.min_cubes_power())
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 2;

    const EXAMPLE: &str = indoc::indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "8");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "2283");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "2286");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "78669");
    }
}
