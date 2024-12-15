use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::HashSet;

pub struct Day15 {}

impl Day15 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Box,
    Robot,
}

type Grid = Vec<Vec<Tile>>;

fn parse_grid(input: &str) -> Grid {
    input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '@' => Tile::Robot,
                    _ => panic!("Invalid character in input"),
                })
                .collect()
        })
        .collect()
}

fn parse_instructions(input: &str) -> Vec<char> {
    input
        .split("\n\n")
        .skip(1)
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .fold(Vec::new(), |mut acc, mut line| {
            acc.append(&mut line);
            acc
        })
}

#[allow(unused)]
fn draw_grid(grid: &Grid) {
    for row in grid {
        for tile in row {
            print!(
                "{}",
                match tile {
                    Tile::Wall => '#',
                    Tile::Empty => '.',
                    Tile::Box => 'O',
                    Tile::Robot => '@',
                }
            );
        }
        println!();
    }
}

#[allow(unused)]
fn draw_grid2(grid: &Grid2) {
    for row in grid {
        for tile in row {
            print!(
                "{}",
                match tile {
                    Tile2::Wall => '#',
                    Tile2::Empty => '.',
                    Tile2::BoxLeft => '[',
                    Tile2::BoxRight => ']',
                    Tile2::Robot => '@',
                }
            );
        }
        println!();
    }
}

fn execute_program(grid: &mut Grid, instructions: &[char]) {
    let mut robot_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|tile| match tile {
                    Tile::Robot => true,
                    _ => false,
                })
                .map(|x| (x as isize, y as isize))
        })
        .unwrap();

    for instruction in instructions {
        // draw_grid(&grid);
        let d = match instruction {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => panic!("Invalid instruction"),
        };

        let mut i = 1;
        loop {
            let cursor = (
                robot_pos.0 as isize + d.0 * i,
                robot_pos.1 as isize + d.1 * i,
            );

            let tile = grid[cursor.1 as usize][cursor.0 as usize];
            match tile {
                Tile::Wall => break,           // cannot move
                Tile::Robot => unreachable!(), // robot should not be on the path
                Tile::Box => i += 1,
                Tile::Empty => {
                    if i == 1 {
                        // empty neightbour. Move just robot
                        grid[robot_pos.1 as usize][robot_pos.0 as usize] = Tile::Empty;
                        grid[cursor.1 as usize][cursor.0 as usize] = Tile::Robot;
                        robot_pos = (cursor.0, cursor.1);
                        break;
                    }

                    grid[robot_pos.1 as usize][robot_pos.0 as usize] = Tile::Empty;
                    assert!(
                        grid[(robot_pos.1 + d.1) as usize][(robot_pos.0 + d.0) as usize]
                            == Tile::Box
                    );
                    grid[(robot_pos.1 + d.1) as usize][(robot_pos.0 + d.0) as usize] = Tile::Robot;
                    grid[cursor.1 as usize][cursor.0 as usize] = Tile::Box;
                    robot_pos = (robot_pos.0 + d.0, robot_pos.1 + d.1);
                    break;
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile2 {
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
    Robot,
}

type Grid2 = Vec<Vec<Tile2>>;

fn mutate_grid(grid: &Grid) -> Grid2 {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|tile| match tile {
                    Tile::Wall => [Tile2::Wall, Tile2::Wall],
                    Tile::Empty => [Tile2::Empty, Tile2::Empty],
                    Tile::Box => [Tile2::BoxLeft, Tile2::BoxRight],
                    Tile::Robot => [Tile2::Robot, Tile2::Empty],
                })
                .fold(Vec::new(), |mut acc, tiles| {
                    acc.push(tiles[0]);
                    acc.push(tiles[1]);
                    acc
                })
        })
        .collect()
}

fn execute_program2(grid: &mut Grid2, instructions: &[char]) {
    let mut robot_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|tile| match tile {
                    Tile2::Robot => true,
                    _ => false,
                })
                .map(|x| (x as isize, y as isize))
        })
        .unwrap();

    for instruction in instructions {
        //draw_grid2(&grid);
        let d = match instruction {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => panic!("Invalid instruction"),
        };

        let mut i = 1;
        let mut fronts = Vec::new();
        let mut x_fronts = HashSet::new();
        x_fronts.insert(robot_pos.0);
        fronts.push(x_fronts);

        'ins: loop {
            let cursor = (robot_pos.0 + d.0 * i, robot_pos.1 + d.1 * i);

            if d.1 == 0 {
                // horizontal

                let tile = grid[cursor.1 as usize][cursor.0 as usize];
                match tile {
                    Tile2::Wall => break,
                    Tile2::Robot => unreachable!(),
                    Tile2::BoxLeft | Tile2::BoxRight => i += 1,
                    Tile2::Empty => {
                        if i == 1 {
                            grid[robot_pos.1 as usize][robot_pos.0 as usize] = Tile2::Empty;
                            grid[cursor.1 as usize][cursor.0 as usize] = Tile2::Robot;
                            robot_pos = (robot_pos.0 + d.0, robot_pos.1 + d.1);
                            break 'ins;
                        }

                        // can move everything
                        let mut replacement_tile = Tile2::Empty;
                        for i in 0..=i {
                            let tmp = grid[(robot_pos.1 + d.1 * i) as usize]
                                [(robot_pos.0 + d.0 * i) as usize];

                            grid[(robot_pos.1 + d.1 * i) as usize]
                                [(robot_pos.0 + d.0 * i) as usize] = replacement_tile;
                            replacement_tile = tmp;
                        }
                        robot_pos = (robot_pos.0 + d.0, robot_pos.1 + d.1);
                        break 'ins;
                    }
                }
            } else {
                // vertical
                let mut next_fronts = HashSet::new();
                for x in fronts.last().unwrap().iter() {
                    let tile = grid[cursor.1 as usize][*x as usize];

                    match tile {
                        Tile2::Wall => break 'ins,
                        Tile2::Robot => unreachable!(),
                        Tile2::BoxLeft => {
                            next_fronts.insert(*x);
                            next_fronts.insert(*x + 1);
                        }
                        Tile2::BoxRight => {
                            next_fronts.insert(*x);
                            next_fronts.insert(*x - 1);
                        }
                        Tile2::Empty => {}
                    }
                }
                i += 1;

                if next_fronts.len() == 0 {
                    // move is possible

                    for (i, front) in fronts.iter().enumerate().rev() {
                        let y = robot_pos.1 + d.1 * (i + 1) as isize;
                        let from_y = robot_pos.1 + d.1 * i as isize;
                        for x in front.iter() {
                            let tile = grid[from_y as usize][*x as usize];
                            grid[y as usize][*x as usize] = tile;
                            grid[from_y as usize][*x as usize] = Tile2::Empty;
                        }
                    }

                    robot_pos = (robot_pos.0 + d.0, robot_pos.1 + d.1);
                    break 'ins;
                }

                fronts.push(next_fronts);
            }
        }
    }
}

impl Challenge for Day15 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut grid = parse_grid(input);
        let instructions = parse_instructions(input);
        let mut result = 0;

        execute_program(&mut grid, &instructions);

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == Tile::Box {
                    result += 100 * y + x;
                }
            }
        }
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut grid = mutate_grid(&parse_grid(input));
        let instructions = parse_instructions(input);
        let mut result = 0;

        execute_program2(&mut grid, &instructions);

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == Tile2::BoxLeft {
                    result += 100 * y + x;
                }
            }
        }
        Ok(result.to_string())
    }
}
