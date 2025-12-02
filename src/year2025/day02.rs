use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day02 {}

impl Day02 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Challenge for Day02 {
    fn part1(&self, input: &str) -> Result<String> {
        let parts = input.split(',');
        let mut result: u64 = 0;

        for part in parts {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            let (from, to) = trimmed.split_once("-").unwrap();

            let from = from.trim().parse::<u64>().unwrap();
            let to = to.trim().parse::<u64>().unwrap();

            for id in from..=to {
                let id_str = format!("{}", id);

                let digits = id_str.chars().collect::<Vec<char>>();

                if digits.len() % 2 != 0 {
                    continue;
                }

                let half = digits.len() / 2;
                if digits[..half] == digits[half..] {
                    result += id;
                }
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let parts = input.split(',');
        let mut result: u64 = 0;

        for part in parts {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                continue;
            }
            let (from, to) = trimmed.split_once("-").unwrap();

            let from = from.trim().parse::<u64>().unwrap();
            let to = to.trim().parse::<u64>().unwrap();

            'ids: for id in from..=to {
                let id_str = format!("{}", id);

                let digits = id_str.chars().collect::<Vec<char>>();

                'chunksize: for chunk in 1..=(digits.len() / 2) {
                    if digits.len() % chunk != 0 {
                        continue;
                    }

                    let first_chunk = &digits[0..chunk];
                    let mut at = chunk;
                    while at < digits.len() {
                        let next_chunk = &digits[at..at + chunk];
                        if first_chunk != next_chunk {
                            continue 'chunksize;
                        }
                        at += chunk;
                    }

                    result += id;
                    continue 'ids;
                }
            }
        }

        Ok(result.to_string())
    }
}
