use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day14 {}

impl Day14 {
    pub fn new() -> Self {
        Self {}
    }
}
const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

// const WIDTH: i64 = 11;
// const HEIGHT: i64 = 7;

fn is_in_quadrant((x, y): (i64, i64)) -> Option<usize> {
    if x < WIDTH / 2 && y < HEIGHT / 2 {
        return Some(0);
    }
    if x > WIDTH / 2 && y < HEIGHT / 2 {
        return Some(1);
    }
    if x < WIDTH / 2 && y > HEIGHT / 2 {
        return Some(2);
    }
    if x > WIDTH / 2 && y > HEIGHT / 2 {
        return Some(3);
    }

    None
}

fn predict(robot: &Robot, steps: i64) -> (i64, i64) {
    let mut x = robot.pos.0;
    let mut y = robot.pos.1;
    for _ in 0..steps {
        x = (x + robot.dir.0) % WIDTH;
        y = (y + robot.dir.1) % HEIGHT;
        if x < 0 {
            x += WIDTH;
        }
        if y < 0 {
            y += HEIGHT;
        }
    }

    (x, y)
}

struct Robot {
    pos: (i64, i64),
    dir: (i64, i64),
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let mut pos_parts = parts.next().unwrap()[2..].split(',');
            let x = pos_parts.next().unwrap().parse().unwrap();
            let y = pos_parts.next().unwrap().parse().unwrap();
            let mut dir_parts = parts.next().unwrap()[2..].split(',');
            let dx = dir_parts.next().unwrap().parse().unwrap();
            let dy = dir_parts.next().unwrap().parse().unwrap();

            Robot {
                pos: (x, y),
                dir: (dx, dy),
            }
        })
        .collect()
}

impl Challenge for Day14 {
    fn part1(&self, input: &str) -> Result<String> {
        let robots = parse_input(input);
        let mut result = vec![0; 4];

        for robot in robots {
            let pos = predict(&robot, 100);
            if let Some(i) = is_in_quadrant(pos) {
                result[i] += 1;
            }
        }

        let result = result.iter().fold(1, |acc, x| acc * x);
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut robots = parse_input(input);

        for i in 0..(WIDTH * HEIGHT) {
            let mut pic = vec![vec![' '; WIDTH as usize]; HEIGHT as usize];
            for robot in &mut robots {
                robot.pos = predict(&robot, 1);
                pic[robot.pos.1 as usize][robot.pos.0 as usize] = 'X';
            }

            if line_in_pic(&pic) {
                return Ok((i + 1).to_string());
            }
        }

        Ok("No tree found!".to_string())
    }
}

#[allow(unused)]
fn print_pic(pic: &Vec<Vec<char>>) {
    for row in pic {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn line_in_pic(pic: &Vec<Vec<char>>) -> bool {
    for row in pic {
        let mut counter = 0;
        for c in row {
            if *c == 'X' {
                counter += 1;
                if counter >= 7 {
                    return true;
                }
            } else {
                counter = 0;
            }
        }
    }

    false
}
