use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day10 {}

impl Day10 {
    pub fn new() -> Self {
        Self {}
    }
}

type Grid = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Tile {
    x: usize,
    y: usize,
    value: u8,
}

fn bfs(grid: &Grid, x: usize, y: usize) -> u32 {
    let mut paths = 0;

    let mut visited = vec![];
    let mut to_visit = vec![Tile { x, y, value: 0 }];

    while let Some(tile) = to_visit.pop() {
        if visited.contains(&tile) {
            continue;
        }
        visited.push(tile);
        if tile.value == 9 {
            paths += 1;
            continue;
        }

        let neighbors = get_neighbors(&grid, tile.x, tile.y);

        for neighbor in neighbors {
            if neighbor.value == tile.value + 1 {
                to_visit.push(neighbor);
            }
        }
    }

    paths
}

fn get_neighbors(grid: &Grid, x: usize, y: usize) -> Vec<Tile> {
    let mut neighbors = vec![];
    if y > 0 {
        neighbors.push(Tile {
            x,
            y: y - 1,
            value: grid[y - 1][x],
        });
    }
    if y < grid.len() - 1 {
        neighbors.push(Tile {
            x,
            y: y + 1,
            value: grid[y + 1][x],
        });
    }
    if x > 0 {
        neighbors.push(Tile {
            x: x - 1,
            y,
            value: grid[y][x - 1],
        });
    }
    if x < grid[y].len() - 1 {
        neighbors.push(Tile {
            x: x + 1,
            y,
            value: grid[y][x + 1],
        });
    }

    neighbors
}

fn bfs_paths(grid: &Grid, x: usize, y: usize) -> u32 {
    let mut paths = 0;
    let mut to_visit = vec![Tile { x, y, value: 0 }];

    while let Some(tile) = to_visit.pop() {
        if tile.value == 9 {
            paths += 1;
            continue;
        }

        let neighbors = get_neighbors(&grid, tile.x, tile.y);

        for neighbor in neighbors {
            if neighbor.value == tile.value + 1 {
                to_visit.push(neighbor);
            }
        }
    }

    paths
}

impl Challenge for Day10 {
    fn part1(&self, input: &str) -> Result<String> {
        let grid = parse_input(input);
        let mut result = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 0 {
                    result += bfs(&grid, x, y);
                }
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let grid = parse_input(input);
        let mut result = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 0 {
                    result += bfs_paths(&grid, x, y);
                }
            }
        }

        Ok(result.to_string())
    }
}
