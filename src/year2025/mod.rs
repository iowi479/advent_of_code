use crate::challenge::DailyChallenge;
use anyhow::Result;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

/// Creates an Instance of the challenge for the given day.
pub fn find_challenge(day: u8) -> Result<DailyChallenge> {
    match day {
        1 => Ok(Box::new(day01::Day01::new())),
        2 => Ok(Box::new(day02::Day02::new())),
        3 => Ok(Box::new(day03::Day03::new())),
        4 => Ok(Box::new(day04::Day04::new())),
        5 => Ok(Box::new(day05::Day05::new())),
        6 => Ok(Box::new(day06::Day06::new())),
        7 => Ok(Box::new(day07::Day07::new())),
        8 => Ok(Box::new(day08::Day08::new())),
        9 => Ok(Box::new(day09::Day09::new())),
        10 => Ok(Box::new(day10::Day10::new())),
        11 => Ok(Box::new(day11::Day11::new())),
        12 => Ok(Box::new(day12::Day12::new())),
        _ => Err(anyhow::anyhow!("Day {} is not in Advent of Code", day)),
    }
}
