use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day06 {}

impl Day06 {
    pub fn new() -> Self {
        Self {}
    }
}

enum Instruction {
    Toggle,
    TurnOn,
    TurnOff,
}

struct Interval {
    start: (usize, usize),
    end: (usize, usize),
}

struct Command {
    instruction: Instruction,
    interval: Interval,
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let instruction = match parts.next().unwrap() {
                "toggle" => Instruction::Toggle,
                "turn" => match parts.next().unwrap() {
                    "on" => Instruction::TurnOn,
                    "off" => Instruction::TurnOff,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            let mut start_parts = parts.next().unwrap().split(",");
            let start = (
                start_parts.next().unwrap().parse().unwrap(),
                start_parts.next().unwrap().parse().unwrap(),
            );

            parts.next(); // skip "through"

            let mut end_parts = parts.next().unwrap().split(",");
            let end = (
                end_parts.next().unwrap().parse().unwrap(),
                end_parts.next().unwrap().parse().unwrap(),
            );

            Command {
                instruction,
                interval: Interval { start, end },
            }
        })
        .collect()
}

struct Grid {
    lights: Vec<Vec<u64>>,
}

impl Grid {
    fn new() -> Self {
        let mut lights = Vec::new();
        for _ in 0..1000 {
            let row = vec![0; 1000];
            lights.push(row);
        }

        Self { lights }
    }

    fn apply(&mut self, command: &Command) {
        for y in command.interval.start.1..=command.interval.end.1 {
            for x in command.interval.start.0..=command.interval.end.0 {
                match command.instruction {
                    Instruction::Toggle => self.lights[y][x] += 2,
                    Instruction::TurnOn => self.lights[y][x] += 1,
                    Instruction::TurnOff => {
                        if self.lights[y][x] > 0 {
                            self.lights[y][x] -= 1
                        }
                    }
                }
            }
        }
    }

    fn count_on(&self) -> u64 {
        self.lights
            .iter()
            .map(|row| {
                row.iter()
                    .map(|light| if *light > 0 { 1 } else { 0 })
                    .sum::<u64>()
            })
            .sum()
    }

    fn count_brightness(&self) -> u64 {
        self.lights.iter().map(|row| row.iter().sum::<u64>()).sum()
    }
}

impl Challenge for Day06 {
    fn part1(&self, input: &str) -> Result<String> {
        let commands = parse_input(input);

        let mut grid = Grid::new();
        for command in commands {
            grid.apply(&command);
        }

        let result = grid.count_on();

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let commands = parse_input(input);

        let mut grid = Grid::new();
        for command in commands {
            grid.apply(&command);
        }

        let result = grid.count_brightness();

        Ok(result.to_string())
    }
}
