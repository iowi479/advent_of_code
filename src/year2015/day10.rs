use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day10 {}

impl Day10 {
    pub fn new() -> Self {
        Self {}
    }
}

fn parse(input: &str) -> String {
    let mut output = String::new();
    let mut current = input.chars().next().unwrap();
    let mut counter = 1;

    let cs = input.chars().skip(1);

    for c in cs {
        if c == current {
            counter += 1;
        } else {
            output.push_str(&counter.to_string());
            output.push(current);
            current = c;
            counter = 1;
        }
    }

    output.push_str(&counter.to_string());
    output.push_str(&current.to_string());
    output
}

impl Challenge for Day10 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut input = input.trim().to_string();
        for _ in 0..40 {
            input = parse(&input);
        }

        Ok(input.len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut input = input.trim().to_string();
        for _ in 0..50 {
            input = parse(&input);
        }

        Ok(input.len().to_string())
    }
}
