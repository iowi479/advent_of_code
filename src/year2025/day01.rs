use crate::challenge::Challenge;
use anyhow::Result;

pub struct Day01 {}

impl Day01 {
    pub fn new() -> Self {
        Self {}
    }
}

const DIAL_COUNT: i32 = 100;
const DIAL_START: i32 = 50;

struct Dial {
    position: i32,
    ticks: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            position: DIAL_START,
            ticks: 0,
        }
    }
}

impl Dial {
    fn turn_left(&mut self, amount: i32) {
        if self.position == amount {
            self.position = 0;
            self.ticks += 1;
        } else if self.position > amount {
            self.position -= amount;
        } else {
            let mut left = amount;
            while left > 0 {
                self.position -= 1;
                if self.position < 0 {
                    self.position = DIAL_COUNT - 1;
                }
                left -= 1;
                if self.position == 0 {
                    self.ticks += 1;
                }
            }
        }
    }

    fn turn_right(&mut self, amount: i32) {
        if self.position == DIAL_COUNT - amount {
            self.position = 0;
            self.ticks += 1;
        } else if DIAL_COUNT - self.position > amount {
            self.position += amount;
        } else {
            let mut left = amount;
            while left > 0 {
                self.position += 1;
                if self.position >= DIAL_COUNT {
                    self.position = 0;
                }
                left -= 1;
                if self.position == 0 {
                    self.ticks += 1;
                }
            }
        }
    }
}

impl Challenge for Day01 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut dial = DIAL_START;
        let mut result = 0;

        for line in input.lines() {
            let mut chars = line.chars();

            let direction = chars.next().unwrap();
            let amount: i32 = chars.collect::<String>().parse().unwrap();

            match direction {
                'L' => dial -= amount,
                'R' => dial += amount,
                _ => panic!("Invalid direction"),
            }

            if dial < 0 {
                let count = -dial / DIAL_COUNT;
                dial += count * DIAL_COUNT;
            } else if dial >= DIAL_COUNT {
                let count = dial / DIAL_COUNT;
                dial -= count * DIAL_COUNT;
            }

            if dial == 0 {
                result += 1;
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut dial = Dial::default();

        for line in input.lines() {
            let mut chars = line.chars();

            let direction = chars.next().unwrap();
            let amount: i32 = chars.collect::<String>().parse().unwrap();

            match direction {
                'L' => dial.turn_left(amount),
                'R' => dial.turn_right(amount),
                _ => panic!("Invalid direction"),
            }
        }

        Ok(dial.ticks.to_string())
    }
}
