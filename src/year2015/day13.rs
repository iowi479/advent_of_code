use crate::challenge::Challenge;
use anyhow::Result;
use std::{collections::HashMap, fmt::Debug};

pub struct Day13 {}

impl Day13 {
    pub fn new() -> Self {
        Self {}
    }
}

type Scores = HashMap<(usize, usize), i64>;

fn parse_input(input: &str) -> (Vec<&str>, Scores) {
    let mut scores = Scores::new();
    let mut names = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let person1 = parts[0];
        if !names.contains(&person1) {
            names.push(person1);
        }

        let person2 = parts[10].trim_end_matches('.');
        if !names.contains(&person2) {
            names.push(person2);
        }

        let mut score = parts[3].parse::<i64>().unwrap();
        score = if parts[2] == "lose" { -score } else { score };
        let p1 = names.iter().position(|p| *p == person1).unwrap();
        let p2 = names.iter().position(|p| *p == person2).unwrap();

        *scores.entry((p1, p2)).or_insert(0) += score;
        *scores.entry((p2, p1)).or_insert(0) += score;
    }

    (names, scores)
}

fn permutations<T: Clone + Debug>(items: Vec<T>) -> Vec<Vec<T>> {
    if items.len() == 1 {
        return vec![items];
    }

    let mut result = Vec::new();

    let mut rest = items.clone();
    let item = rest.pop().unwrap();

    for perm in permutations(rest) {
        for i in 1..perm.len() {
            let mut new_perm = perm.clone();
            new_perm.insert(i, item.clone());
            result.push(new_perm);
        }
        let mut new_perm = perm.clone();
        new_perm.push(item.clone());
        result.push(new_perm);
    }
    result
}

impl Challenge for Day13 {
    fn part1(&self, input: &str) -> Result<String> {
        let (names, scores) = parse_input(input);
        let mut result = 0;

        let idxs = (0..names.len()).collect::<Vec<_>>();
        for perm in permutations(idxs) {
            let mut score = 0;
            for i in 0..perm.len() {
                let p1 = perm[i];
                let p2 = perm[(i + 1) % perm.len()];
                score += scores.get(&(p1, p2)).unwrap();
            }
            result = result.max(score);
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (mut names, mut scores) = parse_input(input);
        for i in 0..names.len() {
            scores.insert((i, names.len()), 0);
            scores.insert((names.len(), i), 0);
        }

        names.push("Me");

        let mut result = 0;

        let idxs = (0..names.len()).collect::<Vec<_>>();
        for perm in permutations(idxs) {
            let mut score = 0;
            for i in 0..perm.len() {
                let p1 = perm[i];
                let p2 = perm[(i + 1) % perm.len()];
                score += scores.get(&(p1, p2)).unwrap();
            }
            result = result.max(score);
        }

        Ok(result.to_string())
    }
}
