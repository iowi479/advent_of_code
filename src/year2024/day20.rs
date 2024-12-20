use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::HashMap;

pub struct Day20 {}

impl Day20 {
    pub fn new() -> Self {
        Self {}
    }
}

struct Grid {
    tiles: Vec<Vec<Option<usize>>>,
    start: (usize, usize),
}

fn parse_input(input: &str) -> Grid {
    let mut tiles = Vec::new();
    let mut start = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => row.push(None),
                '.' | 'E' => row.push(Some(usize::MAX)),
                'S' => {
                    row.push(Some(0));
                    start = (x, y);
                }
                _ => panic!("Invalid character in input"),
            }
        }
        tiles.push(row);
    }

    Grid { tiles, start }
}

fn pathfinder(grid: &mut Grid) {
    let mut queue = Vec::new();
    queue.push(grid.start);

    while let Some((x, y)) = queue.pop() {
        for d in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let (dx, dy) = d;
            let x1 = x as isize + dx;
            let y1 = y as isize + dy;

            if x1 < 0
                || y1 < 0
                || y1 as usize >= grid.tiles.len()
                || x1 as usize >= grid.tiles[y1 as usize].len()
            {
                continue;
            }

            let x1 = x1 as usize;
            let y1 = y1 as usize;

            let v = grid.tiles[y][x].unwrap();

            if let Some(v1) = grid.tiles[y1][x1] {
                if v1 > v + 1 {
                    grid.tiles[y1][x1] = Some(v + 1);
                    queue.push((x1, y1));
                }
            }
        }
    }
}

fn find_cheats(grid: &Grid, distance: isize) -> HashMap<usize, usize> {
    let mut cheats = HashMap::new();

    for y in 0..grid.tiles.len() {
        for x in 0..grid.tiles[y].len() {
            for dy in -distance..=distance {
                for dx in -distance..=distance {
                    if dx.abs() + dy.abs() <= distance {
                        let x1 = x as isize + dx;
                        let y1 = y as isize + dy;

                        if x1 < 0
                            || y1 < 0
                            || y1 as usize >= grid.tiles.len()
                            || x1 as usize >= grid.tiles[y1 as usize].len()
                        {
                            continue;
                        }

                        let x1 = x1 as usize;
                        let y1 = y1 as usize;

                        if let Some(v) = grid.tiles[y][x] {
                            if let Some(v1) = grid.tiles[y1][x1] {
                                if v1 != usize::MAX {
                                    let cheat_cost = (dx.abs() + dy.abs()) as usize;
                                    if v1 > v + cheat_cost {
                                        let entry = cheats.entry(v1 - v - cheat_cost).or_insert(0);
                                        *entry += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    cheats
}

impl Challenge for Day20 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut grid = parse_input(input);
        pathfinder(&mut grid);

        let cheats = find_cheats(&grid, 2);

        let result = cheats
            .iter()
            .filter(|(k, _)| *k >= &100)
            .map(|(_, v)| v)
            .sum::<usize>();

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut grid = parse_input(input);
        pathfinder(&mut grid);

        let cheats = find_cheats(&grid, 20);

        let result = cheats
            .iter()
            .filter(|(k, _)| *k >= &100)
            .map(|(_, v)| v)
            .sum::<usize>();

        Ok(result.to_string())
    }
}
