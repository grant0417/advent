pub fn part1(input: &str) -> impl ToString {
    let engine: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = engine.len();
    let width = engine[0].len();

    let check_part = |x: usize, y: usize| -> bool {
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let x = x as i32 + i;
                let y = y as i32 + j;
                if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                    continue;
                }
                let c = engine[y as usize][x as usize];
                if !c.is_ascii_digit() && c != '.' {
                    return true;
                }
            }
        }
        false
    };

    let mut sum = 0;

    for (y, row) in engine.iter().enumerate() {
        let mut number = None;
        let mut is_part = false;
        for (x, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                match number {
                    None => number = Some(c.to_digit(10).unwrap()),
                    Some(n) => number = Some(n * 10 + c.to_digit(10).unwrap()),
                }

                if check_part(x, y) {
                    is_part = true;
                }
            } else {
                if let Some(n) = number {
                    if is_part {
                        sum += n;
                    }
                }
                number = None;
                is_part = false;
            }
        }
        if let Some(n) = number {
            if is_part {
                sum += n;
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> impl ToString {
    let engine: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = engine.len();
    let width = engine[0].len();

    let mut gears: Vec<Vec<Option<(i32, u8)>>> = vec![vec![None; width]; height];

    let mut annotate_gear = |x_0: usize, x_1: usize, y: usize, val: u32| {
        // find all * that within 1 cell of the range
        for x in (x_0 as i32 - 1)..=(x_1 as i32 + 1) {
            if x < 0 || x >= width as i32 {
                continue;
            }

            let y_offsets: &[_] = if x == x_0 as i32 - 1 || x == x_1 as i32 + 1 {
                &[-1, 0, 1]
            } else {
                &[-1, 1]
            };

            for y_offset in y_offsets {
                let y = y as i32 + y_offset;
                if y < 0 || y >= height as i32 {
                    continue;
                }
                if engine[y as usize][x as usize] == '*' {
                    match &mut gears[y as usize][x as usize] {
                        Some((v, c)) => {
                            *v *= val as i32;
                            *c += 1;
                        }
                        None => {
                            gears[y as usize][x as usize] = Some((val as i32, 1));
                        }
                    }
                }
            }
        }
    };

    for (y, row) in engine.iter().enumerate() {
        let mut number = None;
        let mut x_start = 0;
        for (x, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                match number {
                    None => {
                        x_start = x;
                        number = Some(c.to_digit(10).unwrap());
                    }
                    Some(n) => number = Some(n * 10 + c.to_digit(10).unwrap()),
                }
            } else {
                if let Some(n) = number {
                    annotate_gear(x_start, x - 1, y, n);
                }
                number = None;
            }
        }
        if let Some(n) = number {
            annotate_gear(x_start, width - 1, y, n);
        }
    }

    gears
        .iter()
        .map(|row| {
            row.iter()
                .filter_map(|s| match *s {
                    Some((v, 2)) => Some(v),
                    _ => None,
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 3;

    const EXAMPLE: &str = indoc::indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "4361");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "514969");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "467835");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "78915902");
    }
}
