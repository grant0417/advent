use serde_json::Value;

use crate::prelude::*;

fn sum_value(value: &Value, ignore_red: bool) -> i64 {
    match value {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(vec) => vec.iter().map(|value| sum_value(value, ignore_red)).sum(),
        Value::Object(map) => {
            if ignore_red && map.values().filter_map(|v| v.as_str()).any(|s| s == "red") {
                0
            } else {
                map.values().map(|value| sum_value(value, ignore_red)).sum()
            }
        }
    }
}

pub fn part1(input: &str) -> impl Display {
    let value = serde_json::from_str::<serde_json::Value>(input).unwrap();
    sum_value(&value, false)
}

pub fn part2(input: &str) -> impl Display {
    let value = serde_json::from_str::<serde_json::Value>(input).unwrap();
    sum_value(&value, true)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 12;

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "119433");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "68466");
    }
}
