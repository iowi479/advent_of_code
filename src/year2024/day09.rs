use crate::challenge::Challenge;
use anyhow::Result;
use std::fmt::{self, Display, Formatter};

pub struct Day09 {}

impl Day09 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Challenge for Day09 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut remaining = input
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        let mut result: usize = 0;

        let mut mult = 0;
        let mut front = 0;
        let mut back = remaining.len() - 1;
        loop {
            if front > back {
                break;
            }

            if remaining[front] == 0 {
                front += 1;
                continue;
            }
            if remaining[back] == 0 {
                back -= 1;
                continue;
            }

            if front % 2 == 0 {
                let m = mult * (front / 2);
                result += m;
                mult += 1;
                remaining[front] -= 1;
            } else {
                if back % 2 == 1 {
                    back -= 1;
                }
                let m = mult * (back / 2);
                result += m;
                mult += 1;
                remaining[back] -= 1;
                remaining[front] -= 1;
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut input = parse_input(input);
        let mut result: usize = 0;

        let mut i = input.len() - 1;
        'outer: loop {
            if i == 0 {
                break;
            }
            let last = input[i];
            if last.content == Content::None {
                i -= 1;
                continue;
            }

            for j in 0..i {
                if let Content::File(_) = input[j].content {
                    continue;
                }
                if input[j].length == last.length {
                    input[j].content = last.content;
                    input[i].content = Content::None;
                    i -= 1;
                    continue 'outer;
                } else if input[j].length > last.length {
                    input[j].length -= last.length;
                    input[i].content = Content::None;
                    input.insert(j, last);
                    continue 'outer;
                }
            }

            i -= 1;
        }

        let mut mult = 0;
        for i in 0..input.len() {
            if input[i].content == Content::None {
                mult += input[i].length;
                continue;
            }

            for _ in 0..input[i].length {
                if let Content::File(m) = input[i].content {
                    result += mult * m;
                    mult += 1;
                }
            }
        }
        Ok(result.to_string())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Content {
    File(usize),
    None,
}

#[derive(Debug, Clone, Copy)]
struct Block {
    length: usize,
    content: Content,
}
impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}x{:?}", self.length, self.content)
    }
}

fn parse_input(input: &str) -> Vec<Block> {
    input
        .chars()
        .filter(|c| c.is_digit(10))
        .enumerate()
        .map(|(i, c)| {
            let content = if i % 2 == 0 {
                Content::File(i / 2)
            } else {
                Content::None
            };

            Block {
                length: c.to_digit(10).unwrap() as usize,
                content,
            }
        })
        .collect()
}
