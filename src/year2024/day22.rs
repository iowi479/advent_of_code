use crate::challenge::Challenge;
use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

pub struct Day22 {}

impl Day22 {
    pub fn new() -> Self {
        Self {}
    }
}

fn secret_number(mut num: u64) -> u64 {
    num ^= num << 6;
    num &= 16777216 - 1;
    num ^= num >> 5;
    num ^= num << 11;
    num & (16777216 - 1)
}

type Seq = (i8, i8, i8, i8);

impl Challenge for Day22 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut result = 0;

        for num in input.lines().map(|l| l.parse::<u64>().unwrap()) {
            let mut secret_num = num;
            for _ in 0..2000 {
                secret_num = secret_number(secret_num);
            }
            result += secret_num;
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut sequences = HashMap::new();
        let mut seen = HashSet::new();

        for num in input.lines().map(|l| l.parse::<u64>().unwrap()) {
            let mut secret_num = num;
            let mut last_seq: Seq = (0, 0, 0, 0);
            let mut last_num = (secret_num % 10) as i8;
            seen.clear();

            for i in 0..2000 {
                secret_num = secret_number(secret_num);
                let last_digit = (secret_num % 10) as i8;
                let diff = last_digit - last_num;
                last_seq = (last_seq.1, last_seq.2, last_seq.3, diff);
                last_num = last_digit;

                if i < 4 {
                    continue;
                }

                if !seen.contains(&last_seq) {
                    seen.insert(last_seq);
                    *sequences.entry(last_seq).or_insert(0) += last_digit as u64;
                }
            }
        }

        let max = sequences
            .values()
            .max()
            .ok_or(anyhow!("No max value found"))?;

        Ok(max.to_string())
    }
}
