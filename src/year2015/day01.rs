use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day01 {}

impl Day01 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Challenge for Day01 {
    fn part1(&self, input: &str) -> Result<String> {
        Ok(input
            .chars()
            .fold(0, |acc, c| match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => acc,
            })
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut floor = 0;

        for (i, c) in input.chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => (),
            }
            if floor == -1 {
                return Ok((i + 1).to_string());
            }
        }
        Ok("".to_string())
    }
}
