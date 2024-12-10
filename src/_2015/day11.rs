use crate::prelude::*;

fn three_increasing_chars(s: &[char]) -> bool {
    s.iter()
        .copied()
        .tuple_windows()
        .any(|(a, b, c)| b as i32 - a as i32 == 1 && c as i32 - b as i32 == 1)
}

fn no_confusing_chars(s: &[char]) -> bool {
    !s.iter()
        .copied()
        .any(|c| c == 'i' || c == 'o' || c == 'l')
}

fn multiple_pairs(s: &[char]) -> bool {
    let mut pair = None;
    for i in 0..s.len() - 1 {
        if s[i] == s[i + 1] {
            match pair {
                Some(c) => {
                    if c != s[i] {
                        return true;
                    }
                }
                None => pair = Some(s[i]),
            }
        }
    }
    false
}

fn next_password(password: &str) -> String {
    let mut password = password.chars().collect::<Vec<_>>();
    loop {
        for i in (1..password.len()).rev() {
            if password[i] == 'z' {
                password[i] = 'a'
            } else {
                password[i] = (password[i] as u8 + 1) as char;
                break;
            }
        }

        if three_increasing_chars(&password)
            && no_confusing_chars(&password)
            && multiple_pairs(&password)
        {
            return password.into_iter().collect::<String>();
        };
    }
}

pub fn part1(input: &str) -> impl Display {
    next_password(input.trim())
}

pub fn part2(input: &str) -> impl Display {
    next_password(&next_password(input.trim()))
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 11;

    #[test]
    fn part1_example() {
        assert_eq!(part1("abcdefgh").to_string(), "abcdffaa");
        assert_eq!(part1("ghijklmn").to_string(), "ghjaabcc");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "hepxxyzz");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "heqaabcc");
    }
}
