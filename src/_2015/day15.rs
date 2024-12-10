use crate::prelude::*;

#[derive(Debug)]
struct Ingredient {
    _name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn parse_input(input: &str) -> impl Iterator<Item = Ingredient> + '_ {
    let re = Regex::new(
        r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)",
    ).unwrap();
    input.lines().map(move |line| {
        let c = re.captures(line).unwrap();
        Ingredient {
            _name: c[1].to_owned(),
            capacity: c[2].parse().unwrap(),
            durability: c[3].parse().unwrap(),
            flavor: c[4].parse().unwrap(),
            texture: c[5].parse().unwrap(),
            calories: c[6].parse().unwrap(),
        }
    })
}

fn nested_loops(
    depth: usize,
    remaining: i64,
    current: &mut Vec<i64>,
    ingredients: &Vec<Ingredient>,
    calorie_requirement: bool,
) -> i64 {
    if depth == 0 {
        if remaining == 0 {
            let mut calories = 0;
            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;
            for (ingredient, qty) in ingredients.iter().zip(current.iter()) {
                calories += ingredient.calories * qty;
                capacity += ingredient.capacity * qty;
                durability += ingredient.durability * qty;
                flavor += ingredient.flavor * qty;
                texture += ingredient.texture * qty;
            }

            if !calorie_requirement || calories == 500 {
                return capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0);
            }
        }
        return i64::MIN;
    }

    (0..=remaining)
        .map(|i| {
            current.push(i);
            let v = nested_loops(
                depth - 1,
                remaining - i,
                current,
                ingredients,
                calorie_requirement,
            );
            current.pop();
            v
        })
        .max()
        .unwrap_or(i64::MIN)
}

pub fn part1(input: &str) -> impl Display {
    let ingredients = parse_input(input).collect::<Vec<_>>();
    nested_loops(4, 100, &mut Vec::new(), &ingredients, false)
}

pub fn part2(input: &str) -> impl Display {
    let ingredients = parse_input(input).collect::<Vec<_>>();
    nested_loops(4, 100, &mut Vec::new(), &ingredients, true)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 15;

    const EXAMPLE: &str = indoc! {"
        Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE).to_string(), "62842880");
    }

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "21367368");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE).to_string(), "57600000");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "1766400");
    }
}
