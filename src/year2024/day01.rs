use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::HashMap;

pub struct Day01 {}

impl Day01 {
    pub fn new() -> Self {
        Self {}
    }
}

type Lists = [Vec<i32>; 2];

fn parse_input(input: &str) -> Result<Lists> {
    Ok(input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut words| {
            [
                words.next().expect("no first word"),
                words.next().expect("no second word"),
            ]
        })
        .fold([vec![], vec![]], |mut acc, words| {
            acc[0].push(words[0].parse::<i32>().expect("first word not a number"));
            acc[1].push(words[1].parse::<i32>().expect("second word not a number"));
            acc
        }))
}

impl Challenge for Day01 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut lists = parse_input(input)?;
        lists.iter_mut().for_each(|list| list.sort());

        let mut result = 0;
        for i in 0..lists[0].len() {
            let mut distance = lists[0][i] - lists[1][i];
            distance = distance.abs();
            result += distance;
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let lists = parse_input(input)?;

        let mut amounts: HashMap<i32, i32> = HashMap::new();

        for &num in lists[1].iter() {
            let count = amounts.entry(num).or_insert(0);
            *count += 1;
        }

        let mut result = 0;
        for i in 0..lists[0].len() {
            let left = lists[0][i];
            let factor = amounts.get(&left).unwrap_or(&0);

            result += left * factor;
        }

        Ok(result.to_string())
    }
}
