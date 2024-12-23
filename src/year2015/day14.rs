use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day14 {}

impl Day14 {
    pub fn new() -> Self {
        Self {}
    }
}

struct Reindeer {
    speed: u64,
    fly_time: u64,
    rest_time: u64,
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    let mut reindeers = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let speed = parts[3].parse::<u64>().unwrap();
        let fly_time = parts[6].parse::<u64>().unwrap();
        let rest_time = parts[13].parse::<u64>().unwrap();
        reindeers.push(Reindeer {
            speed,
            fly_time,
            rest_time,
        });
    }
    reindeers
}

impl Challenge for Day14 {
    fn part1(&self, input: &str) -> Result<String> {
        let reindeers = parse_input(input);

        let time = 2503;
        let mut distances = vec![0; reindeers.len()];

        for i in &reindeers {
            let cycle_time = i.fly_time + i.rest_time;
            let cycles = time / cycle_time;
            let remaining = time % cycle_time;
            let fly_time = std::cmp::min(remaining, i.fly_time);
            let distance = cycles * i.speed * i.fly_time + fly_time * i.speed;
            distances.push(distance);
        }

        let max_distance = distances.iter().max().unwrap();
        Ok(max_distance.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let reindeers = parse_input(input);

        let time = 2503;
        let mut points = vec![0; reindeers.len()];
        let mut distances = vec![0; reindeers.len()];

        for i in 1..=time {
            for (j, r) in reindeers.iter().enumerate() {
                let cycle_time = r.fly_time + r.rest_time;
                let cycles = i / cycle_time;
                let remaining = i % cycle_time;
                let fly_time = std::cmp::min(remaining, r.fly_time);
                let distance = cycles * r.speed * r.fly_time + fly_time * r.speed;
                distances[j] = distance;
            }

            let max_distance = distances.iter().max().unwrap();
            for (j, d) in distances.iter().enumerate() {
                if d == max_distance {
                    points[j] += 1;
                }
            }
        }

        let max_points = points.iter().max().unwrap();
        Ok(max_points.to_string())
    }
}
