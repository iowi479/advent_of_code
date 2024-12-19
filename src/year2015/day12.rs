use crate::challenge::Challenge;
use anyhow::{anyhow, Result};

pub struct Day12 {}

impl Day12 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Challenge for Day12 {
    fn part1(&self, input: &str) -> Result<String> {
        let result = input
            .replace(|c: char| !c.is_digit(10) && c != '-', " ")
            .split_whitespace()
            .filter(|t| !t.is_empty())
            .map(|n| n.parse::<i64>().map_err(|_| anyhow!("Invalid number")))
            .sum::<Result<i64>>()
            .map(|n| n.to_string())
            .unwrap();

        Ok(result)
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut stack = Vec::new();
        let mut removes = Vec::new();

        let mut cs = input.chars().enumerate();

        while let Some((i, c)) = cs.next() {
            match c {
                '{' | '[' => stack.push((c, i, false)),
                '}' | ']' => {
                    let last = stack.pop().ok_or_else(|| anyhow!("Unbalanced brackets"))?;
                    if (c == '}' && last.0 != '{') || (c == ']' && last.0 != '[') {
                        return Err(anyhow!("Unbalanced brackets"));
                    }

                    if last.2 {
                        removes.push((last.1, i));
                    }
                }
                'r' => {
                    if cs.next().ok_or_else(|| anyhow!("Unexpected end"))?.1 == 'e'
                        && cs.next().ok_or_else(|| anyhow!("Unexpected end"))?.1 == 'd'
                    {
                        let last = stack.last_mut().ok_or_else(|| anyhow!("Unexpected end"))?;
                        if last.0 == '{' {
                            last.2 = true;
                        }
                    }
                }
                _ => {}
            }
        }

        let mut input = input.to_string();

        let mut front = input.len();
        for remove in removes.iter().rev() {
            if remove.0 > front {
                continue;
            }
            front = remove.0;
            input.replace_range(remove.0..=remove.1, "");
        }

        let result = self.part1(&input)?;
        Ok(result)
    }
}
