use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day07 {}

impl Day07 {
    pub fn new() -> Self {
        Self {}
    }
}

fn is_solvable(res: u64, tasks: &[u64]) -> bool {
    let mut result = vec![tasks[0]];
    for i in 1..tasks.len() {
        let mut new_result = Vec::new();
        for r in &result {
            let addition = r + tasks[i];
            let multiplication = r * tasks[i];
            if addition <= res {
                new_result.push(addition);
            }
            if multiplication <= res {
                new_result.push(multiplication);
            }
        }
        result = new_result;
    }

    result.contains(&res)
}

fn is_solvable_2(res: u64, tasks: &[u64]) -> bool {
    let mut result = vec![tasks[0]];
    for i in 1..tasks.len() {
        let mut new_result = Vec::new();
        for r in &result {
            let addition = r + tasks[i];
            let multiplication = r * tasks[i];
            let concatenation = format!("{}{}", r, tasks[i]).parse::<u64>().unwrap();
            if addition <= res {
                new_result.push(addition);
            }
            if multiplication <= res {
                new_result.push(multiplication);
            }
            if concatenation <= res {
                new_result.push(concatenation);
            }
        }
        result = new_result;
    }

    result.contains(&res)
}
impl Challenge for Day07 {
    fn part1(&self, input: &str) -> Result<String> {
        let equations = input
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(res_str, task_str)| {
                let res = res_str.parse::<u64>().unwrap();
                let tasks = task_str
                    .split_whitespace()
                    .map(|task| task.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                (res, tasks)
            })
            .collect::<Vec<(u64, Vec<u64>)>>();

        let mut result: u64 = 0;

        for eq in equations {
            if is_solvable(eq.0, &eq.1) {
                result += eq.0;
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let equations = input
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(res_str, task_str)| {
                let res = res_str.parse::<u64>().unwrap();
                let tasks = task_str
                    .split_whitespace()
                    .map(|task| task.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                (res, tasks)
            })
            .collect::<Vec<(u64, Vec<u64>)>>();

        let mut result: u64 = 0;

        for eq in equations {
            if is_solvable_2(eq.0, &eq.1) {
                result += eq.0;
            }
        }

        Ok(result.to_string())
    }
}
