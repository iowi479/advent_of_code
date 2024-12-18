use std::collections::HashMap;

use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day05 {}

impl Day05 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Challenge for Day05 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut result = 0;
        'outer: for word in input.lines() {
            let mut vowels = 0;
            let mut double = false;

            for i in 0..word.len() {
                let text = &word[i..];

                if text.len() < 1 {
                    continue;
                }

                match text.chars().next().unwrap() {
                    'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                    _ => {}
                }

                if text.len() < 2 {
                    continue;
                }

                let mut chars = text.chars();

                let c1 = chars.next().unwrap();
                let c2 = chars.next().unwrap();
                if c1 == c2 {
                    double = true;
                }

                let cs = c1.to_string() + &c2.to_string();
                match cs.as_str() {
                    "ab" | "cd" | "pq" | "xy" => continue 'outer,
                    _ => {}
                }
            }

            if double && vowels > 2 {
                result += 1;
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut result = 0;
        for word in input.lines() {
            let mut pairs: HashMap<String, Vec<usize>> = HashMap::new();
            let mut double = false;

            for i in 0..word.len() {
                let text = &word[i..];

                if text.len() < 2 {
                    continue;
                }

                let pair = text.chars().take(2).collect::<String>();
                pairs.entry(pair).or_insert(Vec::new()).push(i);

                if text.len() < 3 {
                    continue;
                }
                let mut chars = text.chars();

                let c1 = chars.next().unwrap();
                let c3 = chars.skip(1).next().unwrap();
                if c1 == c3 {
                    double = true;
                }
            }

            let mut has_pairs = None;
            for (t, positions) in pairs.iter().filter(|(_, ps)| ps.len() >= 2) {
                for i in 0..positions.len() {
                    for j in i + 1..positions.len() {
                        if positions[j] - positions[i] > 1 {
                            has_pairs = Some(t);
                            break;
                        }
                    }
                }
            }

            if double && has_pairs.is_some() {
                println!("{}", word);
                result += 1;
            }
        }

        Ok(result.to_string())
    }
}
