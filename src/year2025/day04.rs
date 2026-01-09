use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day04 {}

impl Day04 {
    pub fn new() -> Self {
        Self {}
    }
}

enum Cell {
    Empty,
    Roll,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}

fn parse_grid(input: &str) -> Grid {
    let mut cells = Vec::new();

    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| match c {
                '.' => Cell::Empty,
                '@' => Cell::Roll,
                _ => panic!("Invalid character in grid"),
            })
            .collect::<Vec<_>>();

        cells.push(row);
    }

    Grid { cells }
}

impl Challenge for Day04 {
    fn part1(&self, input: &str) -> Result<String> {
        let grid = parse_grid(input);

        let mut result = 0;

        for x in 0..grid.cells.len() {
            for y in 0..grid.cells[x].len() {
                if let Cell::Empty = grid.cells[x][y] {
                    continue;
                }

                let mut rolls = 0;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        // not self
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        if let Some(Cell::Roll) = grid
                            .cells
                            .get((x as isize + dx) as usize)
                            .and_then(|row| row.get((y as isize + dy) as usize))
                        {
                            rolls += 1;
                        }
                    }
                }

                if rolls < 4 {
                    result += 1;
                }
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut grid = parse_grid(input);

        let mut result = 0;
        let mut have_removed = true;
        let mut to_remove = Vec::new();

        while have_removed {
            have_removed = false;
            to_remove.clear();

            for x in 0..grid.cells.len() {
                for y in 0..grid.cells[x].len() {
                    if let Cell::Empty = grid.cells[x][y] {
                        continue;
                    }

                    let mut rolls = 0;

                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            // not self
                            if dx == 0 && dy == 0 {
                                continue;
                            }

                            if let Some(Cell::Roll) = grid
                                .cells
                                .get((x as isize + dx) as usize)
                                .and_then(|row| row.get((y as isize + dy) as usize))
                            {
                                rolls += 1;
                            }
                        }
                    }

                    if rolls < 4 {
                        result += 1;
                        have_removed = true;
                        to_remove.push((x, y));
                    }
                }
            }

            for (x, y) in &to_remove {
                grid.cells[*x][*y] = Cell::Empty;
            }
        }

        Ok(result.to_string())
    }
}
