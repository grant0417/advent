pub fn part1(input: &str) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let mut stack = vec![];

            line.chars()
                .map(|c| match c {
                    '(' | '[' | '{' | '<' => {
                        stack.push(c);
                        0
                    }
                    ')' | ']' | '}' | '>' => {
                        let left = stack.pop().unwrap();
                        match (left, c) {
                            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => 0,
                            (_, ')') => 3,
                            (_, ']') => 57,
                            (_, '}') => 1197,
                            (_, '>') => 25137,
                            (_, _) => panic!("Unexpected character"),
                        }
                    }
                    _ => panic!("Unexpected character"),
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

pub fn part2(input: &str) -> impl ToString {
    let lines: Vec<_> = input.lines().collect();

    let mut line_score: Vec<u64> = vec![];

    'line_loop: for line in lines {
        let mut stack = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let top = stack.pop().unwrap();

                    match (top, c) {
                        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => continue,
                        (_, ')') | (_, ']') | (_, '}') | (_, '>') => {
                            continue 'line_loop;
                        }
                        (_, _) => panic!("Unexpected character"),
                    }
                }
                _ => panic!("Unexpected character"),
            }
        }

        line_score.push(stack.iter().rev().fold(0, |mut acc, c| {
            acc *= 5;
            match c {
                '(' => acc += 1,
                '[' => acc += 2,
                '{' => acc += 3,
                '<' => acc += 4,
                _ => panic!("Unexpected character"),
            }
            acc
        }))
    }

    line_score.sort_unstable();

    line_score[line_score.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2021;
    const DAY: u32 = 10;

    const EXAMPLE: &str = indoc::indoc! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "26397");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "240123");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "288957");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "3260812321");
    }
}
