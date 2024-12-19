use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub struct Day09 {}

impl Day09 {
    pub fn new() -> Self {
        Self {}
    }
}

fn parse_input(input: &str) -> HashMap<(&str, &str), usize> {
    let mut distances = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(" = ");
        let mut cities = parts.next().unwrap().split(" to ");
        let city1 = cities.next().unwrap();
        let city2 = cities.next().unwrap();
        let distance = parts.next().unwrap().parse().unwrap();
        distances.insert((city1, city2), distance);
        distances.insert((city2, city1), distance);
    }
    distances
}

fn permutations(cities: Vec<&str>) -> Vec<Vec<&str>> {
    if cities.len() == 2 {
        return vec![vec![cities[1], cities[0]], cities];
    }

    let mut result = Vec::new();
    for i in 0..cities.len() {
        let mut cities = cities.clone();
        let city = cities.remove(i);
        for mut p in permutations(cities) {
            p.push(city);
            result.push(p);
        }
    }
    result
}
impl Challenge for Day09 {
    fn part1(&self, input: &str) -> Result<String> {
        let distances: HashMap<(&str, &str), usize> = parse_input(input);

        let cities: HashSet<&str> = distances
            .keys()
            .map(|(city1, city2)| vec![*city1, *city2])
            .flatten()
            .collect();

        let permutations = permutations(cities.iter().cloned().collect());

        let mut min_distance = usize::MAX;
        'outer: for p in permutations {
            let mut distance = 0;
            for i in 0..p.len() - 1 {
                if p[i] == p[i + 1] {
                    continue 'outer;
                }
                distance += distances.get(&(p[i], p[i + 1])).unwrap();
            }
            min_distance = min_distance.min(distance);
        }

        Ok(min_distance.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let distances: HashMap<(&str, &str), usize> = parse_input(input);

        let cities: HashSet<&str> = distances
            .keys()
            .map(|(city1, city2)| vec![*city1, *city2])
            .flatten()
            .collect();

        let permutations = permutations(cities.iter().cloned().collect());

        let mut max_distance = usize::MIN;
        'outer: for p in permutations {
            let mut distance = 0;
            for i in 0..p.len() - 1 {
                if p[i] == p[i + 1] {
                    continue 'outer;
                }
                distance += distances.get(&(p[i], p[i + 1])).unwrap();
            }
            max_distance = max_distance.max(distance);
        }

        Ok(max_distance.to_string())
    }
}
