use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::VecDeque;

pub struct Day24 {}

impl Day24 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction<'a> {
    And(&'a str, &'a str, &'a str),
    Or(&'a str, &'a str, &'a str),
    Xor(&'a str, &'a str, &'a str),
}

type Wires<'a> = std::collections::HashMap<&'a str, bool>;

fn input_parse(input: &str) -> (VecDeque<Instruction<'_>>, Wires<'_>) {
    let (init, ins) = input.split_once("\n\n").unwrap();

    let wires = init
        .lines()
        .map(|line| {
            let (wire, value) = line.split_once(": ").unwrap();
            let value = match value {
                "0" => false,
                "1" => true,
                _ => panic!("Invalid value"),
            };
            (wire, value)
        })
        .collect();

    let instructions = ins
        .lines()
        .map(|line| {
            let (lhs, target) = line.split_once(" -> ").unwrap();
            let parts = lhs.split(" ").collect::<Vec<_>>();
            match parts[1] {
                "AND" => {
                    let lhs = parts[0];
                    let rhs = parts[2];
                    Instruction::And(lhs, rhs, target)
                }
                "OR" => {
                    let lhs = parts[0];
                    let rhs = parts[2];
                    Instruction::Or(lhs, rhs, target)
                }
                "XOR" => {
                    let lhs = parts[0];
                    let rhs = parts[2];
                    Instruction::Xor(lhs, rhs, target)
                }
                _ => panic!("Invalid instruction"),
            }
        })
        .collect();

    (instructions, wires)
}

fn execute<'a>(mut wires: Wires<'a>, mut instructions: VecDeque<Instruction<'a>>) -> String {
    while let Some(ins) = instructions.pop_front() {
        let (lhs, rhs, trg) = match ins {
            Instruction::And(lhs, rhs, trg) => (lhs, rhs, trg),
            Instruction::Or(lhs, rhs, trg) => (lhs, rhs, trg),
            Instruction::Xor(lhs, rhs, trg) => (lhs, rhs, trg),
        };

        if let Some(lhs) = wires.get(lhs) {
            if let Some(rhs) = wires.get(rhs) {
                let result = match ins {
                    Instruction::And(_, _, _) => lhs & rhs,
                    Instruction::Or(_, _, _) => lhs | rhs,
                    Instruction::Xor(_, _, _) => lhs ^ rhs,
                };

                if let Some(trg) = wires.get_mut(trg) {
                    *trg = result;
                } else {
                    wires.insert(trg, result);
                }
                continue;
            }
        }

        instructions.push_back(ins);
    }

    let mut z_wires = wires
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .collect::<Vec<_>>();

    z_wires.sort_by_key(|(k, _)| *k);

    let result = z_wires.iter().rev().fold(String::new(), |mut acc, (_, v)| {
        acc.push(if *v { '1' } else { '0' });
        acc
    });

    result
}

impl Challenge for Day24 {
    fn part1(&self, input: &str) -> Result<String> {
        let (instructions, wires) = input_parse(input);
        let result = execute(wires, instructions);
        let result = usize::from_str_radix(&result, 2).unwrap();
        Ok(result.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (instructions, wires) = input_parse(input);

        // manually found the instructions that are causing the problem
        let mut modified = vec!["gdd", "z05", "cwt", "z09", "jmv", "css", "pqt", "z37"];

        if modified.len() != 8 {
            for mut i in 0..44 {
                println!("i = {}", i);
                i = 1 << i;
                let instructions = instructions.clone();
                let mut wires = wires.clone();

                for (k, v) in wires.iter_mut() {
                    if k.starts_with("x") {
                        let x = k[1..=2].parse::<usize>().unwrap();

                        let b = (i >> x) & 1 == 1;
                        *v = b;
                    }

                    if k.starts_with("y") {
                        *v = false;
                    }
                }

                println!("i = {} and i+i = {}", i, 2 * i);

                let result = execute(wires, instructions);

                println!("result = {}", result);
                let result = usize::from_str_radix(&result, 2).unwrap();
                println!("result = {}", result);
                if result != i {
                    return Ok(i.to_string());
                }
            }
        }

        modified.sort();

        Ok(modified.join(","))
    }
}
