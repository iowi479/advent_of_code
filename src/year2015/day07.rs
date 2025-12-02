use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::{HashMap, VecDeque};

pub struct Day07 {}

impl Day07 {
    pub fn new() -> Self {
        Self {}
    }
}

enum Instruction<'a> {
    Set(&'a str),
    SetVal(u16),
    Or(&'a str, &'a str),
    And(&'a str, &'a str),
    AndVal(u16, &'a str),
    LShift(&'a str, u16),
    RShift(&'a str, u16),
    Not(&'a str),
}

struct Command<'a> {
    instruction: Instruction<'a>,
    wire: &'a str,
}

fn parse_input(input: &str) -> VecDeque<Command<'_>> {
    input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(ins, to)| {
            if let Some((a, b)) = ins.split_once(" AND ") {
                if let Ok(a) = a.parse() {
                    return Command {
                        instruction: Instruction::AndVal(a, b),
                        wire: to,
                    };
                } else {
                    return Command {
                        instruction: Instruction::And(a, b),
                        wire: to,
                    };
                }
            } else if let Some((a, b)) = ins.split_once(" OR ") {
                return Command {
                    instruction: Instruction::Or(a, b),
                    wire: to,
                };
            } else if let Some((a, b)) = ins.split_once(" LSHIFT ") {
                return Command {
                    instruction: Instruction::LShift(a, b.parse().unwrap()),
                    wire: to,
                };
            } else if let Some((a, b)) = ins.split_once(" RSHIFT ") {
                return Command {
                    instruction: Instruction::RShift(a, b.parse().unwrap()),
                    wire: to,
                };
            } else if let Some(a) = ins.strip_prefix("NOT ") {
                return Command {
                    instruction: Instruction::Not(a),
                    wire: to,
                };
            } else {
                if let Ok(a) = ins.parse() {
                    return Command {
                        instruction: Instruction::SetVal(a),
                        wire: to,
                    };
                }
                return Command {
                    instruction: Instruction::Set(ins),
                    wire: to,
                };
            }
        })
        .collect()
}

fn emulate<'a>(commands: &mut VecDeque<Command<'a>>, wires: &mut HashMap<&'a str, u16>) {
    while let Some(c) = commands.pop_front() {
        match c.instruction {
            Instruction::Set(a) => {
                let a = wires.get(a);

                if a.is_none() {
                    commands.push_back(c);
                    continue;
                }

                wires.insert(c.wire, *a.unwrap());
            }
            Instruction::SetVal(a) => {
                wires.insert(c.wire, a);
            }
            Instruction::Or(a, b) => {
                let a = wires.get(a);
                let b = wires.get(b);

                if a.is_none() || b.is_none() {
                    commands.push_back(c);
                    continue;
                }

                wires.insert(c.wire, a.unwrap() | b.unwrap());
            }

            Instruction::And(a, b) => {
                let a = wires.get(a);
                let b = wires.get(b);

                if a.is_none() || b.is_none() {
                    commands.push_back(c);
                    continue;
                }

                wires.insert(c.wire, a.unwrap() & b.unwrap());
            }

            Instruction::AndVal(a, b) => {
                let b = wires.get(b);

                if b.is_none() {
                    commands.push_back(c);
                    continue;
                }

                wires.insert(c.wire, a & b.unwrap());
            }

            Instruction::LShift(a, b) => {
                let a = wires.get(a);

                if a.is_none() {
                    commands.push_back(c);
                    continue;
                }

                wires.insert(c.wire, a.unwrap() << b);
            }

            Instruction::RShift(a, b) => {
                let a = wires.get(a);

                if a.is_none() {
                    commands.push_back(c);
                    continue;
                }

                wires.insert(c.wire, a.unwrap() >> b);
            }

            Instruction::Not(a) => {
                let a = wires.get(a);

                if a.is_none() {
                    commands.push_back(c);
                    continue;
                }

                wires.insert(c.wire, !a.unwrap());
            }
        }
    }
}

impl Challenge for Day07 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut commands = parse_input(input);

        let mut wires = HashMap::new();

        emulate(&mut commands, &mut wires);

        Ok(wires.get("a").unwrap_or(&0).to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let mut commands = parse_input(input);
        commands = commands.into_iter().filter(|c| c.wire != "b").collect();

        let mut wires = HashMap::new();
        let result = self.part1(input)?;
        wires.insert("b", result.parse().unwrap());

        emulate(&mut commands, &mut wires);
        Ok(wires.get("a").unwrap_or(&0).to_string())
    }
}
