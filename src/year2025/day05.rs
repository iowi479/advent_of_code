use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day05 {}

impl Day05 {
    pub fn new() -> Self {
        Self {}
    }
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut fresh_ranges = Vec::new();
    let mut ingredients = Vec::new();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        let (from, to) = trimmed.split_once("-").unwrap();
        let from = from.trim().parse::<usize>().unwrap();
        let to = to.trim().parse::<usize>().unwrap();

        fresh_ranges.push((from, to));
    }

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let ingredient = trimmed.parse::<usize>().unwrap();
        ingredients.push(ingredient);
    }

    (fresh_ranges, ingredients)
}

impl Challenge for Day05 {
    fn part1(&self, input: &str) -> Result<String> {
        let (fresh_ranges, ingredients) = parse_input(input);

        let mut result = 0;

        for ingredient in ingredients {
            for (from, to) in &fresh_ranges {
                if ingredient >= *from && ingredient <= *to {
                    result += 1;
                    break;
                }
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (fresh_ranges, _) = parse_input(input);

        let mut ranges = Vec::new();

        for (from, to) in fresh_ranges {
            let mut inserted = false;
            for i in 0..ranges.len() {
                let (rfrom, rto) = ranges[i];
                if from <= rfrom {
                    inserted = true;
                    ranges.insert(i, (from, to));
                    break;
                }
            }

            if !inserted {
                ranges.push((from, to));
            }

            clean_ranges(&mut ranges);
        }

        let mut result = 0;
        for (from, to) in ranges {
            result += to - from + 1;
        }

        Ok(result.to_string())
    }
}

fn clean_ranges(ranges: &mut Vec<(usize, usize)>) {
    let mut i = 0;
    while i < ranges.len() - 1 {
        let current = ranges[i];
        let next = ranges[i + 1];

        if current.1 >= next.0 - 1 {
            ranges[i] = (current.0, current.1.max(next.1));
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
}
