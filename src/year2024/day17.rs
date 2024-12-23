use crate::challenge::Challenge;
use anyhow::Result;
use std::collections::HashSet;

pub struct Day17 {}

impl Day17 {
    pub fn new() -> Self {
        Self {}
    }
}

struct Computer {
    registers: [i64; 3],
    program: Vec<Instruction>,
    output: Vec<i64>,
    expected: Vec<i64>,
}
enum Instruction {
    Adv(Operand),
    Bxl(i64),
    Bst(Operand),
    Jnz(i64),
    Bxc(Operand),
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Adv(op) => write!(f, "adv {}: A = A >> {}", op, op),
            Instruction::Bxl(n) => write!(f, "bxl {}: B = B ^ {}", n, n),
            Instruction::Bst(op) => write!(f, "bst {}: B = {} % 8", op, op),
            Instruction::Jnz(n) => write!(f, "jnz {}: jump {} if A not 0", n, n),
            Instruction::Bxc(_) => write!(f, "bxc: B = B ^ C"),
            Instruction::Out(op) => write!(f, "out {}: print {}", op, op),
            Instruction::Bdv(op) => write!(f, "bdv {}: B = A >> {}", op, op),
            Instruction::Cdv(op) => write!(f, "cdv {}: C = A >> {}", op, op),
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(r) => write!(f, "r[{}]", r),
            Operand::Value(v) => write!(f, "{}", v),
        }
    }
}

enum Operand {
    Register(usize),
    Value(i64),
}

impl Operand {
    fn parse(o: i64) -> Self {
        match o {
            0 | 1 | 2 | 3 => Operand::Value(o),
            4 | 5 | 6 => Operand::Register(o as usize - 4),
            _ => panic!("Invalid operand"),
        }
    }
}

fn parse_input(input: &str) -> Computer {
    let (regs, prog) = input.split_once("\n\n").unwrap();
    let registers = regs
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap())
        .collect::<Vec<i64>>();

    let prog_parts = prog[9..]
        .trim()
        .split(',')
        .filter(|n| !n.is_empty())
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut program = Vec::new();
    let mut expected = Vec::new();
    for i in 0..prog_parts.len() {
        expected.push(prog_parts[i]);
        if i % 2 == 1 {
            continue;
        }

        match prog_parts[i] {
            0 => program.push(Instruction::Adv(Operand::parse(prog_parts[i + 1]))),
            1 => program.push(Instruction::Bxl(prog_parts[i + 1])),
            2 => program.push(Instruction::Bst(Operand::parse(prog_parts[i + 1]))),
            3 => program.push(Instruction::Jnz(prog_parts[i + 1])),
            4 => program.push(Instruction::Bxc(Operand::parse(prog_parts[i + 1]))),
            5 => program.push(Instruction::Out(Operand::parse(prog_parts[i + 1]))),
            6 => program.push(Instruction::Bdv(Operand::parse(prog_parts[i + 1]))),
            7 => program.push(Instruction::Cdv(Operand::parse(prog_parts[i + 1]))),
            _ => {}
        }
    }

    Computer {
        registers: registers.try_into().unwrap(),
        program,
        output: Vec::new(),
        expected,
    }
}

impl std::fmt::Debug for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Computer {{ registers: {:?}, output: {:?} }}",
            self.registers, self.output
        )
    }
}
impl Computer {
    fn div(&self, op: &Operand) -> i64 {
        // division
        let val = self.registers[0];
        let den = 2i64.pow(match op {
            Operand::Register(r) => self.registers[*r] as u32,
            Operand::Value(v) => *v as u32,
        });

        val / den
    }

    fn execute(&mut self) {
        let mut pc = 0;
        while pc < self.program.len() {
            match &self.program[pc] {
                Instruction::Adv(n) => {
                    self.registers[0] = self.div(n);
                }
                Instruction::Bxl(n) => {
                    self.registers[1] ^= n;
                }
                Instruction::Bst(n) => {
                    let val = match n {
                        Operand::Register(r) => self.registers[*r],
                        Operand::Value(v) => *v,
                    };
                    self.registers[1] = val % 8;
                }
                Instruction::Jnz(n) => {
                    if self.registers[0] != 0 {
                        pc = *n as usize;
                        continue;
                    }
                }
                Instruction::Bxc(_n) => {
                    self.registers[1] ^= self.registers[2];
                }
                Instruction::Out(n) => {
                    // output
                    let val = match n {
                        Operand::Register(r) => self.registers[*r],
                        Operand::Value(v) => *v,
                    };
                    self.output.push(val % 8);
                }
                Instruction::Bdv(n) => {
                    self.registers[1] = self.div(n);
                }
                Instruction::Cdv(n) => {
                    self.registers[2] = self.div(n);
                }
            }
            pc += 1;
        }
    }

    // This only works for the specified input
    fn find_self_prog(&mut self) -> i64 {
        // each loop A gets shifted by 3 to the right (/8)
        // 3 bits correspont to one printed value and are then discarded
        //
        // B = (A % 8) ^ 1
        // C = A >> B
        // print ((B ^ 5) ^ C) % 8
        //
        // 2 = 010 = 101 = B ^ C
        //
        // A = A >> 3
        //

        let mut solutions = HashSet::new();

        let mut to_check: Vec<i64> = Vec::new();
        for i in 0..=8 {
            to_check.push(8 - i);
        }

        'outer: while let Some(a) = to_check.pop() {
            self.output.clear();
            self.registers[0] = a;
            self.execute();

            if self.output == self.expected {
                solutions.insert(a);
                continue;
            }

            if self.output.len() == 0 || self.output.len() > self.expected.len() {
                continue;
            }

            for i in 0..self.output.len() {
                if self.output[self.output.len() - 1 - i]
                    != self.expected[self.expected.len() - 1 - i]
                {
                    continue 'outer;
                }
            }

            for i in 0..=8 {
                if a == 0 && i == 8 {
                    continue;
                }
                to_check.push((8 - i) | a << 3);
            }
        }

        solutions.iter().min().unwrap().to_owned()
    }
}

impl Challenge for Day17 {
    fn part1(&self, input: &str) -> Result<String> {
        let mut computer = parse_input(input);
        computer.execute();
        Ok(computer
            .output
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(","))
    }

    fn part2(&self, _input: &str) -> Result<String> {
        let mut computer = parse_input(_input);
        let result = computer.find_self_prog();
        Ok(result.to_string())
    }
}
