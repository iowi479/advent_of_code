use crate::challenge::Challenge;
use anyhow::{anyhow, Result};
use std::collections::HashSet;

pub struct Day06 {}

impl Day06 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Blocked,
    Guard(Direction),
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Grid = Vec<Vec<Tile>>;

fn parse_input(input: &str) -> Result<Grid> {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Blocked,
                    'v' => Tile::Guard(Direction::Down),
                    '^' => Tile::Guard(Direction::Up),
                    '<' => Tile::Guard(Direction::Left),
                    '>' => Tile::Guard(Direction::Right),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    Ok(grid)
}

fn find_guard(grid: &Grid) -> Result<(usize, usize, Direction)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Tile::Guard(direction) = tile {
                return Ok((x, y, *direction));
            }
        }
    }
    Err(anyhow!("Guard not found"))
}

fn walk_path(mut grid: Grid) -> Result<HashSet<(usize, usize)>> {
    let mut visited = HashSet::new();

    let mut pos = find_guard(&grid)?;

    grid[pos.1][pos.0] = Tile::Empty;
    visited.insert((pos.0, pos.1));

    loop {
        let (x, y, direction) = pos;

        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x < 0 || new_y < 0 || new_x >= grid[0].len() as i32 || new_y >= grid.len() as i32 {
            break;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        match grid[new_y][new_x] {
            Tile::Empty => {
                pos = (new_x, new_y, direction);
                visited.insert((new_x, new_y));
            }
            Tile::Blocked => {
                pos = (
                    x,
                    y,
                    match direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    },
                );
                continue;
            }
            Tile::Guard(_) => {
                unreachable!()
            }
        }
    }

    Ok(visited)
}

fn is_loop(grid: &Grid) -> bool {
    let guard_pos = find_guard(grid);
    if guard_pos.is_err() {
        return false;
    }

    let mut visited = HashSet::new();

    let original_pos = guard_pos.unwrap();
    let mut pos = original_pos;

    loop {
        let (x, y, direction) = pos;

        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x < 0 || new_y < 0 || new_x >= grid[0].len() as i32 || new_y >= grid.len() as i32 {
            break;
        }

        let new_x = new_x as usize;
        let new_y = new_y as usize;

        match grid[new_y][new_x] {
            Tile::Empty | Tile::Guard(_) => {
                let new = visited.insert(pos);
                if !new {
                    return true;
                }
                pos = (new_x, new_y, direction);
            }
            Tile::Blocked => {
                pos = (
                    x,
                    y,
                    match direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    },
                );
                continue;
            }
        }
    }

    return false;
}

impl Challenge for Day06 {
    fn part1(&self, input: &str) -> Result<String> {
        let grid = parse_input(input)?;
        let visited = walk_path(grid);

        Ok(visited?.len().to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let grid = parse_input(input)?;
        let visited = walk_path(grid.clone())?;
        let mut result = 0;

        for (x, y) in visited {
            let mut mod_grid = grid.clone();
            mod_grid[y][x] = Tile::Blocked;
            if is_loop(&mod_grid) {
                result += 1;
            }
        }

        Ok(result.to_string())
    }
}
