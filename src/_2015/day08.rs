use std::borrow::Cow;

use crate::prelude::*;

pub fn part1(input: &str) -> impl Display {
    let mut mem_size = 0;
    let mut code_size = 0;

    let re = Regex::new(r#"\\"|\\\\|\\x([0-9a-fA-F]{2})"#).unwrap();

    for line in input.lines() {
        code_size += line.len();

        let stripped_line = line.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
        let cow = re.replace_all(stripped_line, |c: &regex::Captures<'_>| {
            let s: Cow<str> = match &c[0] {
                r"\\" => r"\".into(),
                r#"\""# => r#"""#.into(),
                s if s.starts_with(r#"\x"#) => "a".into(),
                _ => unreachable!(),
            };
            s
        });

        mem_size += cow.len();
    }

    code_size - mem_size
}

pub fn part2(input: &str) -> impl Display {
    let mut code_size = 0;
    let mut encoded_size = 0;

    for line in input.lines() {
        code_size += line.len();

        let encoded = line.replace(['\"', '\\'], "xx");
        encoded_size += 2;
        encoded_size += encoded.len();
    }

    encoded_size - code_size
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 8;

    const EXAMPLE: &str = indoc! {r#"
        ""
        "abc"
        "aaa\"aaa"
        "\x27"
    "#};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "12");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "1342");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "19");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "2074");
    }
}
