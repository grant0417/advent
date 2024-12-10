use std::ops::{Shl, Shr};

use crate::prelude::*;

type Map = HashMap<String, (Gate, Option<u16>)>;

fn eval_gate(gates: &mut Map, key: &str) -> u16 {
    let (gate, cached) = gates.get(key).cloned().unwrap();
    match cached {
        Some(cached) => cached,
        None => {
            let value = gate.eval(gates);
            gates.insert(key.into(), (gate, Some(value)));
            value
        }
    }
}

#[derive(Debug, Clone)]
enum SignalOrGate {
    Signal(u16),
    Gate(String),
}

impl SignalOrGate {
    fn parse(s: &str) -> Self {
        if let Ok(value) = s.parse() {
            SignalOrGate::Signal(value)
        } else {
            SignalOrGate::Gate(s.into())
        }
    }

    fn eval(&self, gates: &mut Map) -> u16 {
        match self {
            SignalOrGate::Signal(s) => *s,
            SignalOrGate::Gate(g) => eval_gate(gates, g),
        }
    }
}

#[derive(Debug, Clone)]
enum Gate {
    Value { v: SignalOrGate },
    And { a: SignalOrGate, b: SignalOrGate },
    Or { a: SignalOrGate, b: SignalOrGate },
    Lshift { a: String, b: u16 },
    Rshift { a: String, b: u16 },
    Not { a: String },
}

impl Gate {
    fn eval(&self, gates: &mut Map) -> u16 {
        match self {
            Gate::Value { v } => v.eval(gates),
            Gate::And { a, b } => a.eval(gates) & b.eval(gates),
            Gate::Or { a, b } => a.eval(gates) | b.eval(gates),
            Gate::Lshift { a, b } => eval_gate(gates, a).shl(*b),
            Gate::Rshift { a, b } => eval_gate(gates, a).shr(*b),
            Gate::Not { a } => !eval_gate(gates, a),
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (String, Gate)> + '_ {
    input.lines().map(|line| {
        let (left, right) = line.split_once(" -> ").unwrap();

        let gate = if let Some((a, b)) = left.split_once(" AND ") {
            Gate::And {
                a: SignalOrGate::parse(a),
                b: SignalOrGate::parse(b),
            }
        } else if let Some((a, b)) = left.split_once(" OR ") {
            Gate::Or {
                a: SignalOrGate::parse(a),
                b: SignalOrGate::parse(b),
            }
        } else if let Some((a, b)) = left.split_once(" LSHIFT ") {
            Gate::Lshift {
                a: a.into(),
                b: b.parse().unwrap(),
            }
        } else if let Some((a, b)) = left.split_once(" RSHIFT ") {
            Gate::Rshift {
                a: a.into(),
                b: b.parse().unwrap(),
            }
        } else if let Some(a) = left.strip_prefix("NOT ") {
            Gate::Not { a: a.into() }
        } else {
            Gate::Value {
                v: SignalOrGate::parse(left),
            }
        };

        (right.into(), gate)
    })
}

pub fn part1(input: &str) -> impl Display {
    let mut gates: Map = Map::from_iter(parse_input(input).map(|(k, v)| (k, (v, None))));
    eval_gate(&mut gates, "a")
}

pub fn part2(input: &str) -> impl Display {
    let mut gates: Map = Map::from_iter(parse_input(input).map(|(k, v)| (k, (v, None))));
    let mut gate_clone = gates.clone();
    let a = eval_gate(&mut gates, "a");
    gate_clone.insert(
        "b".into(),
        (
            Gate::Value {
                v: SignalOrGate::Signal(a),
            },
            None,
        ),
    );
    eval_gate(&mut gate_clone, "a")
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    const YEAR: u32 = 2015;
    const DAY: u32 = 7;

    #[tokio::test]
    async fn part1_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part1(&input).to_string(), "16076");
    }

    #[tokio::test]
    async fn part2_solve() {
        let input = util::input(YEAR, DAY).await;
        assert_eq!(part2(&input).to_string(), "2797");
    }
}
