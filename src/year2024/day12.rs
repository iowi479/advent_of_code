use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::HashSet;

pub struct Day12 {}

impl Day12 {
    pub fn new() -> Self {
        Self {}
    }
}

type Grid = Vec<Vec<char>>;

struct Region {
    size: u64,
    perimeter: u64,
    chars: char,
}

struct Tile {
    x: usize,
    y: usize,
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}
struct FenceRegion {
    chars: char,
    tiles: Vec<Tile>,
}

fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn parse_regions(grid: &Grid) -> Vec<Region> {
    let mut regions = vec![];

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if visited.contains(&(x, y)) {
                continue;
            }

            // new region
            let mut region = Region {
                size: 0,
                perimeter: 0,
                chars: grid[y][x],
            };

            let mut to_visit = vec![(x, y)];
            while let Some((x, y)) = to_visit.pop() {
                if visited.contains(&(x, y)) {
                    continue;
                }
                visited.insert((x, y));
                region.size += 1;

                if x > 0 && grid[y][x - 1] == region.chars {
                    if !visited.contains(&(x - 1, y)) {
                        to_visit.push((x - 1, y));
                    }
                } else {
                    region.perimeter += 1;
                }

                if x < grid[y].len() - 1 && grid[y][x + 1] == region.chars {
                    if !visited.contains(&(x + 1, y)) {
                        to_visit.push((x + 1, y));
                    }
                } else {
                    region.perimeter += 1;
                }

                if y > 0 && grid[y - 1][x] == region.chars {
                    if !visited.contains(&(x, y - 1)) {
                        to_visit.push((x, y - 1));
                    }
                } else {
                    region.perimeter += 1;
                }

                if y < grid.len() - 1 && grid[y + 1][x] == region.chars {
                    if !visited.contains(&(x, y + 1)) {
                        to_visit.push((x, y + 1));
                    }
                } else {
                    region.perimeter += 1;
                }
            }
            regions.push(region);
        }
    }
    regions
}

fn parse_fence_regions(grid: &Grid) -> Vec<FenceRegion> {
    let mut regions = vec![];

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if visited.contains(&(x, y)) {
                continue;
            }

            // new region
            let mut region = FenceRegion {
                chars: grid[y][x],
                tiles: vec![],
            };

            let mut to_visit = vec![(x, y)];
            while let Some((x, y)) = to_visit.pop() {
                if visited.contains(&(x, y)) {
                    continue;
                }
                visited.insert((x, y));
                let mut tile = Tile {
                    x,
                    y,
                    north: false,
                    east: false,
                    south: false,
                    west: false,
                };

                if x > 0 && grid[y][x - 1] == region.chars {
                    if !visited.contains(&(x - 1, y)) {
                        to_visit.push((x - 1, y));
                    }
                } else {
                    tile.west = true;
                }

                if x < grid[y].len() - 1 && grid[y][x + 1] == region.chars {
                    if !visited.contains(&(x + 1, y)) {
                        to_visit.push((x + 1, y));
                    }
                } else {
                    tile.east = true;
                }

                if y > 0 && grid[y - 1][x] == region.chars {
                    if !visited.contains(&(x, y - 1)) {
                        to_visit.push((x, y - 1));
                    }
                } else {
                    tile.north = true;
                }

                if y < grid.len() - 1 && grid[y + 1][x] == region.chars {
                    if !visited.contains(&(x, y + 1)) {
                        to_visit.push((x, y + 1));
                    }
                } else {
                    tile.south = true;
                }

                region.tiles.push(tile);
            }
            regions.push(region);
        }
    }
    regions
}

fn count_sides(region: &FenceRegion) -> u64 {
    let mut sides = 0;
    let xs = region
        .tiles
        .iter()
        .map(|tile| tile.x)
        .collect::<HashSet<usize>>();

    let x_min = *xs.iter().min().unwrap();
    let x_max = *xs.iter().max().unwrap();

    let ys = region
        .tiles
        .iter()
        .map(|tile| tile.y)
        .collect::<HashSet<usize>>();
    let y_min = *ys.iter().min().unwrap();
    let y_max = *ys.iter().max().unwrap();

    for y in y_min..=y_max {
        let mut cur_north_side = 0;
        let mut cur_south_side = 0;

        for x in x_min..=x_max {
            if let Some(tile) = region.tiles.iter().find(|tile| tile.x == x && tile.y == y) {
                if tile.north {
                    cur_north_side += 1;
                } else if cur_north_side > 0 {
                    sides += 1;
                    cur_north_side = 0;
                }

                if tile.south {
                    cur_south_side += 1;
                } else if cur_south_side > 0 {
                    sides += 1;
                    cur_south_side = 0;
                }
            } else {
                if cur_north_side > 0 {
                    sides += 1;
                    cur_north_side = 0;
                }
                if cur_south_side > 0 {
                    sides += 1;
                    cur_south_side = 0;
                }
            }
        }
        if cur_north_side > 0 {
            sides += 1;
        }
        if cur_south_side > 0 {
            sides += 1;
        }
    }

    for x in x_min..=x_max {
        let mut cur_east_side = 0;
        let mut cur_west_side = 0;

        for y in y_min..=y_max {
            if let Some(tile) = region.tiles.iter().find(|tile| tile.x == x && tile.y == y) {
                if tile.east {
                    cur_east_side += 1;
                } else if cur_east_side > 0 {
                    sides += 1;
                    cur_east_side = 0;
                }

                if tile.west {
                    cur_west_side += 1;
                } else if cur_west_side > 0 {
                    sides += 1;
                    cur_west_side = 0;
                }
            } else {
                if cur_east_side > 0 {
                    sides += 1;
                    cur_east_side = 0;
                }
                if cur_west_side > 0 {
                    sides += 1;
                    cur_west_side = 0;
                }
            }
        }
        if cur_east_side > 0 {
            sides += 1;
        }
        if cur_west_side > 0 {
            sides += 1;
        }
    }

    sides
}

impl Challenge for Day12 {
    fn part1(&self, input: &str) -> Result<String> {
        let grid = parse_input(input);
        let mut result = 0;
        let regions = parse_regions(&grid);

        for region in regions {
            result += region.size * region.perimeter;
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let grid = parse_input(input);
        let mut result = 0;
        let regions = parse_fence_regions(&grid);

        for region in regions {
            let sides = count_sides(&region);
            result += region.tiles.len() * sides as usize;
        }

        Ok(result.to_string())
    }
}
