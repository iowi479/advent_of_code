use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

pub struct Day08 {}

impl Day08 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn parse_input(input: &str) -> Result<HashMap<char, Vec<Position>>> {
    let mut result: HashMap<char, Vec<Position>> = HashMap::new();

    for y in 0..input.lines().count() {
        for (x, c) in input.lines().nth(y).unwrap().chars().enumerate() {
            if c != '.' {
                result.entry(c).or_insert_with(Vec::new).push(Position {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    Ok(result)
}

impl Challenge for Day08 {
    fn part1(&self, input: &str) -> Result<String> {
        let antennas = parse_input(input)?;
        let mut antinodes: HashSet<Position> = HashSet::new();

        for (_, positions) in antennas.iter() {
            for a in positions {
                for b in positions {
                    if a != b {
                        let dx = a.x - b.x;
                        let dy = a.y - b.y;

                        let pos1 = Position {
                            x: a.x + dx,
                            y: a.y + dy,
                        };
                        let pos2 = Position {
                            x: b.x - dx,
                            y: b.y - dy,
                        };

                        antinodes.insert(pos1);
                        antinodes.insert(pos2);
                    }
                }
            }
        }

        antinodes.retain(|p| p.x >= 0 && p.y >= 0);
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;

        antinodes.retain(|p| p.x < width && p.y < height);

        Ok(antinodes.len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let antennas = parse_input(input)?;
        let mut antinodes: HashSet<Position> = HashSet::new();

        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;

        for (_, positions) in antennas.iter() {
            for a in positions {
                for b in positions {
                    if a != b {
                        let dx = a.x - b.x;
                        let dy = a.y - b.y;

                        let mut x = a.x;
                        let mut y = a.y;
                        while x >= 0 && y >= 0 && x < width && y < height {
                            antinodes.insert(Position { x, y });
                            x += dx;
                            y += dy;
                        }

                        x = b.x;
                        y = b.y;
                        while x >= 0 && y >= 0 && x < width && y < height {
                            antinodes.insert(Position { x, y });
                            x -= dx;
                            y -= dy;
                        }
                    }
                }
            }
        }

        antinodes.retain(|p| p.x >= 0 && p.y >= 0);
        let width = input.lines().next().unwrap().len() as i32;
        let height = input.lines().count() as i32;

        antinodes.retain(|p| p.x < width && p.y < height);

        Ok(antinodes.len().to_string())
    }
}
