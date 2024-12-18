use crate::challenge::Challenge;
use anyhow::{anyhow, Result};

pub struct Day04 {}

impl Day04 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Challenge for Day04 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut input = input.trim().to_string();
        let original_length = input.len();
        for i in 0.. {
            input += &i.to_string();
            let digest = md5::compute(&input);
            if digest.starts_with(&[0, 0]) && digest[2] < 0b1000 {
                return Ok(i.to_string());
            }
            let _ = input.split_off(original_length);
        }

        Err(anyhow!("No solution found"))
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut input = input.trim().to_string();
        let original_length = input.len();
        for i in 0.. {
            input += &i.to_string();
            let digest = md5::compute(&input);
            if digest.starts_with(&[0, 0, 0]) {
                return Ok(i.to_string());
            }
            let _ = input.split_off(original_length);
        }

        Err(anyhow!("No solution found"))
    }
}
