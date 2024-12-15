use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::HashMap;

pub struct Day05 {}

impl Day05 {
    pub fn new() -> Self {
        Self {}
    }
}

struct Input {
    rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}

fn parse_input(input: &str) -> Result<Input> {
    let (rule_str, update_str) = input.split_once("\n\n").unwrap();

    let mut rules = HashMap::new();

    rule_str
        .lines()
        .map(|line| line.trim().split_once("|").unwrap())
        .map(|(key, value)| {
            let key = key.parse::<u32>().unwrap();
            let value = value.parse::<u32>().unwrap();
            (key, value)
        })
        .for_each(|(key, value)| {
            rules.entry(key).or_insert_with(Vec::new).push(value);
        });

    let updates = update_str
        .lines()
        .map(|line| {
            line.trim()
                .split(",")
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    Ok(Input { rules, updates })
}

impl Challenge for Day05 {
    fn part1(&self, input: &str) -> Result<String> {
        let input = parse_input(input)?;
        let mut result = 0;

        'updates: for update in input.updates {
            for i in 1..update.len() {
                let current = update[i];
                let previous = &update[..i];

                if let Some(rules) = input.rules.get(&current) {
                    if rules.iter().any(|r| previous.contains(r)) {
                        continue 'updates;
                    }
                }
            }

            // valid
            let middle = update.len() / 2;
            result += update[middle];
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let input = parse_input(input)?;
        let mut result = 0;

        'updates: for update in input.updates {
            for i in 1..update.len() {
                let current = update[i];
                let previous = &update[..i];

                if let Some(rules) = input.rules.get(&current) {
                    if rules.iter().any(|r| previous.contains(r)) {
                        // is incorrect

                        let middle = correct_update(&update, &input.rules);
                        result += middle;
                        continue 'updates;
                    }
                }
            }
        }

        Ok(result.to_string())
    }
}

fn correct_update(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut corrected_update = update.clone();

    corrected_update.sort_by(|a, b| {
        let ra = rules.get(a);
        let rb = rules.get(b);

        if rb.is_some_and(|rb| rb.contains(a)) {
            return std::cmp::Ordering::Greater;
        } else if ra.is_some_and(|ra| ra.contains(b)) {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Equal;
        }
    });

    let middle = corrected_update.len() / 2;
    let result = corrected_update[middle];
    result
}
