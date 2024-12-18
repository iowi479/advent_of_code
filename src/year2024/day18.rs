use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::HashSet;

pub struct Day18 {}

impl Day18 {
    pub fn new() -> Self {
        Self {}
    }
}

fn parse_inputs(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

fn dijkstra(blocked: &[(usize, usize)]) -> usize {
    let mut visited = HashSet::new();
    let mut queue = vec![(0, (0, 0))];

    while let Some((steps, (x, y))) = queue.pop() {
        if x == WIDTH - 1 && y == HEIGHT - 1 {
            return steps;
        }

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        if x > 0 && !blocked.contains(&(x - 1, y)) {
            queue.push((steps + 1, (x - 1, y)));
        }
        if x < WIDTH - 1 && !blocked.contains(&(x + 1, y)) {
            queue.push((steps + 1, (x + 1, y)));
        }
        if y > 0 && !blocked.contains(&(x, y - 1)) {
            queue.push((steps + 1, (x, y - 1)));
        }
        if y < HEIGHT - 1 && !blocked.contains(&(x, y + 1)) {
            queue.push((steps + 1, (x, y + 1)));
        }
        queue.sort_by(|(a, _), (b, _)| b.cmp(a));
    }
    usize::MAX
}

impl Challenge for Day18 {
    fn part1(&self, input: &str) -> Result<String> {
        let drops = parse_inputs(input);
        let idx = usize::min(drops.len() - 1, 1024);
        let distance = dijkstra(&drops[..idx]);
        Ok(distance.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let drops = parse_inputs(input);
        let mut steps = (1024, drops.len() - 1);

        while steps.0 < steps.1 - 1 {
            let mid = (steps.0 + steps.1) / 2;
            let idx = usize::min(drops.len() - 1, mid);

            let distance = dijkstra(&drops[..idx]);
            if distance == usize::MAX {
                if mid < steps.1 {
                    steps.1 = mid;
                }
            } else {
                if mid > steps.0 {
                    steps.0 = mid;
                }
            }
        }

        Ok(format!("{},{}", drops[steps.1 - 1].0, drops[steps.1 - 1].1))
    }
}
