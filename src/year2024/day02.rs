use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day02 {}

impl Day02 {
    pub fn new() -> Self {
        Self {}
    }
}

enum Mode {
    None,
    Increasing,
    Decreasing,
}

fn parse_reports(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace())
        .map(|levels| {
            levels
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
}

fn check_report(report: &Vec<i32>) -> bool {
    let mut last = report[0];
    let mut mode = Mode::None;
    for i in 1..report.len() {
        if i32::abs(report[i] - last) > 3 {
            // unsafe report
            return false;
        }

        match mode {
            Mode::None => {
                if report[i] > last {
                    mode = Mode::Increasing;
                } else if report[i] < last {
                    mode = Mode::Decreasing;
                } else {
                    return false;
                }
            }
            Mode::Increasing => {
                if report[i] <= last {
                    return false;
                }
            }
            Mode::Decreasing => {
                if report[i] >= last {
                    return false;
                }
            }
        }
        last = report[i];
    }

    true
}

impl Challenge for Day02 {
    fn part1(&self, input: &str) -> Result<String> {
        let reports = parse_reports(input);

        let mut safe_counter = 0;

        for report in reports {
            if check_report(&report) {
                safe_counter += 1;
            }
        }

        Ok(safe_counter.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let reports = parse_reports(input);

        let mut safe_counter = 0;

        for report in reports {
            let mut permutations = Vec::new();
            for r in 0..report.len() {
                let modified_report = report
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &v)| if i == r { None } else { Some(v) })
                    .collect::<Vec<i32>>();

                permutations.push(modified_report);
            }

            let any_safe = permutations
                .iter()
                .map(|report| check_report(report))
                .any(|x| x);

            if any_safe {
                // atleast one permutation is safe
                safe_counter += 1;
            }
        }

        Ok(safe_counter.to_string())
    }
}
