use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub struct Day23 {}

impl Day23 {
    pub fn new() -> Self {
        Self {}
    }
}

type Graph = HashMap<String, Vec<String>>;

fn parse_graph(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();

        graph
            .entry(left.to_string())
            .or_insert(Vec::new())
            .push(right.to_string());
        graph
            .entry(right.to_string())
            .or_insert(Vec::new())
            .push(left.to_string());
    }

    graph
}

impl Challenge for Day23 {
    fn part1(&self, input: &str) -> Result<String> {
        let graph = parse_graph(input);
        let mut sets: HashSet<[String; 3]> = HashSet::new();

        for (node, neighbors) in graph.iter() {
            if neighbors.len() < 2 {
                continue;
            }

            for i in 0..neighbors.len() {
                for j in i + 1..neighbors.len() {
                    let mut set = [node.clone(), neighbors[i].clone(), neighbors[j].clone()];

                    if let Some(con) = graph.get(&set[1]) {
                        if con.contains(&set[2]) {
                            set.sort();
                            sets.insert(set);
                        }
                    }
                }
            }
        }

        let t_sets = sets
            .iter()
            .filter(|set| {
                set[0].starts_with('t') || set[1].starts_with('t') || set[2].starts_with('t')
            })
            .collect::<Vec<_>>();

        let result = t_sets.len();
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let graph = parse_graph(input);
        let mut largest_set = HashSet::new();
        let mut current_set = HashSet::new();

        for (start_node, connections) in graph.iter() {
            current_set.clear();
            current_set.insert(start_node);

            'next_node: for n in connections {
                for &current_node in current_set.iter() {
                    if !graph.get(n).unwrap().contains(current_node) {
                        continue 'next_node;
                    }
                }
                current_set.insert(n);
            }

            if current_set.len() > largest_set.len() {
                largest_set = current_set;
                current_set = HashSet::new();
            }
        }

        let mut largest_fully_connected_graph = largest_set.into_iter().collect::<Vec<_>>();
        largest_fully_connected_graph.sort();

        let mut result = largest_fully_connected_graph
            .iter()
            .fold(String::new(), |acc, &node| acc + node + ",");

        result = result.trim_end_matches(',').to_string();

        Ok(result.to_string())
    }
}
