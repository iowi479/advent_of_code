use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::HashMap;

pub struct Day11 {}

impl Day11 {
    pub fn new() -> Self {
        Self {}
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn rec(stone: u64, steps: u64, known: &mut HashMap<(u64, u64), u64>) -> u64 {
    if steps == 0 {
        return 1;
    }

    if let Some(&res) = known.get(&(stone, steps)) {
        return res;
    }

    let res = match stone {
        0 => rec(1, steps - 1, known),

        x if x.to_string().len() % 2 == 0 => {
            let s = x.to_string();
            let parts = s.split_at(s.len() / 2);

            rec(parts.0.parse::<u64>().unwrap(), steps - 1, known)
                + rec(parts.1.parse::<u64>().unwrap(), steps - 1, known)
        }
        _ => rec(stone * 2024, steps - 1, known),
    };

    known.insert((stone, steps), res);

    return res;
}

impl Challenge for Day11 {
    fn part1(&self, input: &str) -> Result<String> {
        let stones = parse_input(input);
        let mut result = 0;
        let mut known = HashMap::new();

        for stone in stones {
            result += rec(stone, 25, &mut known);
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let stones = parse_input(input);
        let mut result = 0;
        let mut known = HashMap::new();

        for stone in stones {
            result += rec(stone, 75, &mut known);
        }

        Ok(result.to_string())
    }
}
