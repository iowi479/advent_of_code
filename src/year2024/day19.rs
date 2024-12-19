use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::HashMap;

pub struct Day19 {}

impl Day19 {
    pub fn new() -> Self {
        Self {}
    }
}

struct Towels<'a> {
    available: Vec<&'a str>,
    wanted: Vec<&'a str>,
}

fn parse_input(input: &str) -> Towels {
    let mut parts = input.split("\n\n");
    let mut available = parts.next().unwrap().split(", ").collect::<Vec<&str>>();
    let mut wanted = Vec::new();

    let mut lines = parts.next().unwrap().lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        wanted.push(line);
    }

    available.sort();

    Towels { available, wanted }
}

fn possible<'a>(
    available: &Vec<&'a str>,
    wanted: &'a str,
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(r) = cache.get(wanted) {
        return *r;
    }

    let mut result = false;
    for towel in available {
        if towel.len() > wanted.len() {
            continue;
        }

        if wanted.starts_with(towel) {
            if wanted.len() == towel.len() {
                return true;
            }
            if possible(available, &wanted[towel.len()..], cache) {
                result = true;
                break;
            }
        }
    }

    cache.insert(wanted, result);
    result
}

fn mutations<'a>(
    available: &Vec<&'a str>,
    wanted: &'a str,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(r) = cache.get(wanted) {
        return *r;
    }

    let mut result = 0;
    for towel in available {
        if towel.len() > wanted.len() {
            continue;
        }

        if wanted.starts_with(towel) {
            if wanted.len() == towel.len() {
                result += 1;
            } else {
                result += mutations(available, &wanted[towel.len()..], cache);
            }
        }
    }

    cache.insert(wanted, result);
    result
}

impl Challenge for Day19 {
    fn part1(&self, _input: &str) -> Result<String> {
        let towels = parse_input(_input);
        let mut cache = HashMap::new();
        let mut result = 0;

        for towel in towels.wanted {
            if possible(&towels.available, towel, &mut cache) {
                result += 1;
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, _input: &str) -> Result<String> {
        let towels = parse_input(_input);
        let mut cache = HashMap::new();
        let mut result = 0;

        for towel in towels.wanted {
            let i = mutations(&towels.available, towel, &mut cache);
            println!("{}: {}", towel, i);
            result += i;
        }

        Ok(result.to_string())
    }
}
