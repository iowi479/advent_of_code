use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::HashSet;

pub struct Day03 {}

impl Day03 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Challenge for Day03 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut houses = HashSet::new();
        let mut pos = (0, 0);
        houses.insert(pos);

        for c in input.chars() {
            match c {
                '^' => pos.1 += 1,
                'v' => pos.1 -= 1,
                '<' => pos.0 -= 1,
                '>' => pos.0 += 1,
                _ => (),
            }

            houses.insert(pos);
        }

        Ok(houses.len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut houses = HashSet::new();
        let mut pos = [(0, 0); 2];
        houses.insert(pos[0]);

        for (i, c) in input.chars().enumerate() {
            match c {
                '^' => pos[i % 2].1 += 1,
                'v' => pos[i % 2].1 -= 1,
                '<' => pos[i % 2].0 -= 1,
                '>' => pos[i % 2].0 += 1,
                _ => (),
            }

            houses.insert(pos[i % 2]);
        }

        Ok(houses.len().to_string())
    }
}
