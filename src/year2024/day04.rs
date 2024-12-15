use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day04 {}

impl Day04 {
    pub fn new() -> Self {
        Self {}
    }
}

fn input_to_lines(input: &str) -> Vec<String> {
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let mut vertical: Vec<String> = vec![String::new(); lines[0].len()];
    let mut diagonal: Vec<String> = vec![String::new(); lines[0].len() + lines.len()];
    let mut diagonal2: Vec<String> = vec![String::new(); lines[0].len() + lines.len()];

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            vertical[j].push(c);
            diagonal[(lines.len() - 1 - i) + j].push(c);
            diagonal2[(lines.len() - 1 - i) + (lines[0].len() - 1 - j)].push(c);
        }
    }

    let mut all: Vec<String> = Vec::with_capacity(lines.len() + vertical.len() + diagonal.len());
    all.extend_from_slice(&lines);
    all.extend_from_slice(&vertical);
    all.extend_from_slice(&diagonal);
    all.extend_from_slice(&diagonal2);
    all
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

impl Challenge for Day04 {
    fn part1(&self, input: &str) -> Result<String> {
        let lines = input_to_lines(input);
        let mut result = 0;

        for line_str in lines {
            let mut line: &str = &line_str;

            while line.len() >= 4 {
                if line.starts_with("XMAS") {
                    result += 1;
                    line = &line[3..];
                    continue;
                } else if line.starts_with("SAMX") {
                    result += 1;
                    line = &line[3..];
                    continue;
                }
                line = &line[1..];
            }
        }
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let grid = parse_input(input);
        let mut result = 0;

        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                if grid[i][j] == 'A' {
                    match (
                        grid[i - 1][j - 1],
                        grid[i + 1][j + 1],
                        grid[i + 1][j - 1],
                        grid[i - 1][j + 1],
                    ) {
                        ('M', 'S', 'M', 'S') => result += 1,
                        ('S', 'M', 'M', 'S') => result += 1,
                        ('M', 'S', 'S', 'M') => result += 1,
                        ('S', 'M', 'S', 'M') => result += 1,
                        _ => (),
                    }
                }
            }
        }

        Ok(result.to_string())
    }
}
