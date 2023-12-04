use super::intcode::IntcodeI;

fn abc(input: &str, noun: u32, verb: u32) -> Option<u32> {
    let mut intcode_i = IntcodeI::new(input);

    intcode_i.memory_mut()[1] = noun;
    intcode_i.memory_mut()[2] = verb;

    Some(intcode_i.interpret()?.memory()[0])
}

pub fn part1(input: &str) -> impl ToString {
    abc(input, 12, 2).unwrap()
}

pub fn part2(input: &str) -> impl ToString {
    let expected_output = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            let res = abc(input, noun, verb);
            if res == Some(expected_output) {
                return 100 * noun + verb;
            }
        }
    }

    panic!("Could not find input")
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2019;
    const DAY: u32 = 2;

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "5434663");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "4559");
    }
}
