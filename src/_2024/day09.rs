use crate::prelude::*;

const DISK_EMPTY: u16 = u16::MAX;

fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.trim().chars().map(|i| i.to_digit(10).unwrap())
}

fn expand_map(map: impl Iterator<Item = u32>) -> Vec<u16> {
    let mut disk = Vec::with_capacity(100_000);
    let mut id = 0;
    for (i, b) in map.enumerate() {
        if i % 2 == 0 {
            for _ in 0..b {
                disk.push(id);
            }
            id += 1;
        } else {
            for _ in 0..b {
                disk.push(DISK_EMPTY);
            }
        }
    }
    disk
}

fn defrag_1(disk: &mut [u16]) {
    let mut last_empty = disk.len();
    for i in 0..disk.len() {
        if disk[i] == DISK_EMPTY {
            for j in (i..last_empty).rev() {
                if disk[j] != DISK_EMPTY {
                    (disk[i], disk[j]) = (disk[j], disk[i]);
                    last_empty = j;
                    break;
                }
            }
        }
    }
}

fn defrag_2(disk: &mut [u16]) {
    for i in (0..disk.len()).rev() {
        if disk[i] == DISK_EMPTY || (i < disk.len() - 1 && disk[i] == disk[i + 1]) {
            continue;
        }

        let (mut start, end) = (i, i + 1);
        while start > 0 && disk[start - 1] == disk[i] {
            start -= 1;
        }

        let (mut gap_start, mut gap_end) = (0, 0);
        while gap_end <= start {
            if disk[gap_end] == DISK_EMPTY {
                gap_end += 1;
                continue;
            }

            if gap_end - gap_start >= end - start {
                disk.copy_within(start..end, gap_start);
                disk[start..end].fill(DISK_EMPTY);
                break;
            }

            gap_end += 1;
            gap_start = gap_end;
        }
    }
}

fn checksum(disk: &[u16]) -> usize {
    disk.iter()
        .enumerate()
        .map(|(i, &j)| if j == DISK_EMPTY { 0 } else { i * j as usize })
        .sum::<usize>()
}

pub fn part1(input: &str) -> impl Display {
    let map = parse_input(input);
    let mut disk = expand_map(map);
    defrag_1(&mut disk);
    checksum(&disk)
}

pub fn part2(input: &str) -> impl Display {
    let map = parse_input(input);
    let mut disk = expand_map(map);
    defrag_2(&mut disk);
    checksum(&disk)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2024;
    const DAY: u32 = 9;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "1928");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "6432869891895");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "2858");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "6467290479134");
    }
}
