use crate::challenge::Challenge;
use anyhow::Result;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct Day16 {}

impl Day16 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
}

struct Grid {
    board: Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_input(input: &str) -> Grid {
    let mut board = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(Tile::Empty),
                '#' => row.push(Tile::Wall),
                'S' => {
                    row.push(Tile::Empty);
                    start = (x, y);
                }
                'E' => {
                    row.push(Tile::Empty);
                    end = (x, y);
                }
                _ => panic!("Invalid character in input"),
            }
        }
        board.push(row);
    }

    Grid { board, start, end }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_cost(&self, other: &Direction) -> usize {
        if self == other {
            return 0;
        }
        const TURN_COST: usize = 1000;
        (match (self, other) {
            (Direction::Up, Direction::Down) => 2,
            (Direction::Down, Direction::Up) => 2,
            (Direction::Left, Direction::Right) => 2,
            (Direction::Right, Direction::Left) => 2,
            _ => 1,
        } * TURN_COST)
    }
}

fn dijkstra(grid: &Grid) -> usize {
    let mut to_visit = vec![(0, Direction::Right, grid.start)];
    let mut visited = HashSet::new();

    while !to_visit.is_empty() {
        to_visit.sort_by(|a, b| b.0.cmp(&a.0));
        let (base_cost, base_dir, (x, y)) = to_visit.pop().unwrap();

        if (x, y) == grid.end {
            return base_cost;
        }

        if visited.contains(&(base_dir, (x, y))) {
            continue;
        }

        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let cost = base_cost + 1 + base_dir.turn_cost(dir);
            let (nx, ny) = match dir {
                Direction::Up => (x, y - 1),
                Direction::Down => (x, y + 1),
                Direction::Left => (x - 1, y),
                Direction::Right => (x + 1, y),
            };

            match grid.board[ny][nx] {
                Tile::Empty => {
                    to_visit.push((cost, *dir, (nx, ny)));
                }
                Tile::Wall => {}
            }
        }

        visited.insert((base_dir, (x, y)));
    }

    usize::MAX
}

#[derive(Debug, Clone)]
struct Path {
    cost: usize,
    pos: (usize, usize),
    dir: Direction,
    nodes: Vec<(usize, usize)>,
}

fn dijkstra2(grid: &mut Grid) -> usize {
    let initial_path = Path {
        cost: 0,
        pos: grid.start,
        dir: Direction::Right,
        nodes: vec![],
    };
    let mut shortest_path = usize::MAX;
    let mut shortest_paths = vec![];
    let mut paths = vec![initial_path];
    let mut visited = HashMap::new();

    while !paths.is_empty() {
        paths.sort_by(|a, b| b.cost.cmp(&a.cost));
        let path = paths.pop().unwrap();

        if path.cost > shortest_path {
            continue;
        }

        if path.pos == grid.end {
            shortest_path = shortest_path.min(path.cost);
            shortest_paths.push(path);
            continue;
        }

        if let Some(c) = visited.get(&(path.pos, path.dir)) {
            if *c < path.cost {
                continue;
            }
        }

        if path.nodes.contains(&path.pos) {
            continue;
        }

        for dir in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let cost = path.cost + 1 + path.dir.turn_cost(dir);
            let (nx, ny) = match dir {
                Direction::Up => (path.pos.0, path.pos.1 - 1),
                Direction::Down => (path.pos.0, path.pos.1 + 1),
                Direction::Left => (path.pos.0 - 1, path.pos.1),
                Direction::Right => (path.pos.0 + 1, path.pos.1),
            };

            if grid.board[ny][nx] == Tile::Wall {
                continue;
            }

            let mut pn = Path {
                cost,
                pos: (nx, ny),
                dir: *dir,
                nodes: path.nodes.clone(),
            };
            pn.nodes.push(path.pos);
            paths.push(pn);
        }

        visited.insert((path.pos, path.dir), path.cost);
    }

    let mut nodes = HashSet::new();
    nodes.insert(grid.end);

    for path in shortest_paths {
        for node in path.nodes {
            nodes.insert(node);
        }
    }
    nodes.len()
}

impl Challenge for Day16 {
    fn part1(&self, input: &str) -> Result<String> {
        let grid = parse_input(input);
        let result = dijkstra(&grid);
        Ok(result.to_string())
    }

    fn part2(&self, _input: &str) -> Result<String> {
        let mut grid = parse_input(_input);
        let result = dijkstra2(&mut grid);
        Ok(result.to_string())
    }
}
