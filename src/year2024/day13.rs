use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::HashSet;

pub struct Day13 {}

impl Day13 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, Copy)]
struct Game {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    dx: i64,
    dy: i64,
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .split("\n\n")
        .map(|lines| {
            let mut lines = lines.lines();

            let a_parts = lines
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .replace("X", "")
                .replace("Y", "")
                .split(",")
                .map(|n| n.trim().parse().unwrap())
                .collect::<Vec<i64>>();

            let b_parts = lines
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .replace("X", "")
                .replace("Y", "")
                .split(",")
                .map(|n| n.trim().parse().unwrap())
                .collect::<Vec<i64>>();

            let d_parts = lines
                .next()
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .replace("X=", "")
                .replace("Y=", "")
                .split(",")
                .map(|n| n.trim().parse().unwrap())
                .collect::<Vec<i64>>();

            Game {
                ax: a_parts[0],
                ay: a_parts[1],
                bx: b_parts[0],
                by: b_parts[1],
                dx: d_parts[0],
                dy: d_parts[1],
            }
        })
        .collect()
}

fn extended_euclid(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if b == 0 {
        *x = 1;
        *y = 0;
        return a;
    }

    let mut x1 = 0;
    let mut y1 = 0;
    let d = extended_euclid(b, a % b, &mut x1, &mut y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
    d
}

fn matrix_lgs(game: &Game) -> Option<(i64, i64)> {
    let det = game.ax * game.by - game.ay * game.bx;
    let det_b = game.dx * game.by - game.dy * game.bx;
    let det_a = game.ax * game.dy - game.ay * game.dx;

    if det != 0 {
        let x = det_b / det;
        if det_b % det != 0 {
            return None;
        }
        let y = det_a / det;
        if det_a % det != 0 {
            return None;
        }
        return Some((x, y));
    }

    if det == 0 && (det_a != 0 || det_b != 0) {
        return None;
    }

    return game_solution(game);
}

fn game_solution(game: &Game) -> Option<(i64, i64)> {
    let mut xx = 0;
    let mut xy = 0;
    let gcd = extended_euclid(game.ax, game.bx, &mut xx, &mut xy);
    xx = xx * game.dx / gcd;
    xy = xy * game.dx / gcd;
    let x_multiple = (game.bx / gcd, -game.ax / gcd);

    let mut yx = 0;
    let mut yy = 0;
    let gcd = extended_euclid(game.ay, game.by, &mut yx, &mut yy);
    yx = yx * game.dy / gcd;
    yy = yy * game.dy / gcd;
    let y_multiple = (game.by / gcd, -game.ay / gcd);

    let mut min_solutions_x: HashSet<(i64, i64)> = HashSet::new();
    for i in (-(xx / x_multiple.0))..=(-(xy / x_multiple.1)) {
        let x = xx + i * x_multiple.0;
        let y = xy + i * x_multiple.1;

        if (x - yx) % y_multiple.0 != 0 {
            continue;
        }
        if (y - yy) % y_multiple.1 != 0 {
            continue;
        }
        let j = (x - yx) / y_multiple.0;
        if yx + j * y_multiple.0 != x {
            continue;
        }
        if yy + j * y_multiple.1 != y {
            continue;
        }

        min_solutions_x.insert((x, y));
    }

    return min_solutions_x
        .iter()
        .min_by_key(|(x, y)| x * 3 + y)
        .cloned();
}

fn p1_solution(game: &Game) -> Option<(i64, i64)> {
    let mut solutions = vec![];

    let a_max = i64::max(game.dx / game.ax, game.dy / game.ay);
    let b_max = i64::max(game.dx / game.bx, game.dy / game.by);
    for a in 0..=a_max + 1 {
        for b in 0..=b_max + 1 {
            if game.ax * a + game.bx * b == game.dx && game.ay * a + game.by * b == game.dy {
                solutions.push((a, b));
            }
        }
    }

    let min_solution = solutions.iter().min_by_key(|(a, b)| a + b);
    min_solution.cloned()
}

impl Challenge for Day13 {
    fn part1(&self, input: &str) -> Result<String> {
        let games = parse_games(input);
        let mut result = 0;

        for game in games {
            let min_solution = p1_solution(&game);
            if let Some(solution) = min_solution {
                result += solution.0 * 3 + solution.1;
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut games = parse_games(input);
        let mut result = 0;

        for game in games.iter_mut() {
            game.dx += 10000000000000;
            game.dy += 10000000000000;
            let solution = matrix_lgs(&game);
            //let p1_solution = p1_solution(&game);

            if let Some(solution) = solution {
                result += solution.0 * 3 + solution.1;
            }
        }
        Ok(result.to_string())
    }
}
