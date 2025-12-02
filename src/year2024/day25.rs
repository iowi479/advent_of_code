use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day25 {}

impl Day25 {
    pub fn new() -> Self {
        Self {}
    }
}

type Mask = [usize; 5];

fn parse_input(input: &str) -> (Vec<Mask>, Vec<Mask>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for blocks in input.split("\n\n") {
        let lines: Vec<&str> = blocks.lines().collect();
        let mut mask = [0; 5];

        if lines[0].starts_with(".....") {
            for i in 0..6 {
                let line = lines[6 - i];

                for x in 0..5 {
                    if line.chars().nth(x).unwrap() == '#' {
                        mask[x] = i;
                    }
                }
            }
            keys.push(mask);
        } else {
            for i in 0..6 {
                let line = lines[i];

                for x in 0..5 {
                    if line.chars().nth(x).unwrap() == '#' {
                        mask[x] = i;
                    }
                }
            }
            locks.push(mask);
        }
    }

    (keys, locks)
}

impl Challenge for Day25 {
    fn part1(&self, _input: &str) -> Result<String> {
        let (keys, locks) = parse_input(_input);
        let mut result = 0;
        for lock in &locks {
            'key: for key in &keys {
                for x in 0..5 {
                    if 5 - lock[x] < key[x] {
                        continue 'key;
                    }
                }
                result += 1;
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, _input: &str) -> Result<String> {
        Ok("Everything is done".to_string())
    }
}
