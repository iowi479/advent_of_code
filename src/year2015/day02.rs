use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day02 {}

impl Day02 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Challenge for Day02 {
    fn part1(&self, _input: &str) -> Result<String> {
        let input = _input.lines().map(|line| {
            line.split("x")
                .map(|line| line.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        });

        let mut result = 0;
        for package in input {
            let l = package[0];
            let w = package[1];
            let h = package[2];

            let lw = l * w;
            let wh = w * h;
            let hl = h * l;

            let smallest = lw.min(wh).min(hl);

            result += 2 * lw + 2 * wh + 2 * hl + smallest;
        }
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let packages = input.lines().map(|line| {
            line.split("x")
                .map(|line| line.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        });

        let mut result = 0;

        for mut package in packages {
            package.sort();

            let lw = package[0] + package[0] + package[1] + package[1];
            let bow = package[0] * package[1] * package[2];
            result += lw + bow;
        }

        Ok(result.to_string())
    }
}
