use std::str::Lines;

use rayon::{iter::ParallelIterator, slice::ParallelSlice};

#[derive(Debug)]
struct Mapping {
    dest_range_start: usize,
    src_range_start: usize,
    range_len: usize,
}

#[derive(Debug)]
struct Maps {
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

fn parse_input(input: &str) -> (Vec<usize>, Maps) {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    lines.next();

    let seed_to_soil = parse_map(&mut lines);
    let soil_to_fertilizer = parse_map(&mut lines);
    let fertilizer_to_water = parse_map(&mut lines);
    let water_to_light = parse_map(&mut lines);
    let light_to_temperature = parse_map(&mut lines);
    let temperature_to_humidity = parse_map(&mut lines);
    let humidity_to_location = parse_map(&mut lines);
    (
        seeds,
        Maps {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    )
}

fn parse_map(lines: &mut Lines) -> Vec<Mapping> {
    let mut lines = lines.skip(1);

    let mut mappings = vec![];

    loop {
        let Some(line) = lines.next() else {
            break;
        };
        if line.is_empty() {
            break;
        }

        let mut split = line.split(' ');
        let dest_range_start = split.next().unwrap().parse().unwrap();
        let src_range_start = split.next().unwrap().parse().unwrap();
        let range_len = split.next().unwrap().parse().unwrap();

        mappings.push(Mapping {
            dest_range_start,
            src_range_start,
            range_len,
        });
    }

    mappings
}

fn map_number(map: &[Mapping], number: usize) -> usize {
    match map
        .iter()
        .find(|m| m.src_range_start <= number && number < m.src_range_start + m.range_len)
    {
        Some(mapping) => number - mapping.src_range_start + mapping.dest_range_start,
        None => number,
    }
}

fn map_seed(maps: &Maps, seed: usize) -> usize {
    let soil = map_number(&maps.seed_to_soil, seed);
    let fertilizer = map_number(&maps.soil_to_fertilizer, soil);
    let water = map_number(&maps.fertilizer_to_water, fertilizer);
    let light = map_number(&maps.water_to_light, water);
    let temperature = map_number(&maps.light_to_temperature, light);
    let humidity = map_number(&maps.temperature_to_humidity, temperature);

    map_number(&maps.humidity_to_location, humidity)
}

pub fn part1(input: &str) -> impl ToString {
    let (seeds, maps) = parse_input(input);
    seeds
        .iter()
        .map(|seed| map_seed(&maps, *seed))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> impl ToString {
    let (seeds, maps) = parse_input(input);
    seeds
        .par_chunks(2)
        .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .map(|seed| map_seed(&maps, seed))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2023;
    const DAY: u32 = 5;

    const EXAMPLE: &str = indoc::indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "35");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "309796150");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "46");
    }

    #[tokio::test]
    #[ignore = "slow bruteforce"]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "50716416");
    }
}
