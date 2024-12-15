use crate::challenge::Challenge;
use anyhow::{anyhow, Result};

pub struct Day03 {}

impl Day03 {
    pub fn new() -> Self {
        Self {}
    }
}

// checks and parses an input string for a multiplication
// example: 123,4 -> 2024
fn parse_mul(input: &str) -> u64 {
    for c in input.chars() {
        if c.is_ascii_digit() {
            continue;
        }
        if c == ',' {
            continue;
        }
        return 0;
    }

    // no invalid characters found

    if let Some(mul) = input.split_once(',') {
        let a = mul.0.parse::<u64>().unwrap_or(1000);
        let b = mul.1.parse::<u64>().unwrap_or(1000);
        if a > 999 || b > 999 {
            return 0;
        }
        return a * b;
    }
    return 0;
}

impl Challenge for Day03 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut text = input;
        let mut result = 0;

        while let Some(idx) = text.find("mul(") {
            text = &text[idx + 4..];
            let end = text.find(')');
            if end.is_none() {
                // no more ")"
                break;
            }

            let end = end.unwrap();
            let inner_text = &text[..end];

            let mul = parse_mul(inner_text);
            result += mul;
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut text = input;
        let mut result = 0;
        while text.len() > 0 {
            if text.starts_with("mul(") {
                let end = text.find(')');
                if end.is_none() {
                    return Err(anyhow!("No closing )"));
                }
                let end = end.unwrap();
                let inner_text = &text[4..end];
                let mul = parse_mul(inner_text);
                result += mul;
            } else if text.starts_with("don't()") {
                let idx = text.find("do()");
                if idx.is_none() {
                    break;
                }

                text = &text[idx.unwrap() + 3..];
            }

            text = &text[1..];
        }

        Ok(result.to_string())
    }
}
