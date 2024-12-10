use std::io::Write;

use md5::Digest;

use crate::prelude::*;

fn search(input: &str, part2: bool) -> usize {
    let buf = String::with_capacity(input.len() + 10);

    (0..100_000_000)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|i| {
            let mut buf = buf.clone();
            write!(buf, "{input}{i}").unwrap();

            let mut md5 = md5::Md5::new();
            md5.write(buf.as_bytes()).unwrap();
            let b = md5.finalize();

            b[0] == 0 && b[1] == 0 && ((!part2 && b[2] & 0xf0 == 0) || (part2 && b[2] == 0))
        })
        .unwrap()
}

pub fn part1(input: &str) -> impl Display {
    search(input.trim(), false)
}

pub fn part2(input: &str) -> impl Display {
    search(input.trim(), true)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 4;

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string().parse::<u32>().unwrap(), 346_386);
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string().parse::<u32>().unwrap(), 9_958_218);
    }
}
